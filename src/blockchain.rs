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
use libp2p::{identity, PeerId};
use multihash::{Code, Multihash, MultihashDigest};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn genesis(&mut self, localId: Address) {
        let genesis_block = Block.new();
        self.blocks.push(genesis_block);
    }
}

pub fn generate_ed25519() -> identity::ed25519::Keypair {
    //RFC8032
    identity::ed25519::Keypair::generate()
}

pub fn signature(keypair: &identity::ed25519::Keypair, msg: &[u8]) -> Vec<u8> {
    (*keypair).sign(msg)
}

pub fn get_publickey_from_keypair(
    keypair: &identity::ed25519::Keypair,
) -> identity::ed25519::PublicKey {
    (*keypair).public()
}

pub fn verify(pubkey: &identity::ed25519::PublicKey, msg: &[u8], sig: &[u8]) -> bool {
    (*pubkey).verify(msg, sig)
}
