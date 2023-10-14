// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)]
enum Message {
    Move {
        x: i32,
        y: i32,
    },
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        match message {
            Message::Move { x, y } => {
                println!("Move: x = {}, y = {}", x, y);
            }
            Message::Echo(s) => {
                println!("Echo: {}", s);
            }
            Message::ChangeColor(r, g, b) => {
                println!("ChangeColor: r = {}, g = {}, b = {}", r, g, b);
            }
            Message::Quit => {
                println!("Quit");
            }
        }
    }
}
