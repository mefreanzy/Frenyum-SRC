extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::io::{Read, Write, Error};
use std::net::TcpStream;
use std::sync::Mutex;

use crate::skeleton::block::Block;
use crate::skeleton::transaction::TransactionData;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message
{
    TXData(TransactionData),    // A TXData message is a message containing a `TransactionData` object.
    Block(Block),               // A Block message is a message containing a `Block` object.
}

// What is mempool?
// https://www.alchemy.com/overvi

#[derive(Serialize, Deserialize, Debug)]
pub struct Node
{
    pub adress: String,
    pub port: u16,
    pub memory: Vec<TransactionData>,
    pub memory_lock: Mutex<Vec<TransactionData>>,
    pub peers: Vec<String>,
}

impl Node
{
    pub fn new(adress: String, port: u16) -> Node
    {
        // Create a new node.
        let node = Node {
            adress,
            port,
            memory: Vec::new(),
            memory_lock: Mutex::new(Vec::new()),
            peers: Vec::new(),
        };

        node
    }

    pub fn send_message(stream: &mut TcpStream, message: Message) -> Result<(), Error>
    {
        let serialized = serde_json::to_string(&message)?;

        stream.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn receive_message(stream: &mut TcpStream) -> Result<Message, Error>
    {
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer);

        let message: Message = serde_json::from_str(&buffer)?;
        Ok(message)
    }
}
