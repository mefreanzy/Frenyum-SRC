use std::sync::Mutex;

use Frenyum::skeleton::block::Block;
use Frenyum::RPC::peer::{Node, Message};

fn main() {
    println!("Type 'help' to list command.");

    let memory: Vec<Block> = Vec::new();
    let memory_lock: Mutex<Vec<Block>>  = Mutex::new(Vec::new());

    let mut peers: Vec<String> = Vec::new();
}
