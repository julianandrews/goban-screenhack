use glutin::platform::unix::RawContextExt;

use std::sync::Arc;

pub enum WindowWrapper {
    GlutinWindow {
        window: glutin::window::Window,
    },
    RawWindow {
        xconn: Arc<glutin::platform::unix::x11::XConnection>,
        window_id: u64,
    },
}

impl WindowWrapper {
    pub fn inner_size(&self) -> glutin::dpi::PhysicalSize<u32> {
        match self {
            WindowWrapper::GlutinWindow { window } => window.inner_size(),
            WindowWrapper::RawWindow { xconn, window_id } => {
                let geo = xconn
                    .get_geometry(*window_id as glutin::platform::unix::x11::ffi::Window)
                    .unwrap(); // TODO
                glutin::dpi::PhysicalSize {
                    width: geo.width,
                    height: geo.height,
                }
            }
        }
    }

    pub fn scale_factor(&self) -> f32 {
        1.0 // TODO
    }
}

pub struct XScreensaverContext {
    context: glutin::RawContext<glutin::PossiblyCurrent>,
    window: WindowWrapper,
}

impl XScreensaverContext {
    pub fn new(
        event_loop: &glutin::event_loop::EventLoop<()>,
        window_id: Option<u64>,
    ) -> XScreensaverContext {
        let context_builder = glutin::ContextBuilder::new()
            .with_multisampling(4)
            .with_vsync(true);
        let (context, window) = match window_id {
            Some(window_id) => {
                let xconn = Arc::new(glutin::platform::unix::x11::XConnection::new(None).unwrap()); // TODO
                let context = unsafe {
                    context_builder
                        .build_raw_x11_context(xconn.clone(), window_id)
                        .unwrap() // TODO
                };
                let window = WindowWrapper::RawWindow {
                    xconn: xconn,
                    window_id: window_id,
                };
                (context, window)
            }
            None => {
                let window_builder =
                    glutin::window::WindowBuilder::new().with_title("Goban Screenhack");
                let (context, window) = unsafe {
                    context_builder
                        .build_windowed(window_builder, &event_loop)
                        .unwrap()
                        .split()
                };
                let window = WindowWrapper::GlutinWindow { window };
                (context, window)
            }
        };

        let context = unsafe { context.make_current().unwrap() }; // TODO

        XScreensaverContext {
            context: context,
            window: window,
        }
    }

    pub fn window(&self) -> &WindowWrapper {
        &self.window
    }

    pub fn context(&self) -> &glutin::RawContext<glutin::PossiblyCurrent> {
        &self.context
    }
}
