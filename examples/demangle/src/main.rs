use symbolic::common::{Name};
use symbolic::demangle::{Demangle, DemangleOptions};

fn demangle(symbol: &str) {
    let name = Name::from(symbol);
    println!("Symbol  : {}", name.to_string());
    println!("Language: {}", name.detect_language());
    //let result = name.try_demangle(DemangleOptions::complete());
    let result = name.try_demangle(DemangleOptions::name_only());
    println!("Result  : {}", result.to_string());
    println!("");
}

fn main() {
    demangle("_ZN12_GLOBAL__N_15startEv"); // cpp
    demangle("_ZN4base24MessagePumpNSApplication5DoRunEPNS_11MessagePump8DelegateE"); // objcpp
    demangle("?LoadV8Snapshot@V8Initializer@gin@@SAXXZ"); // msvc
    demangle("$S8mangling12curry1ThrowsyyKF"); // Swift
    demangle("_$s10CommandKit0A7BuilderC6filteryACyxq_GXDSbxcFTf4nn_g"); // Swift
}
