use crate::block::TransactionType;

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

mod block;
mod blockchain;
mod header;

fn main() {
    let keypair = blockchain::generate_ed25519();
    let local_id = header::hash(&block::get_publickey_from_keypair(&keypair).encode());
    let mut chain = blockchain::Blockchain::new();
    chain.genesis(&keypair);
    let mut transactions = vec![];
    let data = "Hello First Transaction";
    let transaction = block::Transaction::new(
        block::PartialTransaction::new(
            TransactionType::Create,
            local_id,
            data.as_bytes().to_vec(),
            2022,
        ),
        &keypair,
    );
    transactions.push(transaction);
    chain.new_block(&keypair, &transactions);
    chain.new_block(&keypair, &transactions);
    println!(
        "The verify result {}",
        chain.blocks.last().unwrap().verify()
    );
    println!("{:?}", chain);
}
