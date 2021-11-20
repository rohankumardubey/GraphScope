use crate::api::{HasAny, Unary};
use crate::stream::{Single, SingleItem, Stream};
use crate::tag::tools::map::TidyTagMap;
use crate::{BuildJobError, Data};
use crate::communication::output::OutputProxy;

impl<D: Data> HasAny<D> for Stream<D> {
    fn any(mut self) -> Result<SingleItem<bool>, BuildJobError> {
        self.set_upstream_batch_capacity(1);
        let mut stream = self.unary("any", |info| {
            let mut any_map = TidyTagMap::<()>::new(info.scope_level);
            move |input, output| {
                input.for_each_batch(|batch| {
                    if !batch.is_empty() {
                        if !any_map.contains_key(batch.tag()) {
                            any_map.insert(batch.tag().clone(), ());
                            output.new_session(batch.tag())?.give(())?;
                        }
                        batch.discard();
                    } else {
                        // ignore
                    }
                    if batch.is_last() {
                        any_map.remove(batch.tag());
                    }
                    Ok(())
                })
            }
        })?;
        stream
            .set_upstream_batch_capacity(1)
            .set_upstream_batch_size(1);
        let x = stream.aggregate().unary("any_global", |info| {
            let mut any_map = TidyTagMap::<()>::new(info.scope_level);
            move |input, output| {
                input.for_each_batch(|batch| {
                    if !batch.is_empty() {
                        if !any_map.contains_key(batch.tag()) {
                            any_map.insert(batch.tag().clone(), ());
                            output
                                .new_session(batch.tag())?
                                .give(Single(true))?;
                        }
                        batch.clear();

                        if batch.is_last() {
                            any_map.remove(batch.tag());
                        }
                        return Ok(())
                    }

                    if let Some(end) = batch.take_end() {
                        if any_map.remove(batch.tag()).is_none() {
                            let worker = crate::worker_id::get_current_worker().index;
                            if end.peers.contains_source(worker) {
                                output.new_session(batch.tag())?.give(Single(false))?;
                            } else {
                                output.notify_end(end)?;
                            }
                        }
                    }
                    Ok(())
                })
            }
        })?;

        Ok(SingleItem::new(x))
    }
}
