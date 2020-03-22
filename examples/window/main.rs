//! Opens an empty window and prints events to stdout.

use blazar::{
    event::{Event, Key},
    window::Window,
};

fn main() {
    let mut window = Window::create("Simple window", 800, 600).unwrap();
    'running: loop {
        while let Some(event) = window.poll_event() {
            println!("{:?}", event);
            match event {
                Event::Close | Event::KeyPress { key: Key::Escape } => break 'running,
                _ => {}
            }
        }
    }
}
