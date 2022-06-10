mod models;

fn main() {
    let difficulty = 2;
    let mut blockchain = models::blockchain::Blockchain::new(difficulty);

    // TODO: blockchain.AddBlock("")
    // TODO: blockchain.AddBlock("")

    // TODO: This can be refactored most likely to take a string of data
    models::blockchain::Blockchain::add_block(&mut blockchain, String::from("Test Transaction 1"));
    models::blockchain::Blockchain::add_block(&mut blockchain, String::from("Test Transaction 2"));
    models::blockchain::Blockchain::add_block(&mut blockchain, String::from("Test Transaction 3"));

    // TODO: Add a for loop which loops through the chain, and prints out:
    // Prev block
    // data
    // hash
    println!("Printing contents of Blockchain--------------");
    blockchain.print();
}

