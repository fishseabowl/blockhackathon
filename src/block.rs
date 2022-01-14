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

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
    pub signature: Option<BlockSignature>,
}

impl Block {
    pub fn new(
        header: Header,
        transactions: Vec<Transaction>,
        ed25519_Keypair: identity::ed25519::Keypair,
    ) -> Self {
        Self {
            header: header,
            transactions: transactions,
            signature: None,
        }
    }
}
pub struct Transaction {
    pub nonce: u128,
    pub trans_type: TransactionType,
    pub submmitter: Address,
    pub signature: Option<TransactionSignature>,
    pub payload: Vec<u8>,
}

pub struct Signature {
    signature: Vec<u8>,
    pubkey: identity::ed25519::PublicKey,
}

type TransactionSignature = Signature;
type BlockSignature = Signature;

pub enum TransactionType {
    Create,
}
