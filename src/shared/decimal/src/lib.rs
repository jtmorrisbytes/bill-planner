// because of how wasm_pack binds functions together into modules,
// this should mimic a module
pub mod error;

use error::Error;
#[cfg(target_arch = "wasm32")]
pub mod wasm32;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub fn string_from_jsvalue(value: JsValue) -> String {
  format!("{:#?}", value)
}

pub fn rust_string_to_decimal(string: String) -> Result<rust_decimal::Decimal, Error> {
  use std::str::FromStr;
  Ok(rust_decimal::Decimal::from_str(string.as_str())?)
}

pub fn sum_rust_strings(arr: Vec<String>) -> Result<String, Error> {
  // try to convert the vector of string to decimal
  Ok(
    arr
      .into_iter()
      .map(|val: String| rust_string_to_decimal(val).unwrap())
      .sum::<rust_decimal::Decimal>()
      .to_string(),
  )
  // Ok(String::from(""))
}
pub fn try_from_f64(float: f64) -> Result<rust_decimal::Decimal, Error> {
  use rust_decimal::prelude::FromPrimitive;
  match rust_decimal::Decimal::from_f64(float) {
    Some(value) => Ok(value),
    None => Err(Error::ConvertFromF64Error {
      value: Box::new(float),
    }),
  }
}
pub fn try_from_string(string: &String) -> Result<rust_decimal::Decimal, Error> {
  use std::str::FromStr;
  match rust_decimal::Decimal::from_str(&string) {
    Ok(decimal) => Ok(decimal),
    Err(error) => Err(Error::ConvertFromStringError {
      message: Box::new(format!("{}", error)),
    }),
  }
}
#[cfg(test)]
pub mod tests {
  use super::try_from_string;
  use crate::Error;
  #[test]
  pub fn test_try_from_string() {
    // this is Number.MAX_SAFE_INTEGER from javascript
    let working_int: String = String::from("9007199254740991");
    // the code is required to work up to three decimal places
    let working_float: String = String::from("9007199254740991.99");
    let invalid_format = String::from("hello");
    let extremely_large_number = String::from("18014398509481983.98");

    assert_eq!(try_from_string(&working_int).is_ok(), true);
    assert_eq!(try_from_string(&working_float).is_ok(), true);
    assert_eq!(try_from_string(&extremely_large_number).is_ok(), true);
    println!("{}", try_from_string(&extremely_large_number).unwrap());

    assert_eq!(
      format!(
        "{}",
        try_from_string(&working_float).unwrap() + try_from_string(&working_float).unwrap()
      ),
      extremely_large_number
    );
    assert_eq!(try_from_string(&invalid_format).is_err(), true);
    println!(
      "assert invalid string is error: output: {}",
      try_from_string(&invalid_format).unwrap_err()
    )
  }
}
