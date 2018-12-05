extern crate cdrs;

use cdrs::types::value::{Value, Bytes};

pub trait EnumIntoCDRSValue {
    fn enum_into_cdrs_macro_derive(self) -> Value;
}

impl<T: Into<Bytes>> EnumIntoCDRSValue for T {
    fn enum_into_cdrs_macro_derive(self) -> Value {
        let bytes: Bytes = self.into();
        bytes.into()
    }
}