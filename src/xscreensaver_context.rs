use std::sync::Arc;

use glutin::platform::unix::x11::XConnection;
use glutin::platform::unix::RawContextExt;

#[derive(Debug, Copy, Clone)]
pub enum WindowType {
    New,
    Root,
    WindowId(u64),
}

#[derive(Debug)]
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
        window_type: WindowType,
    ) -> Result<XScreensaverContext, Box<dyn std::error::Error>> {
        let context_builder = glutin::ContextBuilder::new()
            .with_multisampling(4)
            .with_vsync(true);
        let (context, window) = match window_type {
            WindowType::New => {
                let window_builder =
                    glutin::window::WindowBuilder::new().with_title("Goban Screenhack");
                let (context, window) = unsafe {
                    context_builder
                        .build_windowed(window_builder, &event_loop)?
                        .split()
                };
                let window = WindowWrapper::GlutinWindow { window };
                (context, window)
            }
            _ => {
                let xconn = Arc::new(XConnection::new(None)?);
                let window_id = match window_type {
                    WindowType::WindowId(window_id) => window_id,
                    WindowType::Root => unsafe { (xconn.xlib.XRootWindow)(xconn.display, 0) },
                    _ => panic!("Should never happen"),
                };
                let context =
                    unsafe { context_builder.build_raw_x11_context(xconn.clone(), window_id)? };
                let window = WindowWrapper::RawWindow { xconn, window_id };
                (context, window)
            }
        };

        let context = unsafe { context.make_current().map_err(|(_, e)| e)? };

        Ok(XScreensaverContext { context, window })
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
