#[derive(Debug)]
struct Account {
    // Perhaps these should be a fixed length &string instead
// private_key: String,
// public_key: String,
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Hash: {:?}", self)
    }
}

impl Account {
    fn new() -> Account {
        Account {
            // Call function to generate the key pairs
        }
    }

    /// Create a transaction
    fn create_transaction() {
        // Serialize the transaction
    }

    /// Submit a transaction
    fn submit_transaction() {
        // Broadcast the transaction to network
    }
}

#[derive(Debug)]
struct Transaction {
    /// Perhaps this should be a fixed length &string instead
    hash: String,

    from: Account,
    to: Account,

    /// why usize??
    /// Should not allow floating points and negative numbers
    amount: usize,

    /// Perhaps this should be a fixed length &string instead
    signature: String,
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Hash: {:?}", self.hash)
    }
}

// Need a way to serialize a transaction too, so that it can be hashed.
// the serialization method must be specified in the spec

#[derive(Debug)]
struct Block {
    /// Perhaps this should be a fixed length &string instead
    hash: String,

    /// Perhaps this should be a fixed length &string instead
    parent_hash: String,
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Hash: {:?}", self.hash)
    }
}

#[derive(Debug)]
struct Chain {
    blocks: Vec<Block>,
}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Hash: {:?}", self.blocks)
    }
}

impl Chain {
    /// Instantiate a new blockchain
    fn new() -> Chain {
        Chain {
            blocks: vec![Block {
                hash: String::from(""),
                parent_hash: String::from(""),
            }],
        }
    }

    /// Get the latest block from the blockchain
    fn last_block(&self) -> &Block {
        // Can safely expect a Some(..) value here as there will always be at least 1 block in the blocks vector,
        // if the blockchain is correctly initialized using the Chain::new() constructor.
        self.blocks
            .last()
            .expect("UNEXPECTED ERROR: Missing genesis block")
    }

    /// Get the latest block from the blockchain
    fn add_block(&mut self, block: Block) -> Result<(), ()> {
        // @todo Verify the block first
        self.blocks.push(block);

        Ok(())
    }
}

use sha3::{Digest, Sha3_256};

#[forbid(unsafe_code)]

fn hash_sha3_256() -> String {
    format!("{:x}", Sha3_256::new().chain(b"abc").finalize())
}

fn main() {
    let result = hash_sha3_256();
    println!("test 1 3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532");
    println!("test 2 {}", result);
    println!(
        "test 3 {}",
        result == "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"
    );
}
