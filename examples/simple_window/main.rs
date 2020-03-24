//! Opens an empty window and prints events to stdout.

use blazar::{
    event::{Event, Key},
    window::{Result, Window},
};

fn main() -> Result {
    let mut window = Window::create("Simple window", 800, 600)?;
    'running: loop {
        while let Some(event) = window.poll_event() {
            println!("{:?}", event);
            match event {
                Event::Close | Event::KeyPress { key: Key::Escape } => break 'running,
                _ => {}
            }
        }
    }
    Ok(())
}
