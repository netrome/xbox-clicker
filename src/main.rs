extern crate enigo;
extern crate gilrs;

use enigo::*;
use gilrs::{Button, Event, EventType, Gilrs, Axis};

fn main() {
    println!("Hello, world!");

    // Input provided by Gilrs
    let mut gilrs = Gilrs::new().unwrap();

    // Output provided by Enigo
    let mut enigo = Enigo::new();

    // Event loop
    loop {
        while let Some(Event { event, .. }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, ..) => match button {
                    Button::South => enigo.key_click(Key::DownArrow),
                    Button::West => enigo.key_click(Key::LeftArrow),
                    Button::East => enigo.key_click(Key::RightArrow),
                    Button::North => enigo.key_click(Key::UpArrow),
                    _ => (),
                },
                EventType::AxisChanged(Axis::LeftStickY, x, _) => {
                    enigo.mouse_scroll_y((x * 1.2) as i32)
                }
                _ => println!("Event: {:?}", event),
            };
        }
    }
}
