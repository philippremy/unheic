fn main() {
    
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    if target_os == "windows" {
        println!("cargo:warning={}", "Defaulting to a full static build on Windows!");
        println!("cargo:rustc-link-lib=static=c++");
        println!("cargo:rustc-link-lib=static=pthread");
        println!("cargo:rustc-link-arg={}", "-static");
        println!("cargo:rustc-link-arg={}", "-static-libstdc++");
        println!("cargo:rustc-link-arg={}", "-static-libgcc");
    } else {
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=pthread");
    }
    
}
