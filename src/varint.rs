use super::*;

#[derive(Default, Debug, PartialEq, Clone, Eq)]
#[wasm_bindgen]
pub struct DecodeResult {
    n: BigInt,
    size: usize,
}

#[wasm_bindgen]
pub fn decode(buffer: &[u8]) -> DecodeResult {
    let (n, size) = _decode(buffer).unwrap();

    DecodeResult { n: n.into(), size }
}

#[wasm_bindgen]
pub fn encode(n: BigInt) -> Vec<u8> {
    let n = bigint_to_u128(n).unwrap();

    _encode(n)
}

#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub enum Error {
    Overlong,
    Overflow,
    Unterminated,
}
