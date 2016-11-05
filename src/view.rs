use ncurses::*;
use data::*;

/// Renders a frame of a program
pub fn render_frame(program:&Program) {
    let visible_slice = get_visible_slice(&program.data);
    
    let frame = Frame {
        rows: LINES(),
        cols: COLS(),
        visible_data: visible_slice,
        extremes : match program.scale {
            ScaleMode::Fixed(l,u) => (l,u),
            ScaleMode::Variable => get_extremes(visible_slice),
        },
    };
    
    render_bars(&frame);
    render_axes(&frame);
    
    match program.title {
        Some(ref text) => render_centered_string(0, &text),
        None => {},
    }
}

/// Returns the slice of data currently displayed on the terminal
fn get_visible_slice(data:&Vec<f64>) -> &[f64] {
    let mut start = data.len() as i32 - COLS() - 1i32;
    if start < 0 {
        start = 0;
    }
    return &data[start as usize..];
}

/// Calculates the min and max values within a slice of data
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

/// Render data bars
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
    if value > 0.0 {
        let start = value_to_row(frame, 0.0);
        let end = value_to_row(frame, value);
        for i in end - 1 .. start + 1 {
            mvaddch(i,col,' ' as u32);
        }
    
    } else if value < 0.0 {
        let start = value_to_row(frame, 0.0);
        let end = value_to_row(frame, value);
        for i in start .. end+1 {
            mvaddch(i,col,' ' as u32);
        }
    }
}

/// Draw the axes and labels
fn render_axes(frame:&Frame) {
    // Render a vertical line stretching the height of the terminal
    for i in 0 .. frame.rows {
        render_overlay_char(i, frame.cols-1, ACS_VLINE());
    }
    
    // Render a horizontal line stretching the width of the terminal
    let row = value_to_row(frame,0.0);
    for i in 0 .. frame.cols-2 {
        render_overlay_char(row, i, ACS_HLINE() as u32);
    }
    
    let (min,max) = frame.extremes;
    render_label(frame,0.0);
    render_label(frame,max);
    render_label(frame,min);
}

/// Converts a data value to the terminal row that the value sould be 
/// displayed at
fn value_to_row(frame: &Frame, value:f64) -> i32 {
    let (min,max) = frame.extremes;
    let range = max - min;
    let scale = (frame.rows as f64 - 1.0)/(range as f64);
    let row = value*scale - min*scale;
    return frame.rows - row as i32 - 1;
}

/// Renders a label at a position defined by a data value
fn render_label(frame:&Frame,value:f64) {
    let label = value.to_string();
    let row = value_to_row(frame, value);
    let col = frame.cols - 2 - label.len() as i32;
    render_overlay_char(row, frame.cols - 1, ACS_RTEE());
    render_overlay_string(row, col, &label);
}

/// Renders a character at a given row and column. If colors are reversed
/// at the position before the character is draw, they are reversed for
/// the purposes of drawing the character such that text is not obscured
/// by graph bars
fn render_overlay_char(row:i32, col:i32, ch:u32) {
    if colors_reversed(row, col) {
        attron(A_REVERSE());
        mvaddch(row,col, ch);
        attroff(A_REVERSE());
    } else {
        mvaddch(row,col, ch);
    }
}

/// Renders characters at a given row and column via render_overlay_char
fn render_overlay_string(row:i32, col:i32, txt:&str) {
    for (i,ch) in txt.chars().enumerate() {
        let c = col + i as i32;
        render_overlay_char(row, c, ch as u32);
    }
}

/// Centers a string on a row. Text is rendered using render_overlay_string
fn render_centered_string(row:i32, text:&str) {
    let mid = COLS() / 2;
    let start = mid - (text.len()/2) as i32;
    if start > 0 {
        render_overlay_string(row, start, text);
    }
}

/// Returns true if colors are reveresed at a given row and column
fn colors_reversed(row:i32, col:i32) -> bool {
    let attributes = mvinch(row,col) & A_ATTRIBUTES() as u32;
    return 0 < attributes & A_REVERSE() as u32;
}
