extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::io::{Read, Write, Error, ErrorKind};
use std::net::TcpStream
use std::sync::{Arc, Mutex};

use crate::skeleton::block::Block;

enum Message
{
    Block(Block),               // A block message is a message containing a `Block` object.
}

#[derive(Serialize, Deserialize, Debug)]
struct Node
{
    adress: String,
    port: u16,
    memory: Vec<Block>,
    memory_lock: Mutex<Vec<Block>>,
    peers: Vec<String>,
}

impl Node
{
    fn new(adress: String, port: u16, memory: Vec<Block>, memory_lock: Mutex<Vec<Block>>, peers: Vec<String>)
    {
        // Create new node
        let mut node = Node {
            adress,
            port,
            memory,
            memory_lock,
            peers,
        };
    }

    fn send_message(&mut self, stream: &mut TcpStream, message: Message) -> Result<(), Error>
    {
        let serialized = serde_json::to_string(&message)?;

        stream.write_all(serialized.as_bytes())?;
        Ok(())
    }

    fn receive_message(&mut self, stream: &mut TcpStream) -> Result<Message, Error>
    {
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer);

        let message: Message = serde_json::from_str(&buffer)?;
        Ok(message)
    }
}
