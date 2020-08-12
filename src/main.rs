extern crate gl;
extern crate glutin;
extern crate nanovg;
extern crate sgf_parse;

mod args;
mod goban;
mod ui;
mod xscreensaver_context;

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
            eprintln!("Creation of XScreensaver Context failed: {}", e);
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
    let mut ui = match ui::UI::new(
        sgfs,
        parsed_args.move_delay,
        parsed_args.end_delay,
        parsed_args.annotations,
    ) {
        Ok(ui) => ui,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };

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

        if let Err(error) = ui.update_game_state() {
            eprintln!("{}", error);
            std::process::exit(1);
        }

        let (width, height, scale_factor) = match get_geometry(&xs) {
            Ok(values) => values,
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

fn get_geometry(
    xs: &xscreensaver_context::XScreensaverContext,
) -> Result<(u32, u32, f64), Box<dyn std::error::Error>> {
    let physical_size = xs.inner_size()?;
    let (width, height) = (physical_size.width, physical_size.height);
    let scale_factor = xs.scale_factor()?;
    Ok((width, height, scale_factor))
}
