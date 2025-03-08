use core::blockchain;
fn main() {

    let mut bc=blockchain::BlockChain::new_block_chain();

    bc.add_block("xiaoming -> xiaohong:5btc".to_string());
    bc.add_block("me -> mom:100btc".to_string());

    for b in bc.blocks {
        println!("------------------------------");
        println!("{:#?}",b);
        println!("------------------------------");
    }

}
