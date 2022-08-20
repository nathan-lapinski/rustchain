mod models;

fn main() {
    println!("Hello, world!");
    let mut block = models::block::Block::new(String::from("data"), String::from("a"));
    block.set_hash();
}
