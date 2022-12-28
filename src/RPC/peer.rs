extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::io::{Read, Write, Error, ErrorKind};
use std::net::TcpStream;
use std::sync::Mutex;

use crate::skeleton::block::Block;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message
{
    Block(Block),               // A block message is a message containing a `Block` object.
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node
{
    pub adress: String,
    pub port: u16,
    pub memory: Vec<Block>,
    pub memory_lock: Mutex<Vec<Block>>,
    pub peers: Vec<String>,
}

impl Node
{
    pub fn new(adress: String, port: u16) -> Node
    {
        // Create new node
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
