use std::{io::Write, path::PathBuf};

fn main() {
    // If we have *-gnullvm as target, we need to overwrite this with *-gnu
    let target = std::env::var("TARGET").unwrap();
    let clang_target: String;
    if target.ends_with("llvm") {
        clang_target = target.strip_suffix("llvm").unwrap().to_string();
    } else {
        clang_target = target;
    }

    // Bindgen
    #[rustfmt::skip]
    let unheic_bridge_bindings = bindgen::builder()

        /* API wrapper header */
        .header(format!(
            "{}/src/cxx/unheic_bridge/include/ui_api.hxx",
            env!("CARGO_MANIFEST_DIR")
        ))

        /* Dependent headers need to be found */
        .clang_args([
            "-DUNHEIC_BINDGEN".into(),
            format!(
                "-I{}",
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("src")
                    .join("cxx")
                    .join("unheic_bridge")
                    .join("include")
                    .display()
            ),
            format!("--target={clang_target}"),
            "-fparse-all-comments".into(),
            "-fretain-comments-from-system-headers".into(),
            "-std=c++20".into(),
        ])

        /* Allowlisting single types */
        .allowlist_type("UnHEICBridgeResult")
        .allowlist_type("UnHEICCXXImageHandle")

        /* Allowlisting single functions */
        .allowlist_function("heic_get_image_dimensions")
        .allowlist_function("heic_read_input_image")
        .allowlist_function("heic_get_image_data_rgba")
        .allowlist_function("heic_get_bytes_per_pixel")

        /* C/C++ types that are needed */
        .allowlist_type("char")
        .allowlist_type("size_t")
        .allowlist_type("uint8_t")

        /* Types which are opaque to Rust */
        .opaque_type("UnHEICCXXImageHandle")

        /* Enums which should be turned into Rust enums */
        .rustified_enum("UnHEICBridgeResult")

        /* General settings */
        .size_t_is_usize(true)
        .generate_comments(true)
        .merge_extern_blocks(true)

        /* All the handles assume to handle unique data, no copy! */
        .derive_copy(false) // DO NOT DERIVE COPY!

        /* Do it. */
        .generate()
        .unwrap();

    let mut bindings_file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join("rust")
                .join("bridge")
                .join("generated.rs"),
        )
        .unwrap();

    bindings_file
        .write_all("#![allow(unused)]\n#![allow(unsafe_op_in_unsafe_fn)]\n\n".as_bytes())
        .unwrap();
    unheic_bridge_bindings
        .write(Box::new(bindings_file))
        .unwrap();

    build_info_build::build_script().build();
}
