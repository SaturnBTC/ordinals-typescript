use super::*;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq)]
#[wasm_bindgen]
pub struct Terms {
    internal: _Terms,
}

#[wasm_bindgen]
impl Terms {
    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> Option<BigInt> {
        self.internal.amount.map(BigInt::from)
    }

    #[wasm_bindgen(getter)]
    pub fn cap(&self) -> Option<BigInt> {
        self.internal.cap.map(BigInt::from)
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> Vec<BigInt> {
        vec![
            self.internal
                .height
                .0
                .map(BigInt::from)
                .unwrap_or(BigInt::from(-1)),
            self.internal
                .height
                .1
                .map(BigInt::from)
                .unwrap_or(BigInt::from(-1)),
        ]
    }

    #[wasm_bindgen(getter)]
    pub fn offset(&self) -> Vec<BigInt> {
        vec![
            self.internal
                .offset
                .0
                .map(BigInt::from)
                .unwrap_or(BigInt::from(-1)),
            self.internal
                .offset
                .1
                .map(BigInt::from)
                .unwrap_or(BigInt::from(-1)),
        ]
    }
}

impl_from!(Terms);
