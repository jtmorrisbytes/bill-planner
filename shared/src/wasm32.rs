use rust_decimal;

use wasm_bindgen::prelude::*;

use wasm_bindgen::JsValue;

pub fn try_from_jsvalue(value: JsValue) -> Result<rust_decimal::Decimal, JsValue> {
  use rust_decimal::prelude::FromPrimitive;
  use std::str::FromStr;
  if value.is_null() {
    Err(JsValue::from(crate::Error::TypeError {
      unexpected_type: "null",
      expected_types: "String or Number",
    }))
  } else if value.is_undefined() {
    Err(JsValue::from(
      crate::Error::TypeError {
        unexpected_type: "undefined",
        expected_types: "String or Number",
      }
      .to_string(),
    ))
  } else if value.is_object() {
    Err(JsValue::from(
      crate::Error::TypeError {
        unexpected_type: "Object",
        expected_types: "String or Number",
      }
      .to_string(),
    ))
  } else if value.is_string() {
    match rust_decimal::Decimal::from_str(value.as_string().unwrap().as_str()) {
      Ok(value) => Ok(value),
      Err(rd_error) => Err(JsValue::from(
        crate::Error::ConvertFromJsValueError {
          from_type: "String",
          to_type: "crate::Decimal",
          value: Box::new(crate::string_from_JsValue(value)),
        }
        .to_string(),
      )),
    }
  } else {
    match value.as_f64() {
      Some(number) => Ok(rust_decimal::Decimal::from_f64(number).unwrap()),
      None => Err(JsValue::from(
        crate::Error::ConvertFromJsValueError {
          from_type: "Number",
          to_type: "crate::Decimal",
          value: Box::new(crate::string_from_JsValue(value)),
        }
        .to_string(),
      )),
    }
  }
}

pub fn rust_string_to_decimal(string: String) -> Result<rust_decimal::Decimal, crate::Error> {
  use std::str::FromStr;
  Ok(rust_decimal::Decimal::from_str(string.as_str())?)
}

pub fn sum_string_array(arr: Vec<String>) -> Result<String, crate::Error> {
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

#[wasm_bindgen]
pub fn sum_strings_array(decimal_js_arr: Vec<JsValue>) -> Result<String, JsValue> {
  use std::panic;
  use web_sys::console;
  match panic::catch_unwind(|| {
    decimal_js_arr
      .into_iter()
      .map(|maybe_js_string: JsValue| maybe_js_string.as_string().unwrap())
      .map(|decimal: String| rust_string_to_decimal(decimal).unwrap())
      .sum::<rust_decimal::Decimal>()
      .to_string()
  }) {
    Ok(string) => Ok(string),
    Err(error) => {
      console::log_1(&JsValue::from(format!("{:#?}", error)));
      Err(
        JsValue::from_serde(&crate::Error::Unknown("Unknown Error"))
          .unwrap_or(JsValue::from_str("Unknown Error")),
      )
    }
  }
  // Ok("hello".to_string())
}

#[cfg(test)]
mod tests {
  #[test]
  pub fn sum_rust_strings_valid_nopanic() {
    use super::sum_rust_strings;
    // 2 to power of 28 elements
    // equivelent to the summation
    // of sum j where j=1 to 2^28 or 36,028,797,153,181,696.
    // At this point, nodejs heap will run out of memory becase
    // we have to allocate so many JSValues.
    // we can safely say that we can sum a total of 268,435,456
    // values at once, but should not do it because of memory and time.
    // this test will take more than 60 seconds to complete
    let capacity: usize = 268435456;
    let mut values: Vec<String> = Vec::with_capacity(capacity);
    for val in 1..capacity {
      values.push(val.to_string())
    }
    let result = sum_rust_strings(values).unwrap();
    println!("{}", result)
  }
}
#[cfg(test)]
#[cfg(target_arch = "wasm32")]
pub mod test {

  use wasm_bindgen::JsValue;
  use wasm_bindgen_test::*;
  use web_sys::console;
  #[wasm_bindgen_test]
  pub fn sum_js_strings_valid_nopanic() {
    use super::sum_strings_array;
    // 2 to power of 28 elements
    // equivelent to the summation
    // of sum j where j=1 to 2^27 or 9,007,199,321,849,856.
    // At this point, nodejs heap will run out of memory becase
    // we have to allocate so many JSValues.
    // we can safely say that we can sum a total of 268,435,456
    // values at once, but should not do it because of memory and time.
    // this test will take more than 30 seconds to complete
    let capacity: usize = 134217728;
    let mut values: Vec<JsValue> = Vec::with_capacity(capacity);
    for val in 1..capacity {
      values.push(JsValue::from(val.to_string()))
    }
    let result = sum_strings_array(values).unwrap();
    console::log_1(&JsValue::from(result))
  }
}
