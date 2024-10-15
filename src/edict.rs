use crate::rune_id::RuneId;

use super::*;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq)]
#[wasm_bindgen]
pub struct Edict {
    internal: _Edict,
}

#[wasm_bindgen]
impl Edict {
    #[wasm_bindgen(constructor)]
    pub fn new(id: RuneId, amount: BigInt, output: u32) -> Result<Edict, String> {
        let amount = bigint_to_u128(amount)?;

        Ok(Edict {
            internal: _Edict {
                id: id.into(),
                amount,
                output,
            },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> RuneId {
        self.internal.id.into()
    }

    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> js_sys::BigInt {
        self.internal.amount.into()
    }

    #[wasm_bindgen(getter)]
    pub fn output(&self) -> u32 {
        self.internal.output
    }
}

impl_from!(Edict);
