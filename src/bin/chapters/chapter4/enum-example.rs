#![allow(dead_code)] // The '!' means apply to the whole file
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Each variant can hold different data!
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Named fields (like struct)
    Write(String),              // Single value
    ChangeColor(i32, i32, i32), // Multiple values (like tuple)
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("Quitting"),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
        }
    }
}

fn match_return(dir: Direction) -> () {
    match dir {
        Direction::Up => println!("Going to the top"),
        Direction::Down => println!("Going down"),
        Direction::Left => println!("Going to the left"),
        Direction::Right => println!("Going to the right"),
    }
}

fn main() {
    let dir = Direction::Up;
    match_return(dir);
    let msg = Message::Write(String::from("Hello World"));
    Message::process(&msg);
    let v2 = Message::Move { x: 20, y: 40 };
    // common way to use method lol!
    v2.process();
}
