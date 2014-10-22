
#![feature(globs)]

extern crate ncurses;

use ncurses::*;
use std::io;
use std::num::*;

static WINDOW_HEIGHT: i32 = 3;
static WINDOW_WIDTH: i32 = 10;



fn main()
{
    /* Setup ncurses. */
    initscr();
    
    let mut data: Vec<int> = Vec::new();
    let mut t = 0f32;
    for line in io::stdin().lines() {
        clear();
        let input: int = from_str(line.unwrap().as_slice().trim()).unwrap();
        let n = (50f32*(t).sin()) as i32 + 70i32;
        t = t+0.1f32;
        data.push(input);
        draw_graph(data.as_slice());
    }
    endwin();
}

fn draw_graph(data:&[int]) {
    mvaddch(0,0, ACS_VLINE());

    let mut start = data.len() as i32 - COLS - 1i32;
    
    if start < 0 {
        start = 0;
    }

    let visible_slice = data.slice_from(start as uint);

    let mut max = 0i;
    for i in visible_slice.iter() {
        if *i > max {
            max = *i;
        }
    }
    
    let scale = (LINES as f32) / (max as f32);
    
    draw_y_axis(max);

    attron(A_REVERSE());
    
    for (i, n) in visible_slice.iter().rev().enumerate() {
        let col = COLS - i as i32;
        if col < 0 {
            continue;
        } else {
            draw_col(*n,col,scale);
        }
    }
    attroff(A_REVERSE());
    refresh();

}

fn draw_y_axis(max:int) {
    let top_label = max.to_string();
    mvprintw(0, COLS-1i32 - top_label.len() as i32, top_label.as_slice());
    mvaddch(0,COLS-1i32, ACS_RTEE() as u32);
    for i in range(1,LINES) {
        mvaddch(i,COLS-1i32, ACS_VLINE() as u32);
    
    
    }
}

fn draw_col(n:int, col:i32, scale:f32) {
    let height = (n as f32 * scale) as u32;
    for i in range(0, height) {
        let row = LINES - i as i32;
        if row < 0 {
            break;
        }
        mvaddch(row,col, ' ' as u32);
    }
}
