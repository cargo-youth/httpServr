use std::io::{Read,Write};
use std::net::TcpListener;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:3000").expect("bind failed");

    println!("Running on port 3000...");


    for stream in listener.incoming() {//持续的 监听
        let mut stream = stream.unwrap();
        println!("Conection established");

        let mut buffer = [0;1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();

    }
}
