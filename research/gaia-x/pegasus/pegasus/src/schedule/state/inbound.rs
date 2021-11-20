use crate::communication::IOResult;
use crate::data::{EndOfScope, MicroBatch};
use crate::data_plane::{GeneralPush, Push};
use crate::graph::Port;
use crate::progress::{EndSyncSignal, DynPeers};
use crate::tag::tools::map::TidyTagMap;
use crate::{Data, Tag};

#[allow(dead_code)]
struct ScopeEndPanel {
    tag: Tag,
    source: DynPeers,
    children: DynPeers,
    merged: DynPeers,
    count: u64,
    is_exhaust: bool,
}

impl ScopeEndPanel {
    fn new(src: u32, end: EndSyncSignal) -> Self {
        let mut merged = DynPeers::empty();
        merged.add_source(src);
        let (end, children) = end.take();
        ScopeEndPanel {
            tag: end.tag,
            source: end.peers,
            children,
            merged,
            count: end.total_send,
            is_exhaust: false,
        }
    }

    fn merge(&mut self, src: u32, end: EndSyncSignal) -> Option<EndOfScope> {
        let (end, children) = end.take();
        assert_eq!(end.tag, self.tag);
        assert_eq!(end.peers, self.source);
        self.merged.add_source(src);
        self.children.merge(children);
        self.count += end.total_send;

        if self.merged == self.source {
            self.is_exhaust = true;
            let mut src = std::mem::replace(&mut self.children, DynPeers::empty());
            if src.value() == 0 {
                assert_eq!(self.count, 0);
                // empty scope;
                let mut owner = 0;
                let worker_id = crate::worker_id::get_current_worker();
                if self.tag.len() > 0 {
                    owner = self.tag.current_uncheck() % worker_id.total_peers();
                }
                src = DynPeers::single(owner);
                Some(EndOfScope::new(self.tag.clone(), src, self.count))
            } else {
                Some(EndOfScope::new(self.tag.clone(), src, self.count))
            }
        } else {
            None
        }
    }
}

pub trait InputEndNotify: Send + 'static {
    fn notify(&mut self, end: EndOfScope) -> IOResult<()>;

    fn close_notify(&mut self);
}

impl<T: Data> InputEndNotify for GeneralPush<MicroBatch<T>> {
    fn notify(&mut self, end: EndOfScope) -> IOResult<()> {
        let last = MicroBatch::last(0, end);
        if last.tag().is_root() {
            self.push(last)?;
            self.close()?;
        } else {
            self.push(last)?;
        }
        Ok(())
    }

    fn close_notify(&mut self) {
        self.close().ok();
    }
}

pub struct InboundStreamState {
    port: Port,
    scope_level: u32,
    notify_guards: Vec<TidyTagMap<ScopeEndPanel>>,
    notify: Box<dyn InputEndNotify>,
}

impl InboundStreamState {
    pub fn new(port: Port, scope_level: u32, notify: Box<dyn InputEndNotify>) -> Self {
        let mut notify_guards = Vec::new();
        for i in 0..scope_level + 1 {
            notify_guards.push(TidyTagMap::new(i));
        }
        InboundStreamState { port, scope_level, notify_guards, notify }
    }

    pub fn on_end(&mut self, src: u32, end: EndSyncSignal) -> IOResult<()> {
        if end.sources() == 1 {
            let (mut end, child) = end.take();
            trace_worker!("input[{:?}] get end of {:?}, total pushed {}, peers: {:?}=>{:?}", self.port, end.tag, end.total_send, end.peers, child);
            end.peers = child;
            return self.notify.notify(end);
        }

        let idx = end.tag().len();
        assert!(idx <= self.scope_level as usize);
        let tag = end.tag().clone();
        // if idx < self.scope_level as usize {
        //     // this is an end of parent scope;
        //     let mut notify_guards = std::mem::replace(&mut self.notify_guards, vec![]);
        //     for i in idx + 1..self.scope_level as usize + 1 {
        //         for (t, p) in notify_guards[i].iter_mut() {
        //             if tag.is_parent_of(&*t) {
        //                 if let Some(end) = p.add_end_source(src) {
        //                     self.notify.notify(end)?;
        //                 }
        //             }
        //         }
        //     }
        //     // TODO: clean notify guards; (only retain not exhaust;)
        //     self.notify_guards = notify_guards;
        // }

        if let Some(mut p) = self.notify_guards[idx].remove(end.tag()) {
            if let Some(e) = p.merge(src, end) {
                trace_worker!("input[{:?}] get end of {:?}, total pushed {}, peers: {:?}", self.port, e.tag, e.total_send, e.peers);
                self.notify.notify(e)?;
            } else {
                trace_worker!(
                    "input[{:?}] partial end of {:?}, expect {:?}, current {:?};",
                    self.port,
                    p.tag,
                    p.source,
                    p.merged
                );
                self.notify_guards[idx].insert(tag, p);
            }
        } else {
            let p = ScopeEndPanel::new(src, end);
            self.notify_guards[idx].insert(tag, p);
        }
        Ok(())
    }
}

impl Drop for InboundStreamState {
    fn drop(&mut self) {
        self.notify.close_notify();
    }
}
