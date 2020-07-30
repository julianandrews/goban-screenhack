use std::sync::Arc;

use glutin::platform::unix::x11::XConnection;
use glutin::platform::unix::RawContextExt;

enum WindowWrapper {
    GlutinWindow {
        window: glutin::window::Window,
    },
    RawWindow {
        xconn: Arc<XConnection>,
        window_id: u64,
    },
}

pub struct XScreensaverContext {
    context: glutin::RawContext<glutin::PossiblyCurrent>,
    window: WindowWrapper,
}

impl XScreensaverContext {
    pub fn new(
        event_loop: &glutin::event_loop::EventLoop<()>,
        window_id: Option<u64>,
    ) -> Result<XScreensaverContext, Box<dyn std::error::Error>> {
        let context_builder = glutin::ContextBuilder::new()
            .with_multisampling(4)
            .with_vsync(true);
        let (context, window) = match window_id {
            Some(window_id) => {
                let xconn = Arc::new(XConnection::new(None)?);
                let context =
                    unsafe { context_builder.build_raw_x11_context(xconn.clone(), window_id)? };
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

        let context = unsafe { context.make_current().map_err(|(_, e)| e)? };

        Ok(XScreensaverContext {
            context: context,
            window: window,
        })
    }

    pub fn context(&self) -> &glutin::RawContext<glutin::PossiblyCurrent> {
        &self.context
    }

    pub fn inner_size(&self) -> Result<glutin::dpi::PhysicalSize<u32>, Box<dyn std::error::Error>> {
        match &self.window {
            WindowWrapper::GlutinWindow { window } => Ok(window.inner_size()),
            WindowWrapper::RawWindow { xconn, window_id } => {
                let geometry = xconn.get_geometry(*window_id)?;
                Ok(glutin::dpi::PhysicalSize {
                    width: geometry.width,
                    height: geometry.height,
                })
            }
        }
    }

    pub fn scale_factor(&self) -> Result<f64, Box<dyn std::error::Error>> {
        match &self.window {
            WindowWrapper::GlutinWindow { window } => Ok(window.scale_factor()),
            WindowWrapper::RawWindow { xconn, window_id } => {
                let geometry = xconn.get_geometry(*window_id)?;
                let coords = xconn.translate_coords(*window_id, geometry.root)?;
                let rect = glutin::platform::unix::x11::util::AaRect::new(
                    (coords.x_rel_root, coords.y_rel_root),
                    (geometry.width, geometry.height),
                );
                let monitor = xconn.get_monitor_for_window(Some(rect));
                Ok(monitor.scale_factor())
            }
        }
    }
}