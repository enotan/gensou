mod backend;
mod state;

use state::GensouState;
use backend::winit::init_winit;

use std::error::Error;
use tracing::info;
use tracing_subscriber::EnvFilter;
use smithay::reexports::{
    calloop::EventLoop,
    wayland_server::Display,
};

fn main() -> Result<(), Box<dyn Error>> {
    if let Ok(env_filter) = EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }

    info!("starting gensou");

    let mut event_loop: EventLoop<GensouState> = EventLoop::try_new()?;

    let _display: Display<GensouState> = Display::new()?;
    let mut state = GensouState::new(&event_loop);

    init_winit(&mut event_loop, &mut state)?;

    info!("event loop initialised");
    info!("state initialised");

    event_loop.run(None, &mut state, |_| {})?;

    Ok(())
}