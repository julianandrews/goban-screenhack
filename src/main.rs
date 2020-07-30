extern crate gl;
extern crate glutin;
extern crate nanovg;

mod goban;
mod goban_display;
mod xscreensaver_window;

use std::env;
use std::rc::Rc;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_id = env::var("XSCREENSAVER_WINDOW").ok().map(|window_id_string|
        window_id_string.parse::<u64>().unwrap() // TODO
    );
    // For RawContext windows, window must not be destroyed before the context
    let (glutin_context, window) = xscreensaver_window::build_raw_context(&event_loop, window_id);

    unsafe {
        gl::load_with(|symbol| glutin_context.get_proc_address(symbol) as *const _);
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
                    glutin_context.resize(physical_size)
                }
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit
                }
                _ => (),
            },
            _ => (),
        }

        let physical_size = window.inner_size();
        let (width, height) = (physical_size.width, physical_size.height);
        let scale_factor = window.scale_factor();

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let (width, height) = (width as f32, height as f32);
        let goban = Rc::new(goban::Goban::new());
        let goban_display = goban_display::GobanDisplay::new(goban);

        nanovg_context.frame((width, height), scale_factor, |mut frame| {
            goban_display.draw(&mut frame, width, height);
        });
        glutin_context.swap_buffers().unwrap();
    });
}
