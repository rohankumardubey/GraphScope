use crate::channel_id::ChannelInfo;
use crate::communication::decorator::evented::EventEmitPush;
use crate::data::MicroBatch;
use crate::data_plane::Push;
use crate::errors::IOError;
use crate::progress::DynPeers;
use crate::Data;

pub struct AggregateBatchPush<D: Data> {
    pub ch_info: ChannelInfo,
    src: u32,
    target: u32,
    data_push: EventEmitPush<D>,
    push_count: usize,
    event_push: Option<EventEmitPush<D>>,
}

impl<D: Data> AggregateBatchPush<D> {
    pub fn new(target: u32, info: ChannelInfo, pushes: Vec<EventEmitPush<D>>) -> Self {
        assert_eq!(info.scope_level, 0);
        let source = crate::worker_id::get_current_worker().index;
        let mut vec = vec![];
        for mut p in pushes {
            if p.target_worker == source || p.target_worker == target {
                vec.push(Some(p));
            } else {
                p.close().ok();
                vec.push(None);
            }
        }
        let data_push = vec[target as usize]
            .take()
            .expect("data push lost");
        let event_push = vec[source as usize].take();
        AggregateBatchPush { ch_info: info, src: source, target, data_push, push_count: 0, event_push }
    }
}

impl<D: Data> Push<MicroBatch<D>> for AggregateBatchPush<D> {
    fn push(&mut self, mut batch: MicroBatch<D>) -> Result<(), IOError> {
        self.push_count += batch.len();
        let end = batch.take_end();
        if !batch.is_empty() {
            self.data_push.push(batch)?;
        }

        if let Some(mut end) = end {
            end.total_send = self.push_count as u64;
            if let Some(ref mut event) = self.event_push {
                let mut new_end = end.clone();
                new_end.total_send = 0;
                new_end.peers = DynPeers::single(self.src);
                event.push_end(new_end, DynPeers::single(self.target))?;
            }
            self.data_push.sync_end(end, DynPeers::single(self.target))
        } else {
            Ok(())
        }
    }

    fn flush(&mut self) -> Result<(), IOError> {
        self.data_push.flush()?;
        if let Some(ref mut p) = self.event_push {
            p.flush()?;
        }
        Ok(())
    }

    fn close(&mut self) -> Result<(), IOError> {
        self.data_push.close()?;
        if let Some(ref mut p) = self.event_push {
            p.close()?;
        }
        Ok(())
    }
}
