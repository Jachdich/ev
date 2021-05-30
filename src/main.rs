extern crate termion;

use termion::raw::IntoRawMode;
use crate::termion::input::TermRead;
use std::io::Write;

fn main() {
    let stdout = std::io::stdout();
    let stdin = std::io::stdin();
	let mut screen = stdout.into_raw_mode().unwrap();

    for event in stdin.events() {
        let event = event.unwrap();
        match event {
            termion::event::Event::Key(termion::event::Key::Ctrl('c')) => break,
            e                              => write!(screen, "{:?}\r\n", e).unwrap(),
        }

	  	screen.flush().unwrap();
	}
}
