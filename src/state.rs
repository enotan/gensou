use smithay::{
    reexports::{
        calloop::{EventLoop, Interest, LoopSignal, Mode, PostAction, generic::Generic},
        wayland_server::{
            Display,
            backend::{ClientData, ClientId, DisconnectReason},
        },
    },
    wayland::{
        compositor::{CompositorClientState, CompositorState},
        socket::ListeningSocketSource,
    },
};
use std::{ffi::OsString, sync::Arc};
use tracing::{debug, info};

///the mutable state
#[derive(Debug)]
pub struct GensouState {
    loop_signal: LoopSignal,
    pub compositor_state: CompositorState,
    pub socket_name: OsString,
}

impl GensouState {
    ///create an empty state
    pub fn new(event_loop: &mut EventLoop<Self>, display: Display<Self>) -> Self {
        let display_handle = display.handle();
        let compositor_state = CompositorState::new::<Self>(&display_handle);
        let socket_name = Self::init_wayland_listener(display, event_loop);

        Self {
            compositor_state,
            loop_signal: event_loop.get_signal(),
            socket_name,
        }
    }

    ///request that the compositor's event loop exits
    pub fn stop(&self) {
        self.loop_signal.stop();
    }

    fn init_wayland_listener(display: Display<Self>, event_loop: &mut EventLoop<Self>) -> OsString {
        let listening_socket =
            ListeningSocketSource::new_auto().expect("failed to create Wayland listening socket");

        let socket_name = listening_socket.socket_name().to_os_string();
        let mut display_handle = display.handle();

        event_loop
            .handle()
            .insert_source(listening_socket, move |client_stream, _, _state| {
                display_handle
                    .insert_client(client_stream, Arc::new(ClientState::default()))
                    .expect("failed to insert Wayland client");
            })
            .expect("failed to insert Wayland socket source");

        event_loop
            .handle()
            .insert_source(
                Generic::new(display, Interest::READ, Mode::Level),
                |_, display, state| {
                    unsafe {
                        display.get_mut().dispatch_clients(state).unwrap();
                    }

                    Ok(PostAction::Continue)
                },
            )
            .expect("failed to insert Wayland display source");

        socket_name
    }
}

///data smithay associates with each connected wayland client
#[derive(Default)]
pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(&self, client_id: ClientId) {
        debug!(?client_id, "Wayland client initialised");
    }

    fn disconnected(&self, client_id: ClientId, reason: DisconnectReason) {
        info!(?client_id, ?reason, "Wayland client disconnected");
    }
}
