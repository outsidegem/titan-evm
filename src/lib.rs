#![deny(clippy::all)]
#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::Buffer;
use revm::{
    context::TxEnv,
    database::InMemoryDB,
    primitives::{Bytes, TxKind},
    Context, ExecuteEvm, MainBuilder, MainContext,
};

#[napi]
pub fn execute_rda_payload(bytecode: Buffer) -> String {
    // 1. Initialize sterile DB for the Refined Data Agents
    let db = InMemoryDB::default();
    let ctx = Context::mainnet().with_db(db);
    let mut evm = ctx.build_mainnet();

    // 2. Map RDA payload directly to a standalone TxEnv
    let mut tx_env = TxEnv::default();
    tx_env.kind = TxKind::Create;
    tx_env.data = Bytes::from(bytecode.as_ref().to_vec());

    // 3. Inject the payload at execution time
    match evm.transact(tx_env) {
        Ok(tx_res) => format!("RDA Payload Executed! Gas consumed: {}", tx_res.result.gas_used()),
        Err(e) => format!("EVM Fault: {:?}", e),
    }
}
