fn main() {
    println!("Type 'help' to list command.");

    let chain: Arc<Mutex<Vec<Block>>> = Arc::new(Mutex::new(Vec::new()));
    let mut peers: Vec<String> = Vec::new();
}
