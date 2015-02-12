extern crate scribe;
extern crate rustbox;

use std::os;

mod view;

fn main() {
    let path = os::args()[1].clone();
    let mut buffer = scribe::buffer::from_file(&Path::new(path)).unwrap();
    let view = view::new();
    view.display(buffer.data().as_slice());
    view.set_cursor(&*buffer.cursor);
    let mut insert = false;

    loop {
        match view.get_input() {
            Some(c) => {
                if insert {
                    if c == '\\' {
                        insert = false
                    } else {
                        buffer.insert(c.to_string().as_slice());
                        buffer.cursor.move_right();
                        view.set_cursor(&*buffer.cursor);
                        view.display(buffer.data().as_slice());
                    }
                } else {
                    match c {
                        'q' => break,
                        'j' => {
                            buffer.cursor.move_down();
                            view.set_cursor(&*buffer.cursor);
                        },
                        'k' => {
                            buffer.cursor.move_up();
                            view.set_cursor(&*buffer.cursor);
                        },
                        'h' => {
                            buffer.cursor.move_left();
                            view.set_cursor(&*buffer.cursor);
                        },
                        'l' => {
                            buffer.cursor.move_right();
                            view.set_cursor(&*buffer.cursor);
                        },
                        'i' => {
                            insert = true;
                        },
                        's' => {
                            buffer.save();
                        },
                        _ => continue,
                    }
                }
            },
            None => {},
        }
    }
}
