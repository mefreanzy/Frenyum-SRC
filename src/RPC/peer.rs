use std::sync::Mutex;
use std::thread::spawn;
use std::net::TcpListener;
use std::io::{Read, Write, Error, ErrorKind};

use Frenyum::skeleton::block::Block;
use Frenyum::RPC::peer::{Node, Message};

fn main() {
    println!("Type 'help' to list command.");

    let memory: Vec<Block> = Vec::new();
    let memory_lock: Mutex<Vec<Block>>  = Mutex::new(Vec::new());

    let mut peers: Vec<String> = Vec::new();

    spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:1000");

        for income in listener.expect("Adress is wrong.").incoming()
        {
            let mut stream = income.unwrap();
            println!("New connection accepted: {}", stream.peer_addr().unwrap());

            let message = receive_message(&mut stream).unwrap();
            println!("Message received: {}", message);

            match message
            {
                Message::Block(block) => {
                    let lock = memory_lock.lock().unwrap();

                    memory.push(block);
                }
            }
        }

    });
}
