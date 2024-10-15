use super::*;

pub fn bigint_to_u128(n: BigInt) -> Result<u128, String> {
    let n_str: String = n.to_string(10).unwrap().into();

    match u128::from_str_radix(n_str.as_str(), 10) {
        Ok(num) => Ok(num),
        Err(cause) => Err(format!("Error while converting BigInt to u128: {cause}").to_string()),
    }
}
