use super::*;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq, Ord, PartialOrd, Default)]
#[wasm_bindgen]
pub struct RuneId {
    internal: _RuneId,
}

#[wasm_bindgen]
impl RuneId {
    #[wasm_bindgen(constructor)]
    pub fn new(block: u64, tx: u32) -> RuneId {
        _RuneId::new(block, tx)
            .map(|internal| RuneId { internal })
            .unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn block(&self) -> u64 {
        self.internal.block
    }

    #[wasm_bindgen(getter)]
    pub fn tx(&self) -> u32 {
        self.internal.tx
    }
}

impl_from!(RuneId);
