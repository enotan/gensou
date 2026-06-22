use smithay::{
    backend::{
        renderer::{Color32F, Frame, Renderer, gles::GlesRenderer},
        winit,
    },
    reexports::calloop::EventLoop,
    utils::{Rectangle, Transform},
};
use tracing::{error, info};

use crate::state::GensouState;

///initialise the winit backend for development (compositor in compositor)
pub fn init_winit(
    event_loop: &mut EventLoop<GensouState>,
    _state: &mut GensouState,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut backend, winit) = winit::init::<GlesRenderer>()?;
    
    event_loop.handle().insert_source(winit, move |event, _, state| {
        match event {
            winit::WinitEvent::Redraw => {
                let size = backend.window_size();
                let damage = Rectangle::from_size(size);

                {
                    let Ok((renderer, mut framebuffer)) = backend.bind() else {
                        error!("failed to bind winit framebuffer");
                        state.stop();
                        return;
                    };

                    let Ok(mut frame) = renderer.render(&mut framebuffer, size, Transform::Flipped180) else {
                        error!("failed to start winit render pass");
                        state.stop();
                        return;
                    };

                    if frame
                        .clear(Color32F::new(0.08, 0.08, 0.1, 1.0), &[damage])
                        .and_then(|_| frame.finish())
                        .is_err()
                    {
                        error!("failed to clear winit frame");
                        state.stop();
                        return;
                    }
                }

                if backend.submit(Some(&[damage])).is_err() {
                    error!("failed to submit winit frame");
                    state.stop();
                    return;
                }

                backend.window().request_redraw();
            }
            winit::WinitEvent::CloseRequested => {
                info!("winit window close requested");
                state.stop();
            }
            _ => {}
        }
    })?;
    
    Ok(())
}