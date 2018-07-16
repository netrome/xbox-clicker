extern crate enigo;
extern crate gilrs;

use enigo::*;
use gilrs::{Gilrs, Event, EventType, Button};

fn main() {
    println!("Hello, world!");

    // Input provided by Gilrs
    let mut gilrs = Gilrs::new();

    // Output provided by Enigo
    let mut enigo = Enigo::new();

    // Event loop
    loop {
        while let Some(Event { event, .. }) = gilrs.next_event() {
            if let EventType::ButtonPressed(button, ..) = event{
                match button{
                    Button::South => enigo.key_sequence("b"),
                    Button::West => enigo.key_click(Key::LeftArrow),
                    Button::East => enigo.key_click(Key::RightArrow),
                    _ => (),
                };
            }
        }
    }
}

