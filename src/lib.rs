#[macro_use]
extern crate paste;

use std::panic;

use wasm_bindgen::prelude::*;

use crate::util::*;
use bitcoin::Network as _Network;
use js_sys::BigInt;
use ordinals::{
    varint::{decode as _decode, encode as _encode},
    Edict as _Edict, Etching as _Etching, Height, Rune as _Rune, RuneId as _RuneId,
    Runestone as _Runestone, Terms as _Terms,
};
use serde::{Deserialize, Serialize};

mod edict;
mod etching;
mod rune;
mod rune_id;
mod runestone;
mod terms;
mod util;
mod varint;

#[wasm_bindgen]
pub fn init_ordinals_lib() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[macro_export]
macro_rules! impl_from {
    ($Type:ident) => {
        paste! {
            // From _Internal to External
            impl From<[<_ $Type>]> for $Type {
                fn from(internal: [<_ $Type>]) -> $Type {
                    $Type { internal }
                }
            }

            // From &_Internal To External, uses Clone
            impl From<&[<_ $Type>]> for $Type {
                fn from(internal: &[<_ $Type>]) -> $Type {
                    $Type::from(internal.clone())
                }
            }

            // From Enternal to _Internal
            impl From<$Type> for [<_ $Type>] {
                fn from(value: $Type) -> [<_ $Type>] {
                    value.internal
                }
            }
        }
    };
}
