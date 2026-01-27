fn main() {
    
    println!("cargo:rustc-link-lib=static=c++");
    println!("cargo:rustc-link-lib=static=pthread");
    println!("cargo:rustc-link-arg={}", "-static");
    println!("cargo:rustc-link-arg={}", "-static-libstdc++");
    println!("cargo:rustc-link-arg={}", "-static-libgcc");
    
}
