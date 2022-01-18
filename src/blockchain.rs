/*
   Copyright 2021 JFrog Ltd

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/
use super::block::*;
use super::header::*;
use libp2p::identity;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn genesis(&mut self, keypair: &identity::ed25519::Keypair) {
        let local_id = hash(&get_publickey_from_keypair(keypair).encode());
        let genesis_block_header =
            Header::new(PartialHeader::new(hash(b""), local_id, hash(b""), 0, 1));
        let genesis_block = Block::new(genesis_block_header, vec![], keypair);
        self.blocks.push(genesis_block);
    }

    pub fn new_block(
        &mut self,
        keypair: &identity::ed25519::Keypair,
        transactions: &[Transaction],
    ) {
        let last_block = match self.blocks.last() {
            Some(block) => block,
            None => {
                Blockchain::genesis(self, keypair);
                return;
            }
        };

        let local_id = hash(&get_publickey_from_keypair(keypair).encode());
        let transaction_root = hash(&bincode::serialize(transactions).unwrap());
        let block_header = Header::new(PartialHeader::new(
            last_block.header.current_hash,
            local_id,
            transaction_root,
            last_block.header.number + 1,
            last_block.header.nonce + 1,
        ));
        let block = Block::new(block_header, transactions.to_vec(), keypair);
        self.blocks.push(block);
    }
}

pub fn generate_ed25519() -> identity::ed25519::Keypair {
    //RFC8032
    identity::ed25519::Keypair::generate()
}
