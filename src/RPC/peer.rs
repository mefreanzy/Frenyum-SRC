use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use crate::Skeleton::block::Block;

struct Node
{
    id: String,
    blockchain: Vec<<Mutex<Block>>,
}

impl Node
