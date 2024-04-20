// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit(String),
    Echo(String),
    Move(i32, i32),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("{:?}", Message::Quit("Goodbye, world!".to_string()));

    println!("{:?}", Message::Echo("Hello, world!".to_string()));

    println!("{:?}", Message::Move(10, 20));

    println!("{:?}", Message::ChangeColor(255, 0, 0));
}
