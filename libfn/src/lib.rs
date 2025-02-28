mod utils;

pub mod types;

mod wallet;
pub use wallet::WalletEntry;

mod builder;
pub use builder::Builder;

pub mod net;

pub mod entity;

mod mapper;

mod error;
pub use error::{Error, Result};

// pub async fn build_transaction<R: CryptoRng + RngCore>(
//     prng: &mut R,
//     entries: Vec<Entry>,
// ) -> Result<Transaction> {
//     let inputs = Vec::new();
//     let outputs = Vec::new();
//
//     let mut tx = Transaction {
//         txid: H512::zero(),
//         inputs: inputs,
//         outputs: outputs,
//         proof: zei_body.proofs.asset_type_and_amount_proof,
//         signatures: Vec::new(),
//     };
//
//     log::debug!("Result tx is: {:?}", tx);
//
//     tx.signature(keypairs)?;
//
//     Ok(tx)
//     let mut input_ids = Vec::new();
// let mut inputs = Vec::new();
// let mut output_ids = Vec::new();
// let mut outputs = Vec::new();
// let mut output_pks = Vec::new();
// let mut keypairs = Vec::new();
//
// let mut transfer_entry = Vec::new();
//
// let mut index = 0;
//
// for entry in entries {
//     match entry {
//         Entry::Issue(e) => {
//             keypairs.push(e.keypair.clone());
//             output_pks.push(e.keypair.get_pk_ref().clone().into());
//             let output = e.to_output_asset_record(prng)?;
//             input_ids.push((H512::zero(), index, InputOperation::IssueAsset));
//             output_ids.push(OutputOperation::IssueAsset);
//             inputs.push(output.clone());
//             outputs.push(output);
//         }
//         Entry::Transfer(e) => {
//             transfer_entry.push(e);
//         }
//     };
//     index += 1;
// }
//
// let ios = build_input_asset_record_and_id(prng, transfer_entry).await?;
//
// for input in ios.0 {
//     keypairs.push(input.0);
//     input_ids.push((input.1, input.2, InputOperation::TransferAsset));
//     inputs.push(input.3);
// }
//
// for output in ios.1 {
//     output_ids.push(OutputOperation::TransferAsset);
//     outputs.push(output.0);
//     output_pks.push(output.1);
// }
//
// log::debug!("Inputs is : {:?}", inputs);
// log::debug!("Outputs is : {:?}", outputs);
//
// let zei_body = gen_xfr_body(prng, &inputs, &outputs)?;
//
// let mut tx_inputs = Vec::new();
// let mut tx_outputs = Vec::new();
//
// for iii in input_ids {
//     tx_inputs.push(Input {
//         txid: iii.0,
//         n: iii.1,
//         operation: iii.2,
//     });
// }
//
// for i in 0..zei_body.outputs.len() {
//     let operation = &output_ids[i];
//     let owner_memo = &zei_body.owners_memos[i];
//     let core = &zei_body.outputs[i];
//     let address = &output_pks[i];
//     tx_outputs.push(Output {
//         core: utxo::Output {
//             address: address.clone(),
//             amount: core.amount.clone(),
//             asset: core.asset_type.clone(),
//             owner_memo: owner_memo.clone(),
//         },
//         operation: operation.clone(),
//     });
// }
//
// }
