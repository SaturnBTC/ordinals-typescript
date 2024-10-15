use crate::{rune::Rune, terms::Terms};

use super::*;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq)]
#[wasm_bindgen]
pub struct Etching {
    internal: _Etching,
}

#[wasm_bindgen]
impl Etching {
    #[wasm_bindgen(getter)]
    pub fn divisibility(&self) -> Option<u8> {
        self.internal.divisibility
    }

    #[wasm_bindgen(getter)]
    pub fn premine(&self) -> Option<js_sys::BigInt> {
        self.internal.premine.map(js_sys::BigInt::from)
    }

    #[wasm_bindgen(getter)]
    pub fn rune(&self) -> Option<Rune> {
        self.internal.rune.map(Rune::from)
    }

    #[wasm_bindgen(getter)]
    pub fn spacers(&self) -> Option<u32> {
        self.internal.spacers
    }

    #[wasm_bindgen(getter)]
    pub fn symbol(&self) -> Option<char> {
        self.internal.symbol
    }

    #[wasm_bindgen(getter)]
    pub fn terms(&self) -> Option<Terms> {
        self.internal.terms.map(Terms::from)
    }

    #[wasm_bindgen(getter)]
    pub fn turbo(&self) -> bool {
        self.internal.turbo
    }
}

impl_from!(Etching);
