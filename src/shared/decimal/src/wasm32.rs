use rust_decimal;

use wasm_bindgen::prelude::*;

use wasm_bindgen::JsValue;

pub fn try_from_jsvalue(value: JsValue) -> Result<rust_decimal::Decimal, JsValue> {
  use rust_decimal::prelude::FromPrimitive;
  use std::str::FromStr;
  #[allow(non_upper_case_globals)]
  const expected_types: &'static str = "String or Number";
  if value.is_null() {
    Err(JsValue::from(
      crate::Error::TypeError {
        unexpected_type: "null",
        expected_types,
        value: Box::new("".to_string()),
      }
      .to_string(),
    ))
  } else if value.is_undefined() {
    Err(JsValue::from(
      crate::Error::TypeError {
        unexpected_type: "undefined",
        expected_types,
        value: Box::new("".to_string()),
      }
      .to_string(),
    ))
  } else if value.is_object() {
    Err(JsValue::from(
      crate::Error::TypeError {
        unexpected_type: "Object",
        expected_types,
        value: Box::new("".to_string()),
      }
      .to_string(),
    ))
  } else if value.is_function() {
    Err(JsValue::from(
      crate::Error::TypeError {
        unexpected_type: "Function",
        expected_types,
        value: Box::new(crate::string_from_jsvalue(value)),
      }
      .to_string(),
    ))
  } else if value.is_string() {
    match value.as_string() {
      Some(string) => match rust_decimal::Decimal::from_str(string.as_str()) {
        Ok(value) => Ok(value),
        Err(_rd_error) => Err(JsValue::from(
          crate::Error::ConvertFromJsValueError {
            from_type: "String",
            to_type: "rust_decimal::Decimal",
            value: Box::new(string),
          }
          .to_string(),
        )),
      },
      None => Err(JsValue::from(
        crate::Error::ConvertFromJsValueError {
          from_type: "String",
          to_type: "rust_decimal::Decimal",
          value: Box::new(crate::string_from_jsvalue(value)),
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
          to_type: "rust_decimal::Decimal",
          value: Box::new(crate::string_from_jsvalue(value)),
        }
        .to_string(),
      )),
    }
  }
}

#[wasm_bindgen]
pub fn is_valid(value: JsValue) -> bool {
  match try_from_jsvalue(value) {
    Ok(_) => true,
    Err(_) => false,
  }
}

#[wasm_bindgen]
pub fn add(left: JsValue, right: JsValue) -> Result<String, JsValue> {
  Ok((try_from_jsvalue(left)? + try_from_jsvalue(right)?).to_string())
}
#[wasm_bindgen]
pub fn subtract_jsvalue(left: JsValue, right: JsValue) -> Result<String, JsValue> {
  Ok((try_from_jsvalue(left)? - try_from_jsvalue(right)?).to_string())
}
#[wasm_bindgen]
pub fn subtract(left: f64, right: f64) -> Result<f64, JsValue> {
  rust_decimal::Decimal::from_f64(left) - rust_decimal::Decimal::from_f64(n: f64)
}
#[wasm_bindgen]
pub fn divide(left: JsValue, right: JsValue) -> Result<String, JsValue> {
  Ok((try_from_jsvalue(left)? / try_from_jsvalue(right)?).to_string())
}
#[wasm_bindgen]
pub fn multiply(left: JsValue, right: JsValue) -> Result<String, JsValue> {
  Ok((try_from_jsvalue(left)? * try_from_jsvalue(right)?).to_string())
}

#[wasm_bindgen]
pub fn sum_strings_array(decimal_js_arr: Vec<JsValue>) -> Result<String, JsValue> {
  use rust_decimal::prelude::FromPrimitive;
  let mut accumulator = rust_decimal::Decimal::from_i64(0).unwrap();
  for value in decimal_js_arr {
    accumulator = accumulator + try_from_jsvalue(value)?;
  }
  Ok(accumulator.to_string())
  // Ok("hello".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
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
pub mod tests {

  use console_error_panic_hook::set_once;
  use wasm_bindgen::JsValue;
  use wasm_bindgen_test::*;
  use web_sys::console;
  #[wasm_bindgen_test]
  pub fn sum_js_strings_valid_nopanic() {
    use super::sum_strings_array;
    // this has been tested up to 2 to power of 28 elements
    // equivelent to the summation
    // of sum j where j=1 to 2^27 or 9,007,199,321,849,856.
    // At this point, nodejs heap will run out of memory becase
    // we have to allocate so many JSValues.
    // we can safely say that we can sum a total of 268,435,456
    // values at once, but should not do it because of memory and time.
    // this test will take more than 30 seconds to complete
    let capacity: usize = 100_000;
    let mut values: Vec<JsValue> = Vec::with_capacity(capacity);
    for val in 1..capacity {
      values.push(JsValue::from(val.to_string()))
    }
    let result = sum_strings_array(values).unwrap();
    console::log_1(&JsValue::from(result))
  }
  #[wasm_bindgen_test]
  pub fn divide_jsstring() {
    use super::divide;
    assert_eq!(
      divide(JsValue::from_f64(444_444_444.444), JsValue::from_f64(2.0)).unwrap(),
      "222222222.222".to_string()
    )
  }
  #[wasm_bindgen_test]
  pub fn test_is_valid() {
    use super::is_valid;
    console_error_panic_hook::set_once();

    assert_eq!(is_valid(JsValue::from_f64(0.0)), true);
    assert_eq!(is_valid(JsValue::from_str("{}")), false);

    assert_eq!(
      is_valid(JsValue::from_str("9999999999999999999999.99")),
      true
    );
    assert_eq!(is_valid(JsValue::from_str("0.0")), true);
    assert_eq!(is_valid(JsValue::undefined()), false);
    assert_eq!(is_valid(JsValue::null()), false);
  }
}
