
#![feature(globs)]

extern crate ncurses;
use ncurses::*;
use std::io;
use std::num::*;
use std::os;

fn main() {
    let args = os::args();
    let title = args[1].clone();

    /* Setup ncurses. */
    initscr();
    curs_set(CURSOR_INVISIBLE);
    
    let mut data: Vec<i32> = Vec::new();
    for line in io::stdin().lines() {
        clear();
        let input: i32 = from_str(line.unwrap().as_slice().trim()).unwrap();
        data.push(input);
        draw_graph(title.as_slice(),data.as_slice());
    }
    
    endwin();
}

fn draw_graph(title:&str,data:&[i32]) {

    let mut start = data.len() as i32 - COLS - 1i32;
    
    if start < 0 {
        start = 0;
    }

    let visible_slice = data.slice_from(start as uint);

    let mut max = 0i32;
    for i in visible_slice.iter() {
        if *i > max {
            max = *i;
        }
    }
    
    let scale = (LINES as f32) / (max as f32);
    

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
     draw_y_axis(max);
    centered_text(0, title);
    refresh();
}

fn draw_overlay_char(row:i32, col:i32, ch:u32) {
    let attr = mvinch(row,col) & A_ATTRIBUTES() as u32;
    if 0u32 < attr & (A_REVERSE() as u32) {
        attroff(A_REVERSE());
        attron(A_REVERSE());
        mvaddch(row,col, ch);
        attroff(A_REVERSE());
    } else {
        mvaddch(row,col, ch);
    }
}

fn centered_text(row:i32, text:&str) {
    let mid = COLS / 2;
    let start = mid - (text.len()/2u) as i32;
    if start > 0 {
        draw_overlay_string(row, start, text);
    }
}

fn draw_overlay_string(row:i32, col:i32, txt:&str) {
    for (i,ch) in txt.chars().enumerate() {
        let c = col + i as i32;
        draw_overlay_char(row, c, ch as u32);
    }
}

fn draw_y_axis(max:i32) {
    let top_label = max.to_string();
    draw_overlay_string(0, COLS-1i32 - top_label.len() as i32, top_label.as_slice());
    mvaddch(0,COLS-1i32, ACS_RTEE() as u32);
    
    for i in range(1,LINES) {
        draw_overlay_char(i, COLS-1i32, ACS_VLINE());
    }
    
    if LINES > 10 {
        let mid_label = (max/2).to_string();
        draw_overlay_char(LINES/2,COLS-1i32, ACS_RTEE());
        draw_overlay_string(LINES/2, COLS-1i32 - mid_label.len() as i32, mid_label.as_slice());
    }
}

fn draw_col(n:i32, col:i32, scale:f32) {
    let height = (n as f32 * scale).ceil() as u32;
    for i in range(0, height) {
        let row = (LINES-1i32) - i as i32;
        if row < 0 {
            break;
        }
        mvaddch(row,col, ' ' as u32);
    }
}
