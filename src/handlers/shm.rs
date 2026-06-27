use smithay::{
    reexports::wayland_server::protocol::wl_buffer,
    wayland::{
        shm::{ShmHandler, ShmState},
        buffer::BufferHandler,
    }
};

use crate::state::GensouState;

impl ShmHandler for GensouState {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

impl BufferHandler for GensouState {
    fn buffer_destroyed(&mut self, _buffer: &wl_buffer::WlBuffer) {
        
    }
}