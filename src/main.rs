use std::sync::Mutex;
use std::thread::spawn;
use std::net::TcpListener;
use std::io::{Read, Write, Error};
use std::io;
use rocksdb::DB;


use Frenyum::skeleton::block::Block;
use Frenyum::skeleton::blockchain::Blockchain;
use Frenyum::RPC::peer::{Node, Message};

fn main() {
    println!("Welcome to Frenyum Protocol by freanzy");
    println!("----------------------------");
    println!("Type 'help' to list command.");

    // New blockchain
    let blockchain = Blockchain::new("blocks_db")

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
                Message::TXData(TransactionData) => {
                    let lock = node.memory_lock.lock().unwrap();

                    node.memory.push(TransactionData);
                }
                Message::Block(Block) => {
                    Blockchain.db
                    .put(Block.index.to_string().as_bytes(),
                    Block.time.to_string().as_bytes(),
                    Block.nonce.to_string().as_bytes(),
                    serde_json::to_vec(&Block).unwrap()),
                    .unwrap();
                }
            }
        }

    });

    loop
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.trim().split(" ").collect();
        let rpc = input[0];

        if rpc == "help"
        {
            println!("RPC commands:
                     help: List available RPC commands
                     add_tx: Add new transaction data
                     exit: Exit to program");
        }
        else if rpc == "exit"
        {
            println!("Thanks for using Frenyum.");
            break;
        }
        else
        {
            println!("Incorrect RPC command. Please 'help' to list command.")
        }
    }
}
