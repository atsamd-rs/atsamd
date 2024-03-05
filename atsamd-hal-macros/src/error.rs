use proc_macro::TokenStream;

pub enum Error {
    UnknownPeripheral { peripheral: String },
    GenericParseError { expected: String },
}

impl Error {
    pub fn to_compile_error(&self, name: &str) -> TokenStream {
        let s = match self {
            Error::UnknownPeripheral { peripheral } => {
                format!("{name}: unknown peripheral {peripheral:?}")
            }
            Error::GenericParseError { expected } => {
                format!("{name}: could not parse the argument, expected {expected}")
            }
        };
        let s = format!("compile_error!({s:?});");
        s.parse().unwrap()
    }
}
