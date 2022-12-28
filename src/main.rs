use std::sync::Mutex;
use std::thread::spawn;
use std::net::TcpListener;
use std::io::{Read, Write, Error};
use std::io;

use Frenyum::skeleton::block::Block;
use Frenyum::RPC::peer::{Node, Message};

fn main() {
    println!("Type 'help' to list command.");

    // Create new node structure
    let adrr: String = "0.0.0.0".to_string();
    let nport: u16 = 1000;
    let mut node = Node::new(adrr, nport);

    spawn(move || {
        let listener = TcpListener::bind(&format!("{}:{}", node.adress, node.port)).unwrap();

        for income in listener.incoming()
        {
            let mut stream = income.unwrap();
            println!("New connection accepted: {}", stream.peer_addr().unwrap());

            let message = Node::receive_message(&mut stream).unwrap();
            println!("Message received");

            match message
            {
                Message::Block(block) => {
                    let lock = node.memory_lock.lock().unwrap();

                    node.memory.push(block);
                }
            }
        }

    });

    loop
    {
        let mut input = String::new();
        println!("Enter RPC command: ");
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.trim().split(" ").collect();
        let rpc = input[0];

        if rpc == "help"
        {
            println!("RPC commands:
                     help: List available RPC commands
                     add_tx: Add new transaction data");
        }
    }
}
