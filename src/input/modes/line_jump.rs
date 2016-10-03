use models::application::modes::LineJumpMode;
use commands::{Command, application, line_jump};
use input::Key;

pub fn handle(mode: &mut LineJumpMode, input: Key) -> Option<Command> {
    match input {
        Key::Esc => Some(application::switch_to_normal_mode),
        Key::Char('\n') => Some(line_jump::accept_input),
        Key::Backspace => {
            // Remove a character from the search term.
            mode.input.pop();

            None
        }
        Key::Char(c) => {
            // Add a character to the search term.
            mode.input.push(c);

            None
        }
        Key::Ctrl('z') => Some(application::suspend),
        Key::Ctrl('c') => Some(application::exit),
        _ => None,
    }
}
