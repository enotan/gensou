use smithay::{
    backend::renderer::utils::on_commit_buffer_handler,
    delegate_dispatch2,
    reexports::wayland_server::{Client, protocol::wl_surface::WlSurface},
    wayland::compositor::{CompositorClientState, CompositorHandler, CompositorState},
};
use tracing::debug;

use crate::state::{ClientState, GensouState};

impl CompositorHandler for GensouState {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client
            .get_data::<ClientState>()
            .expect("client data should be ClientState")
            .compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler::<Self>(surface);
        debug!("surface committed");
    }
}

delegate_dispatch2!(GensouState);
