extern crate gl;
extern crate glutin;
extern crate nanovg;

mod sgf;
mod args;
mod goban;
mod goban_display;
mod ui;
mod xscreensaver_context;

use std::time::Duration;
use rand::thread_rng;
use rand::seq::SliceRandom;

enum GameState {
    New,
    Ongoing,
    Ended,
}

fn load_sgfs(_sgf_dirs: &Vec<std::path::PathBuf>) -> Result<Vec<sgf::SgfNode>, sgf::SgfParseError> {
    // TODO
    sgf::parse("(;SZ[13:9]B[dc];W[ef](;B[aa])(;B[cc];W[ee]))")
}

fn main() {
    // Parse arguments
    let args: Vec<String> = std::env::args().collect();
    let opts = args::build_opts();
    let parsed_args = match args::parse_args(&opts, &args) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{}", error);
            args::print_usage(&args[0], &opts);
            std::process::exit(1);
        }
    };
    if parsed_args.print_help {
        args::print_usage(&args[0], &opts);
        return;
    }

    // Graphics setup
    let event_loop = glutin::event_loop::EventLoop::new();
    let xs = xscreensaver_context::XScreensaverContext::new(&event_loop, parsed_args.window_type)
        .unwrap(); // TODO
    unsafe {
        gl::load_with(|symbol| xs.context().get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
    let nanovg_context = nanovg::ContextBuilder::new()
        .build()
        .expect("Initialization of NanoVG failed!"); // TODO

    // Goban setup
    let sgfs = match load_sgfs(&parsed_args.sgf_dirs) {
        Ok(sgfs) => sgfs,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };
    let mut rng = thread_rng();
    let mut sgf_node = sgfs.choose(&mut rng).unwrap().clone(); // sgfs is never empty
    let mut game_state = GameState::New;
    let mut last_action_time = std::time::Instant::now();
    let mut ui = ui::UI::new();

    // Main Loop
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
        match game_state {
            GameState::New => {
                println!("Initializing with {:?}", sgf_node);
                let board_size = sgf_node.get_size().unwrap_or((19, 19));
                ui.reset(board_size);
                // TODO: Handle other start properties

                game_state = GameState::Ongoing;
            },
            GameState::Ongoing => {
                if last_action_time.elapsed() > Duration::from_millis(parsed_args.move_delay) {
                    println!("Processing Node: {:?}", sgf_node);
                    if let Some(stone) = sgf_node.get_move() {
                        println!("Playing stone {:?}", stone);
                        ui.play_stone(stone).unwrap(); // TODO
                    }
                    // TODO: process other properties
                    if sgf_node.children.is_empty() {
                        // TODO: avoid these stupid clones
                        sgf_node = sgfs.choose(&mut rng).unwrap().clone(); // sgfs is never empty
                        game_state = GameState::Ended;
                    } else {
                        // TODO: avoid these stupid clones
                        sgf_node = sgf_node.clone().children.into_iter().next().unwrap();
                    }
                    last_action_time = std::time::Instant::now();
                }
            },
            GameState::Ended => {
                if last_action_time.elapsed() > Duration::from_millis(parsed_args.end_delay) {
                    println!("Ending Game");
                    game_state = GameState::New;
                    last_action_time = std::time::Instant::now();
                }
            },
        }

        let physical_size = xs.inner_size().unwrap(); // TODO
        let (width, height) = (physical_size.width, physical_size.height);
        let scale_factor = xs.scale_factor().unwrap(); // TODO

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let (width, height) = (width as f32, height as f32);

        nanovg_context.frame((width, height), scale_factor as f32, |mut frame| {
            ui.draw(&mut frame, width, height);
        });
        xs.context().swap_buffers().unwrap();
    });
}
