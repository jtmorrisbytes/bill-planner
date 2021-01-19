use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub enum Error {
  Unknown(&'static str),
  #[cfg(target_arch = "wasm32")]
  TypeError {
    unexpected_type: &'static str,
    expected_types: &'static str,
    value: Box<String>,
  },
  #[cfg(target_arch = "wasm32")]
  ConvertFromJsValueError {
    from_type: &'static str,
    to_type: &'static str,
    value: Box<String>,
  },
  ConvertFromF64Error {
    value: Box<f64>,
  },
  ConvertFromStringError {
    message: Box<String>,
  },
}

impl Default for Error {
  fn default() -> Self {
    Error::Unknown("default handler")
  }
}

impl std::fmt::Display for crate::Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *&self {
      #[cfg(not(target_arch = "wasm32"))]
      crate::Error::Unknown(message) => write!(f, "Unknown Error: {}", message),
      #[cfg(target_arch = "wasm32")]
      crate::Error::Unknown(message) => write!(f, "UnknownError: {}", message),
      #[cfg(target_arch = "wasm32")]
      crate::Error::TypeError {
        unexpected_type,
        expected_types,
        value,
      } => {
        write!(
          f,
          "TypeError: unexpected type {}, expected type(s) {}. {:#?} ",
          unexpected_type, expected_types, value
        )
      }
      #[cfg(target_arch = "wasm32")]
      crate::Error::ConvertFromJsValueError {
        from_type,
        to_type,
        value,
      } => write!(
        f,
        "ConvertFromJsValueError: Failed to convert type '{}' to type '{}'. Got value {:#?} ",
        from_type, to_type, value
      ),
      crate::Error::ConvertFromF64Error { value } => write!(
        f,
        "ConvertFromF64Error: Failed to convert  {:#?} to a Decimal",
        *value
      ),
      crate::Error::ConvertFromStringError { message } => {
        write!(f, "ConvertFromStringError:'{}'", message)
      }
    }
  }
}

impl std::error::Error for crate::Error {}
impl std::convert::From<rust_decimal::Error> for crate::Error {
  fn from(_error: rust_decimal::Error) -> crate::Error {
    Error::Unknown("")
  }
}

#[cfg(target_arch = "wasm32")]
impl From<crate::Error> for JsValue {
  fn from(error: crate::Error) -> JsValue {
    JsValue::from_serde(&error).unwrap()
  }
}
