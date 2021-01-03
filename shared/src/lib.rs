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
    fn from(error: rust_decimal::Error) -> crate::Error {
        Error::Unknown
    }
}

pub fn rust_string_to_decimal(string: String) -> Result<rust_decimal::Decimal, crate::Error> {
    use std::str::FromStr;
    Ok(rust_decimal::Decimal::from_str(string.as_str())?)
}

pub fn sum_decimal_str(arr: Vec<String>) -> Result<String, crate::Error> {
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
pub fn sum_strings_array(decimalJsArr: Vec<JsValue>) -> Result<String, JsValue> {
    use std::panic;
    use web_sys::console;
    match panic::catch_unwind(|| {
        decimalJsArr
            .into_iter()
            .map(|maybeJsString: JsValue| maybeJsString.as_string().unwrap())
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
    use crate::sum_decimal_str;
    #[test]
    fn it_works() {
        assert_eq!(
            sum_decimal_str(vec![
                String::from("1.1"),
                String::from("2.2"),
                String::from("3.3")
            ])
            .unwrap(),
            String::from("6.6")
        )
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
        let mut values: Vec<JsValue> = Vec::new();
        for val in 1..111 {
            values.push(JsValue::from(format!("{0}.{0}", val.to_string())))
        }
        let result = sum_strings_array(values).unwrap();
        console::log_1(&JsValue::from(result))
    }
}
