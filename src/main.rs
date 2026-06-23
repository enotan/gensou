mod backend;
mod handlers;
mod state;

use backend::winit::init_winit;
use state::GensouState;

use smithay::reexports::{calloop::EventLoop, wayland_server::Display};
use std::error::Error;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Box<dyn Error>> {
    if let Ok(env_filter) = EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }

    info!("starting gensou");

    let mut event_loop: EventLoop<GensouState> = EventLoop::try_new()?;

    let display: Display<GensouState> = Display::new()?;
    let mut state = GensouState::new(&mut event_loop, display);

    init_winit(&mut event_loop, &mut state)?;

    unsafe {
        std::env::set_var("WAYLAND_DISPLAY", &state.socket_name);
    }

    info!(socket = ?state.socket_name, "Wayland socket ready");

    spawn_client();

    info!("entering event loop");
    
    event_loop.run(None, &mut state, |_| {})?;

    Ok(())
}

fn spawn_client() {
    let mut args = std::env::args().skip(1);
    let flag = args.next();
    let command = args.next();

    if let (Some("-c" | "--command"), Some(command)) = (flag.as_deref(), command) 
        && let Err(error) = std::process::Command::new(command).spawn()
    {
        tracing::warn!(?error, "failed to spawn client");
    }
}
