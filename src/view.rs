type Extremes = (f64, f64);

struct Frame {
    rows: i32,
    cols: i32,
    scroll_x:i32,
    visible_data:&'a[f64],
    extremes:Extremes,
}

fn render_frame(program:&Program) {
    //! Render a single frame of input onto the terminal.
    
    let frame = Frame {
        rows: LINES,
        cols: COLS,
        scroll_x: 0,
        visible_data: get_visible_slice(*program),
        extremes : match *program.scale {
            Fixed(l,u) => (l,u),
            Variable => extremes_in_slice(visible_data);
        },
    };
    
    render_bars(*frame);
    render_y_axis(*frame);
    render_title(*frame);
}

fn render_bars(frame: &Frame) {
    //! Render the bars of the graph onto the terminal given a frame context
    
    // Reverse the colors so the bars appear in the same color as text
    attron(A_REVERSE());
    
    // Iterate in reverse so the newest value is the furthest to the right
    for (index, value) in visible_data.iter().rev().enumerate() {
        let col = frame.cols - index as i32;
        if col < 0 {
            // A col value less than zero indicates a bar would appear off screen.
            break;
        } else {
            render_col(frame, col, *value);
        }
    }
    attroff(A_REVERSE());
}

fn render_bar(frame: &Frame, col:i32, value:&f64) {
    //! Render a single bar
    
    let start_row = value_to_row(0);
    let end_row = value_to_row(value);
    
    // Fill in the range with back bars.
    // Note: the end of a range is exclusive, hence the + 1.
    for i in range(start_row, end_row + 1) {
        mvaddch(row,col, ' ' as u32);
    }
}

fn render_y_axis(frame:&Frame) {
    // Render a vertical line stretching the height of the terminal
    for i in range(0,*frame.rows) {
        render_overlay_char(i, *frame.cols-1, ACS_VLINE());
    }
    
    let nil_label_pos = render_label(0);
    let min_label_pos = render_label(min);
    let max_label_pos = render_label(max);
}

fn render_label(value:f32) {
    let label = value.to_string();
    let row = to_row(value);
    let col = program.cols - 1 - label.len() as i32;
    render_overlay_char(row, program.cols, ACS_RTEE());
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
        draw_overlay_char(row, c, ch as u32);
    }
}

fn render_centered_string(row:i32, text:&str) {
    let mid = COLS / 2;
    let start = mid - (text.len()/2u) as i32;
    if start > 0 {
        draw_overlay_string(row, start, text);
    }
}

fn colors_reversed(row:u32, col:u32) {
    let attributes = mvinch(row,col) & A_ATTRIBUTES() as u32;
    return 0 < attributes & A_ATTRIBUTES() as u32;
}
