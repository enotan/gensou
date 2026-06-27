use smithay::{
    reexports::{
        wayland_server::protocol::wl_seat,
        wayland_protocols::xdg::shell::server::xdg_toplevel,
    },
    utils::Serial,
    wayland::shell::xdg::{
        PopupSurface, 
        PositionerState, 
        ToplevelSurface, 
        XdgShellHandler, 
        XdgShellState,
    },
    desktop::Window,
};
use tracing::debug;

use crate::state::GensouState;

impl XdgShellHandler for GensouState {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        surface.with_pending_state(|state| {
            state.states.set(xdg_toplevel::State::Activated);
        });
        surface.send_configure();

        let window = Window::new_wayland_window(surface);
        self.windows.push(window);

        debug!(window_count = self.windows.len(), "new toplevel");
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {
        debug!("new popup");
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {
        debug!("popup grab requested");
    }

    fn reposition_request(&mut self, _surface: PopupSurface, _positioner: PositionerState, _token: u32) {
        debug!("popup reposition requested");
    }

    fn toplevel_destroyed(&mut self, surface: ToplevelSurface) {
        self.windows.retain(|window| {
            window
                .toplevel()
                .is_none_or(|toplevel| toplevel.wl_surface() != surface.wl_surface())
        });
            

        debug!(window_count = self.windows.len(), "toplevel destroyed");
    }
}