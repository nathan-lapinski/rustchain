mod models;

fn main() {
    println!("Hello, world!");
    let mut blockchain = models::blockchain::Blockchain::new();
    blockchain.add_block(String::from("Data 1"));
    blockchain.add_block(String::from("Data 2"));
    blockchain.print();
}
