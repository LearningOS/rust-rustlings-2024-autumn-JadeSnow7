// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,   // This message is used when the program should exit
    Echo,   // This message is used to echo a message
    Move,   // This message is used to move an object
    ChangeColor,    // This message is used to change the color of an object
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
