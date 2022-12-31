// Fix all errors without adding newline
fn main() {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }
    let v = vec!["Hello".to_string(), "World".to_string()];

    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();

    println!("{:?}", v1);
}
