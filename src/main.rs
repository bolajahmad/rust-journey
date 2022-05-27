#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(file: &mut File) -> bool {
    true
}

fn close(file: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(file: &mut File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = file.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let mut file1 = File {
        name: String::from("file1.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut file1);
    let f1_length = read(&mut file1, &mut buffer);
    close(&mut file1);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file1);
    println!("{} is {} bytes long", &file1.name, f1_length);
    println!("{}", text);
}