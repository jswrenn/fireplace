extern crate getopts;
extern crate ncurses;
use data::*;
use ncurses::*;
use std::io::{self, BufRead};
use std::env;
use getopts::{Options, Matches};
mod view;
mod data;

fn print_usage(program: &str, opts: Options) {
    println!("Usage: {} [--title <TITLE>] [--fixed [--lower <LOWER BOUND>] --upper <UPPER_BOUND>] [--variable]{}", program, opts.usage(""));
}

fn initialize_program() -> Option<Program> {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];
    
    let mut opt = Options::new();
    opt.optflag("h", "help", "print this help menu");
    opt.optopt("t", "title", "set the title of the graph", "TITLE");
    opt.optflag("f", "fixed", "use a fixed scale (upper bound must be set)");
    opt.optflag("v", "variable", "use a variable scale (default)");
    opt.optopt("l", "lower", "lower bound of y axis when using a fixed scale", "MINIMUM");
    opt.optopt("u", "upper", "upper bound of y axis when using a fixed scale", "MAXIMUM");
    
    let matches = match opt.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.opt_present("h") {
        print_usage(program_name, opt);
        return None;
    }
    
    return Some(Program {
        data: Vec::new(),
        title: matches.opt_str("t"),
        scale: initialize_scale(&matches),
    });
}
    
fn initialize_scale(matches:&Matches)-> ScaleMode {
    if matches.opt_present("f") {
        // Configure lower and upper bounds
        let raw_lower = matches.opt_str("l");
        let raw_upper = matches.opt_str("u");
        
        let lower:f64 = match raw_lower {
            Some(txt) => txt.parse().unwrap(),
            None => 0.0,
        };
        
        let upper = match raw_upper {
            Some(txt) => txt.parse().unwrap(),
            None => {
                panic!("Upper bound must be specified");
            }
        };
        return ScaleMode::Fixed(lower,upper);
    } else {
        return ScaleMode::Variable;
    }
}

fn main() {
    if let Some(mut program) = initialize_program() {
        /* Setup ncurses. */
        initscr();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        // While input is availabe on stdin
        let stdin = io::stdin();
        let stdinlock = stdin.lock();
        for line in stdinlock.lines() {
            if let Ok(line) = line {
                // Parse an f64 from the inputted line
                if let Ok(value) = line.parse::<f64>() {
                    // Push it into the array
                    program.data.push(value);

                    // Clear the screen
                    clear();

                    view::render_frame(&program);

                    // Refresh the screen with the new frame
                    refresh();
                }
            } else {
                break;
            }
        }
        endwin();
    }
}
