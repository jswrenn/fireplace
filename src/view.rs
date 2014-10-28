use ncurses::*;
use data::*;

type Extremes = (f64, f64);

struct Frame<'a> {
    rows: i32,
    cols: i32,
    visible_data:&'a[f64],
    extremes:Extremes,
}

pub fn render_frame(program:&Program) {
    //! Render a single frame of input onto the terminal.
    
    let visible_slice = get_visible_slice(&program.data);
    
    let frame = Frame {
        rows: LINES,
        cols: COLS,
        visible_data: visible_slice,
        extremes : match program.scale {
            Fixed(l,u) => (l,u),
            Variable => get_extremes(visible_slice),
        },
    };
    
    render_bars(&frame);
    render_axes(&frame);
    
    match program.title {
        Some(ref text) => render_centered_string(0, text.as_slice()),
        None => {},
    }
}

fn get_visible_slice(data:&Vec<f64>) -> &[f64] {
    let mut start = data.len() as i32 - COLS - 1i32;
    if start < 0 {
        start = 0;
    }
    return data.slice_from(start as uint);
}

fn get_extremes(data:&[f64])-> Extremes{
    let (mut min, mut max) = (0.0f64, 0.0f64);
    for i in data.iter() {
        if *i < min {
            min = *i;
        }
        if *i > max {
            max = *i;
        }
    }
    return (min, max);
}

fn render_bars(frame: &Frame) {
    //! Render the bars of the graph onto the terminal given a frame context
    
    // Reverse the colors so the bars appear in the same color as text
    attron(A_REVERSE());
    
    // Iterate in reverse so the newest value is the furthest to the right
    for (index, value) in frame.visible_data.iter().rev().enumerate() {
        let col = frame.cols - index as i32;
        if col < 0 {
            // A col value less than zero indicates a bar would appear off screen.
            break;
        } else {
            render_bar(frame, col, *value);
        }
    }
    attroff(A_REVERSE());
}

fn render_bar(frame: &Frame, col:i32, value:f64) {
    //! Render a single bar
    
    let (start,end) = if value > 0.0 {
        (value_to_row(frame, 0.0), value_to_row(frame, value))
    } else {
        (value_to_row(frame, value), value_to_row(frame, 0.0))
    };
    
    if value > 0.0{
        let start = value_to_row(frame, 0.0);
        let end = value_to_row(frame, value);
        for i in range(end - 1, start + 1) {
            mvaddch(i,col,' ' as u32);
        }
    
    } else if value < 0.0 {
        let start = value_to_row(frame, 0.0);
        let end = value_to_row(frame, value);
        for i in range(start, end+1) {
            mvaddch(i,col,' ' as u32);
        }
    }
}

fn render_axes(frame:&Frame) {
    // Render a vertical line stretching the height of the terminal
    for i in range(0,frame.rows) {
        render_overlay_char(i, frame.cols-1, ACS_VLINE());
    }
    
    // Render a horizontal line stretching the width of the terminal
    let row = value_to_row(frame,0.0);
    for i in range(0,frame.cols-1) {
        render_overlay_char(row, i, ACS_HLINE() as u32);
    }
    
    let (min,max) = frame.extremes;
    let nil_label_pos = render_label(frame,0.0);
    let max_label_pos = render_label(frame,max);
    let min_label_pos = render_label(frame,min);
}

fn value_to_row(frame: &Frame, value:f64) -> i32 {
    let (min,max) = frame.extremes;
    let range = max - min;
    let scale = (frame.rows as f64 - 1.0)/(range as f64);
    let row = value*scale - min*scale;
    return frame.rows - row as i32 - 1;
}

fn render_label(frame:&Frame,value:f64) {
    let label = value.to_string();
    let row = value_to_row(frame, value);
    let col = frame.cols - 2 - label.len() as i32;
    render_overlay_char(row, frame.cols - 1, ACS_RTEE());
    render_overlay_string(row, col, label.as_slice());
}

fn render_overlay_char(row:i32, col:i32, ch:u32) {
    if colors_reversed(row, col) {
        attron(A_REVERSE());
        mvaddch(row,col, ch);
        attroff(A_REVERSE());
    } else {
        mvaddch(row,col, ch);
    }
}

fn render_overlay_string(row:i32, col:i32, txt:&str) {
    for (i,ch) in txt.chars().enumerate() {
        let c = col + i as i32;
        render_overlay_char(row, c, ch as u32);
    }
}

fn render_centered_string(row:i32, text:&str) {
    let mid = COLS / 2;
    let start = mid - (text.len()/2u) as i32;
    if start > 0 {
        render_overlay_string(row, start, text);
    }
}

fn colors_reversed(row:i32, col:i32) -> bool {
    let attributes = mvinch(row,col) & A_ATTRIBUTES() as u32;
    return 0 < attributes & A_REVERSE() as u32;
}
