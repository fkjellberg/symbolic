use symbolic::common::{Name};
use symbolic::demangle::{Demangle, DemangleOptions};

fn main() {
    let name = Name::from("_$s10CommandKit0A7BuilderC6filteryACyxq_GXDSbxcFTf4nn_g");
    println!("Symbol: {}", name.to_string());
    println!("Language: {}", name.language());
    println!("Detect language: {}", name.detect_language());
    let result = name.try_demangle(DemangleOptions::complete());
    println!("Result: {}", result.to_string());
}
