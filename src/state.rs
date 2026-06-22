use smithay::reexports::calloop::{EventLoop, LoopSignal};

///the mutable state
#[derive(Debug)]
pub struct GensouState {
    loop_signal: LoopSignal,
}

impl GensouState {
    ///create an empty state
    pub fn new(event_loop: &EventLoop<Self>) -> Self {
        Self {
            loop_signal: event_loop.get_signal(),
        }
    }

    ///request that the compositor's event loop exits
    pub fn stop(&self) {
        self.loop_signal.stop();
    }
}