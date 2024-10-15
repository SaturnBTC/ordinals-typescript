use std::str::FromStr;

use super::*;

#[derive(Default, Debug, PartialEq, Copy, Clone, PartialOrd, Ord, Eq)]
#[wasm_bindgen]
pub struct Rune {
    internal: _Rune,
}

#[wasm_bindgen]
impl Rune {
    #[wasm_bindgen(getter)]
    pub fn rune(&self) -> js_sys::BigInt {
        self.internal.0.into()
    }

    #[wasm_bindgen(js_name = fromString)]
    pub fn from_str(s: &str) -> Rune {
        _Rune::from_str(s).unwrap().into()
    }

    #[wasm_bindgen(js_name = isReserved)]
    pub fn is_reserved(&self) -> bool {
        self.internal.is_reserved()
    }

    #[wasm_bindgen(js_name = minimumAtHeight)]
    pub fn minimum_at_height(network: Network, height: u32) -> Rune {
        _Rune::minimum_at_height(network.to_btc_network(), Height(height)).into()
    }
}

#[wasm_bindgen]
pub enum Network {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

impl Network {
    pub fn to_btc_network(self) -> _Network {
        match self {
            Network::Mainnet => _Network::Bitcoin,
            Network::Testnet => _Network::Testnet,
            Network::Signet => _Network::Signet,
            Network::Regtest => _Network::Regtest,
        }
    }
}

impl_from!(Rune);
