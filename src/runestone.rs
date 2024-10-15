use bitcoin::{consensus::deserialize, psbt::Psbt, Transaction};
use ordinals::Artifact;
use serde::Deserialize;

use crate::{edict::Edict, etching::Etching, rune_id::RuneId};

use super::*;

#[wasm_bindgen]
pub struct Runestone {
    internal: _Runestone,
}

#[wasm_bindgen]
impl Runestone {
    #[wasm_bindgen]
    pub fn default() -> Runestone {
        _Runestone::default().into()
    }

    #[wasm_bindgen]
    pub fn encipher(&self) -> Vec<u8> {
        self.internal.encipher().to_bytes()
    }

    /// Expects a base64 encoded PSBT
    #[wasm_bindgen]
    pub fn decipher(psbt: String) -> Option<Runestone> {
        let psbt = bitcoin::base64::decode(psbt.as_bytes()).unwrap();
        let psbt = Psbt::deserialize(&psbt).unwrap();
        let artifact = _Runestone::decipher(&psbt.extract_tx());

        match artifact {
            Some(Artifact::Runestone(runestone)) => Some(runestone.into()),
            _ => None,
        }
    }

    // Expects a base64 encoded transaction
    #[wasm_bindgen]
    pub fn decipher_transaction(tx: String) -> Option<Runestone> {
        let tx_bytes = hex::decode(&tx).expect("A valid hex string");
        let tx: Transaction = deserialize(&tx_bytes).unwrap();
        let artifact = _Runestone::decipher(&tx);

        match artifact {
            Some(Artifact::Runestone(runestone)) => Some(runestone.into()),
            _ => None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn edicts(&self) -> Vec<Edict> {
        self.internal.edicts.iter().map(Edict::from).collect()
    }

    #[wasm_bindgen(setter)]
    pub fn set_edicts(&mut self, edicts: Vec<Edict>) {
        self.internal.edicts = edicts.into_iter().map(Edict::into).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn etching(&self) -> Option<Etching> {
        self.internal.etching.map(Etching::from)
    }

    #[wasm_bindgen(setter)]
    pub fn set_etching(&mut self, etching: Option<Etching>) {
        self.internal.etching = etching.map(Etching::into)
    }

    #[wasm_bindgen(getter)]
    pub fn mint(&self) -> Option<RuneId> {
        self.internal.mint.map(RuneId::from)
    }

    #[wasm_bindgen(setter)]
    pub fn set_mint(&mut self, mint: Option<RuneId>) {
        self.internal.mint = mint.map(RuneId::into)
    }

    #[wasm_bindgen(getter)]
    pub fn pointer(&self) -> Option<u32> {
        self.internal.pointer
    }

    #[wasm_bindgen(setter)]
    pub fn set_pointer(&mut self, pointer: Option<u32>) {
        self.internal.pointer = pointer
    }
}

impl_from!(Runestone);
