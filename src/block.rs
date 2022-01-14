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

pub enum TransactionType {
    Create,
}

pub struct Signature {
    signature: Vec<u8>,
    pubkey: identity::ed25519::PublicKey,
}

impl Signature {
    pub fn new(msg: &[u8], keypair: &identity::ed25519::Keypair) -> Self {
        Self {
            signature: sign(msg, keypair),
            pubkey: get_publickey_from_keypair(keypair),
        }
    }
}

type TransactionSignature = Signature;
type BlockSignature = Signature;

pub fn sign(msg: &[u8], keypair: &identity::ed25519::Keypair) -> Vec<u8> {
    (*keypair).sign(msg)
}

pub fn get_publickey_from_keypair(
    keypair: &identity::ed25519::Keypair,
) -> identity::ed25519::PublicKey {
    (*keypair).public()
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
    pub signature: BlockSignature,
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
        }
    }
}

pub struct Transaction {
    pub trans_type: TransactionType,
    pub submmitter: Address,
    pub timestamp: u64,
    pub payload: Vec<u8>,
    pub nonce: u128,
    pub transaction_hash: HashDigest,
    pub signature: TransactionSignature,
}

impl Transaction {
    pub fn new(
        partial_transaction: PartialTransaction,
        ed25519_Keypair: identity::ed25519::Keypair,
    ) -> Self {
        let hash = hash(&(bincode::serialize(&partial_transaction).unwrap()));
        Self {
            trans_type: partial_transaction.trans_type,
            submmitter: partial_transaction.submmitter,
            timestamp: partial_transaction.timestamp,
            payload: partial_transaction.payload,
            nonce: partial_transaction.nonce,
            transaction_hash: hash,
            signature: Signature.new(hash, ed25519_Keypair),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PartialTransaction {
    pub trans_type: TransactionType,
    pub submmitter: Address,
    pub timestamp: u64,
    pub payload: Vec<u8>,
    pub nonce: u128,
}

impl PartialTransaction {
    pub fn new(trans_type: trans_type, submmitter: Address, payload: Vec<u8>, nonce: u128) -> Self {
        Self {
            trans_type: trans_type,
            submmitter: submmitter,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            payload: payload,
            nonce: nonce,
        }
    }
}
