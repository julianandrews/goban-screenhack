extern crate gl;
extern crate glutin;
extern crate nanovg;

mod goban;
mod goban_display;
mod ui;
mod xscreensaver_context;

use std::env;

fn get_window_id() -> Option<u64> {
    env::var("XSCREENSAVER_WINDOW")
        .ok()
        .map(|window_id_string| {
            if window_id_string.starts_with("0x") {
                u64::from_str_radix(window_id_string.trim_start_matches("0x"), 16).unwrap()
            } else {
                u64::from_str_radix(&window_id_string, 10).unwrap()
            }
        })
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_id = get_window_id();
    let xs = xscreensaver_context::XScreensaverContext::new(&event_loop, window_id).unwrap(); // TODO

    unsafe {
        gl::load_with(|symbol| xs.context().get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
    let nanovg_context = nanovg::ContextBuilder::new()
        .build()
        .expect("Initialization of NanoVG failed!"); // TODO

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::LoopDestroyed => return,
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::Resized(physical_size) => {
                    xs.context().resize(physical_size)
                }
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit
                }
                _ => (),
            },
            _ => (),
        }

        let physical_size = xs.inner_size().unwrap(); // TODO
        let (width, height) = (physical_size.width, physical_size.height);
        let scale_factor = xs.scale_factor().unwrap(); // TODO

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let (width, height) = (width as f32, height as f32);
        let ui = ui::UI::new();

        nanovg_context.frame((width, height), scale_factor as f32, |mut frame| {
            ui.draw(&mut frame, width, height);
        });
        xs.context().swap_buffers().unwrap();
    });
}
