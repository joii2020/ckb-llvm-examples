mod auto_complete;

use anyhow;
use anyhow::Context;
use auto_complete::auto_complete;
use ckb_debugger_api::embed::Embed;
use ckb_jsonrpc_types::JsonBytes;
use ckb_mock_tx_types::ReprMockTransaction;
use ckb_types::packed::CellOutput;
use ckb_types::prelude::Entity;
use lazy_static::lazy_static;
use molecule::bytes::Bytes;
use serde_json::from_str as from_json_str;
use std::{fs::read_to_string, path::PathBuf};

lazy_static! {
    pub static ref DL_INC: Bytes = Bytes::from(&include_bytes!("../../../../build/dl")[..]);
}

pub fn read_tx_template(file_name: &str) -> Result<ReprMockTransaction, anyhow::Error> {
    let mock_tx =
        read_to_string(file_name).with_context(|| format!("Failed to read from {}", file_name))?;
    let mock_tx = auto_complete(&mock_tx)?;

    let mut mock_tx_embed = Embed::new(PathBuf::from(file_name), mock_tx.clone());
    let mock_tx = mock_tx_embed.replace_all();
    let mut repr_mock_tx: ReprMockTransaction =
        from_json_str(&mock_tx).with_context(|| "in from_json_str(&mock_tx)")?;
    if repr_mock_tx.tx.cell_deps.len() == 0 {
        repr_mock_tx.tx.cell_deps = repr_mock_tx
            .mock_info
            .cell_deps
            .iter()
            .map(|c| c.cell_dep.clone())
            .collect::<Vec<_>>();
    }
    if repr_mock_tx.tx.inputs.len() == 0 {
        repr_mock_tx.tx.inputs = repr_mock_tx
            .mock_info
            .inputs
            .iter()
            .map(|c| c.input.clone())
            .collect::<Vec<_>>();
    }
    Ok(repr_mock_tx)
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hash = CellOutput::calc_data_hash(&DL_INC).as_slice().to_vec();

    let mut tx = read_tx_template("templates/tx.json")?;

    tx.mock_info.inputs.get_mut(0).unwrap().output.lock.args = JsonBytes::from_vec(hash);

    let json = serde_json::to_string_pretty(&tx).unwrap();
    println!("{}", json);

    Ok(())
}
