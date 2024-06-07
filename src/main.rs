use blockchain::Blockchain;

mod block;
mod blockchain;
fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("First Block".to_string());
    blockchain.add_block("Second Block".to_string());
    blockchain.add_block("Second Block".to_string());
    blockchain.add_block("Second Block".to_string());
    blockchain.add_block("Second Block".to_string());
    blockchain.add_block("Second Block".to_string());
    blockchain.add_block("Second Block".to_string());
    blockchain.add_block("Second Block".to_string());

    for block in blockchain.chain.iter() {
        println!("{:?}", block);
    }

    println!("Is blockchain valid? {}", blockchain.is_valid());
}
