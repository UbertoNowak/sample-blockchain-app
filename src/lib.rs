use chrono::Utc;
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        Self { chain: Vec::new() }
    }

    // create first block in chain, with hardcoded data
    pub fn create_genesis_block(&mut self) {
        let genesis_block = Block {
            id: 0,
            data: String::from("Genesis block"),
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("Genesis previos block's hash"),
            hash: String::from("Genesis block's hash"),
        };
        self.chain.push(genesis_block);
    }
}

struct Block {
    id: u64,
    data: String,
    timestamp: i64,
    previous_hash: String,
    hash: String, // hash of data, timestamp and previous hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_genesis_block() {
        let mut blockchain = Blockchain::new();
        blockchain.create_genesis_block();
        assert_eq!(blockchain.chain.iter().len(), 1);
        assert_eq!(blockchain.chain[0].id, 0);
    }
}
