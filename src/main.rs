extern crate gl;
extern crate glutin;
extern crate nanovg;
extern crate sgf_parse;

mod args;
mod goban;
mod goban_display;
mod ui;
mod xscreensaver_context;

use rand::seq::SliceRandom;
use rand::thread_rng;
use sgf_parse::SgfProp;
use std::time::Duration;

enum GameState {
    New,
    Ongoing,
    Ended,
}

fn load_sgfs(
    sgf_dirs: &Vec<std::path::PathBuf>,
) -> Result<Vec<sgf_parse::SgfNode>, Box<dyn std::error::Error>> {
    let mut sgfs = vec![];
    for dir in sgf_dirs.iter() {
        for entry in std::fs::read_dir(&dir)? {
            let path = entry?.path();
            if path.is_file() {
                match path.extension().and_then(std::ffi::OsStr::to_str) {
                    Some("sgf") => {
                        let contents = std::fs::read_to_string(path.clone())?;
                        match sgf_parse::parse(&contents) {
                            Ok(new_nodes) => sgfs.extend(new_nodes),
                            Err(e) => eprintln!("Error parsing {}: {}", path.to_string_lossy(), e),
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(sgfs)
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
    let xs = match xscreensaver_context::XScreensaverContext::new(
        &event_loop,
        parsed_args.window_type,
    ) {
        Ok(xs) => xs,
        Err(e) => {
            eprintln!("Creationg of XScreensaver Context failed: {}", e);
            std::process::exit(1);
        }
    };
    unsafe {
        gl::load_with(|symbol| xs.context().get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
    let nanovg_context = match nanovg::ContextBuilder::new().build() {
        Ok(nanovg_context) => nanovg_context,
        Err(_) => {
            eprintln!("Initialization of NanoVG failed");
            std::process::exit(1);
        }
    };

    // Goban setup
    let sgfs = match load_sgfs(&parsed_args.sgf_dirs) {
        Ok(sgfs) => sgfs,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };
    if sgfs.is_empty() {
        eprintln!("No valid sgf files found.");
        std::process::exit(1);
    }
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
                let board_size = match sgf_node.get_property("SZ") {
                    Some(SgfProp::SZ(size)) => size.clone(),
                    None => (19, 19),
                    _ => unreachable!(),
                };
                ui.reset(board_size);
                game_state = GameState::Ongoing;
            }
            GameState::Ongoing => {
                if last_action_time.elapsed() > Duration::from_millis(parsed_args.move_delay) {
                    for prop in sgf_node.properties() {
                        match prop {
                            SgfProp::B(sgf_parse::Move::Move(point)) => {
                                if point.x == 19
                                    && point.y == 19
                                    && ui.board_size().0 < 20
                                    && ui.board_size().1 < 20
                                {
                                    continue; // "tt" pass
                                }
                                let stone = goban::Stone {
                                    x: point.x,
                                    y: point.y,
                                    color: goban::StoneColor::Black,
                                };
                                if let Err(error) = ui.play_stone(stone) {
                                    eprintln!("{}", error);
                                    std::process::exit(1);
                                }
                            }
                            SgfProp::W(sgf_parse::Move::Move(point)) => {
                                if point.x == 19
                                    && point.y == 19
                                    && ui.board_size().0 < 20
                                    && ui.board_size().1 < 20
                                {
                                    continue; // "tt" pass
                                }
                                let stone = goban::Stone {
                                    x: point.x,
                                    y: point.y,
                                    color: goban::StoneColor::White,
                                };
                                if let Err(error) = ui.play_stone(stone) {
                                    eprintln!("{}", error);
                                    std::process::exit(1);
                                }
                            }
                            SgfProp::AB(points) => {
                                for point in points.iter() {
                                    let stone = goban::Stone {
                                        x: point.x,
                                        y: point.y,
                                        color: goban::StoneColor::Black,
                                    };
                                    if let Err(error) = ui.add_stone(stone) {
                                        eprintln!("{}", error);
                                        std::process::exit(1);
                                    }
                                }
                            }
                            SgfProp::AW(points) => {
                                for point in points.iter() {
                                    let stone = goban::Stone {
                                        x: point.x,
                                        y: point.y,
                                        color: goban::StoneColor::White,
                                    };
                                    if let Err(error) = ui.add_stone(stone) {
                                        eprintln!("{}", error);
                                        std::process::exit(1);
                                    }
                                }
                            }
                            SgfProp::AE(points) => {
                                for point in points.iter() {
                                    ui.clear_point((point.x, point.y));
                                }
                            }
                            SgfProp::MN(num) => ui.set_move_number(*num as u64),
                            _ => {}
                        }
                    }
                    // TODO: avoid these stupid clones
                    sgf_node = match sgf_node.clone().into_iter().next() {
                        None => {
                            game_state = GameState::Ended;
                            sgfs.choose(&mut rng).unwrap().clone() // sgfs is never empty
                        }
                        Some(node) => node,
                    };
                    last_action_time = std::time::Instant::now();
                }
            }
            GameState::Ended => {
                if last_action_time.elapsed() > Duration::from_millis(parsed_args.end_delay) {
                    game_state = GameState::New;
                    last_action_time = std::time::Instant::now();
                }
            }
        }

        let physical_size = match xs.inner_size() {
            Ok(physical_size) => physical_size,
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        };
        let (width, height) = (physical_size.width, physical_size.height);
        let scale_factor = match xs.scale_factor() {
            Ok(scale_factor) => scale_factor,
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        };

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
