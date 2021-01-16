extern crate getopts;

use std::path::PathBuf;

use super::xscreensaver_context::WindowType;

const DEFAULT_MOVE_DELAY: u64 = 5000;
const DEFAULT_END_DELAY: u64 = 10000;

#[derive(Debug)]
pub struct GobanHackArgs {
    pub window_type: WindowType,
    pub sgf_dirs: Vec<PathBuf>,
    pub move_delay: u64,
    pub end_delay: u64,
    pub print_help: bool,
}

#[derive(Debug)]
pub enum UsageError {
    ArgumentParseError,
    TooManyInputsError,
    FlagParseError,
}

impl std::fmt::Display for UsageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            UsageError::ArgumentParseError => write!(f, "Failed to parse arguments."),
            UsageError::TooManyInputsError => write!(f, "Too many inputs."),
            UsageError::FlagParseError => write!(f, "Failed to parse flag."),
        }
    }
}

impl ::std::error::Error for UsageError {}

pub fn parse_args(
    opts: &getopts::Options,
    args: &Vec<String>,
) -> Result<GobanHackArgs, UsageError> {
    let matches = opts
        .parse(&args[1..])
        .map_err(|_| UsageError::ArgumentParseError)?;

    if matches.free.len() > 0 {
        return Err(UsageError::TooManyInputsError);
    };

    let window_type = parse_window_type(&matches)?;
    let sgf_dirs = parse_sgf_dirs(&matches);
    let move_delay = parse_flag_or_default(&matches, "move-delay", DEFAULT_MOVE_DELAY)?;
    let end_delay = parse_flag_or_default(&matches, "end-delay", DEFAULT_END_DELAY)?;
    let print_help = matches.opt_present("h");

    Ok(GobanHackArgs {
        window_type: window_type,
        sgf_dirs: sgf_dirs,
        move_delay: move_delay,
        end_delay: end_delay,
        print_help: print_help,
    })
}

pub fn build_opts() -> getopts::Options {
    let mut opts = getopts::Options::new();
    opts.long_only(true);
    opts.optflag("h", "help", "Display this help and exit");
    opts.optflag("", "root", "Draw to the root window");
    opts.optflag(
        "",
        "window",
        "Draw to the a new window (the default behavior)",
    );
    opts.optopt(
        "",
        "window-id",
        "Window id to draw to (e.g. 0xa001f5)",
        "WINDOW_ID",
    );
    opts.optopt(
        "",
        "move-delay",
        &format!("Time (ms) between moves (default {})", DEFAULT_MOVE_DELAY),
        "NUM",
    );
    opts.optopt(
        "",
        "end-delay",
        &format!(
            "Time (ms) before loading a new game (default {})",
            DEFAULT_END_DELAY
        ),
        "NUM",
    );
    opts.optmulti(
        "",
        "sgf-dir",
        "Directory to search for sgf files. Multiple allowed.",
        "DIR",
    );

    opts
}

pub fn print_usage(program: &str, opts: &getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn parse_flag_or_default<T: ::std::str::FromStr>(
    matches: &getopts::Matches,
    flag: &str,
    default: T,
) -> Result<T, UsageError> {
    matches
        .opt_str(flag)
        .map(|c| c.parse::<T>())
        .unwrap_or(Ok(default))
        .map_err(|_| UsageError::FlagParseError)
}

pub fn parse_sgf_dirs(matches: &getopts::Matches) -> Vec<PathBuf> {
    let sgf_dirs: Vec<_> = matches
        .opt_strs("sgf-dir")
        .iter()
        .map(PathBuf::from)
        .collect();
    if sgf_dirs.len() == 0 {
        get_default_sgf_dir().into_iter().collect()
    } else {
        sgf_dirs
    }
}

pub fn parse_window_type(matches: &getopts::Matches) -> Result<WindowType, UsageError> {
    // If present, use the window id from XSCREENSAVER_WINDOW.
    // Otherwise return the last flag of -root, -window, or -window_id.
    let window_type = match std::env::var("XSCREENSAVER_WINDOW").ok() {
        Some(s) => {
            let window_id = parse_window_id(&s).map_err(|_| UsageError::FlagParseError)?;
            WindowType::WindowId(window_id)
        }
        None => {
            let mut window_type = WindowType::New;
            let mut last_position = 0;
            for position in matches.opt_positions("root") {
                if position >= last_position {
                    last_position = position;
                    window_type = WindowType::Root;
                }
            }
            for position in matches.opt_positions("window") {
                if position >= last_position {
                    last_position = position;
                    window_type = WindowType::New;
                }
            }
            for (position, s) in matches.opt_strs_pos("window-id") {
                if position >= last_position {
                    last_position = position;
                    let window_id = parse_window_id(&s).map_err(|_| UsageError::FlagParseError)?;
                    window_type = WindowType::WindowId(window_id);
                }
            }
            window_type
        }
    };

    Ok(window_type)
}

fn parse_window_id(window_id_string: &str) -> Result<u64, std::num::ParseIntError> {
    if window_id_string.starts_with("0x") {
        u64::from_str_radix(window_id_string.trim_start_matches("0x"), 16)
    } else {
        u64::from_str_radix(&window_id_string, 10)
    }
}

fn get_default_sgf_dir() -> Option<PathBuf> {
    let xdg_data_home: PathBuf = {
        match std::env::var("XDG_DATA_HOME").ok().map(PathBuf::from) {
            Some(path) => path,
            None => {
                let mut path = PathBuf::from(std::env::var("HOME").unwrap_or("".to_string()));
                path.push(".local/share");

                path
            }
        }
    };
    let xdg_data_dirs: Vec<PathBuf> = std::env::var("XDG_DATA_DIRS")
        .unwrap_or("/usr/local/share:/usr/share".to_string())
        .split(":")
        .map(PathBuf::from)
        .collect();
    std::iter::once(xdg_data_home)
        .chain(xdg_data_dirs.into_iter())
        .map(|mut path| {
            path.push("goban-screenhack");
            path
        })
        .filter(|path| path.exists())
        .next()
}
