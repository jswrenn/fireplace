#![feature(globs)]
extern crate getopts;
extern crate ncurses;
use view::*;
use data::*;
use ncurses::*;
use std::io;
use std::num::*;
use std::os;
use getopts::{optopt,optflag,getopts,OptGroup,Matches};
mod view;
mod data;

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [--title <TITLE>] [--fixed [--lower <LOWER BOUND>] --upper <UPPER_BOUND>] [--variable]", program);
    for opt in _opts.iter() {
        print!("-{} ",opt.short_name);
        print!("--{} ",opt.long_name);
        print!("{} ",opt.hint);
        print!("{}\n",opt.desc);
    }
}

fn initialize_program() -> Program {
    let args: Vec<String> = os::args();
    let program_name = args[0].clone();
    
    
    let opts = [
        optflag("h", "help", "print this help menu"),
        optopt("t", "title", "set the title of the graph", "TITLE"),
        optflag("f", "fixed", "used a fixed scale"),
        optflag("v", "variable", "use a variable scale"),
        optopt("l", "lower", "lower bound of y axis when using a fixed scale", "MINIMUM"),
        optopt("u", "upper", "upper bound of y axis when using a fixed scale", "MAXIMUM"),
    ];
    
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_string()) }
    };
    
    if matches.opt_present("h") {
        print_usage(program_name.as_slice(), opts);
        fail!("Help Menu");
    }
    
    return Program {
        data: Vec::new(),
        title: matches.opt_str("t"),
        scale: initialize_scale(&matches),
    };
}
    
fn initialize_scale(matches:&Matches)-> ScaleMode {
    if matches.opt_present("f") {
        // Configure lower and upper bounds
        let raw_lower = matches.opt_str("l");
        let raw_upper = matches.opt_str("u");
        
        let lower:f64 = match raw_lower {
            Some(txt) => from_str(txt.as_slice()).unwrap(),
            None => 0.0,
        };
        
        let upper = match raw_upper {
            Some(txt) => from_str(txt.as_slice()).unwrap(),
            None => {
                fail!("Upper bound must be specified");
            }
        };
        return Fixed(lower,upper);
    } else {
        return Variable;
    }
}

fn main() {
    let mut program: Program = initialize_program();

    /* Setup ncurses. */
    initscr();
    curs_set(CURSOR_INVISIBLE);

    // While input is availabe on stdin
    for line in io::stdin().lines() {
        // Clear the screen
        clear();
        
        // Parse an f64 from the inputted line
        let value:f64 = from_str(line.unwrap().as_slice().trim()).unwrap();
        
        // Push it into the array
        program.data.push(value);
        
        view::render_frame(&program);
        
        // Refresh the screen with the new frame
        refresh();
    }
    endwin();
}
