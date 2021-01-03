use rust_decimal;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug)]
pub enum Error {
    Unknown,
}
impl std::fmt::Display for crate::Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "enum Error {{}}")
    }
}

impl std::error::Error for crate::Error {}
impl std::convert::From<rust_decimal::Error> for crate::Error {
    fn from(_error: rust_decimal::Error) -> crate::Error {
        Error::Unknown
    }
}

pub fn rust_string_to_decimal(string: String) -> Result<rust_decimal::Decimal, crate::Error> {
    use std::str::FromStr;
    Ok(rust_decimal::Decimal::from_str(string.as_str())?)
}

pub fn sum_rust_strings(arr: Vec<String>) -> Result<String, crate::Error> {
    // try to convert the vector of string to decimal
    Ok(arr
        .into_iter()
        .map(|val: String| rust_string_to_decimal(val).unwrap())
        .sum::<rust_decimal::Decimal>()
        .to_string())
    // Ok(String::from(""))
}

#[no_mangle]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
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
            Err(JsValue::from(format!("{}", Error::Unknown)))
        }
    }
    // Ok("hello".to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn sum_rust_strings_valid_nopanic() {
        use crate::sum_rust_strings;
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
        use crate::sum_strings_array;
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
