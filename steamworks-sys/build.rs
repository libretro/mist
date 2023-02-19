use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-env-changed=STEAMWORKS_SDK");
    let sdk_path =
        PathBuf::from(env::var("STEAMWORKS_SDK").expect("The STEAMWORKS_SDK variable is missing"));
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let target = env::var("TARGET").unwrap();

    match target.as_str() {
        // Linux 64-bit
        "x86_64-unknown-linux-gnu" => {
            fs::copy(
                sdk_path.join("redistributable_bin/linux64/libsteam_api.so"),
                out_path.join("libsteam_api.so"),
            )
            .unwrap();
            println!("cargo:rustc-link-lib=dylib=steam_api");
        }
        // macOS 64-bit
        "x86_64-apple-darwin" => {
            fs::copy(
                sdk_path.join("redistributable_bin/osx/libsteam_api.dylib"),
                out_path.join("libsteam_api.dylib"),
            )
            .unwrap();
            println!("cargo:rustc-link-lib=dylib=steam_api");
        }
        // macOS arm64
        "aarch64-apple-darwin" => {
            fs::copy(
                sdk_path.join("redistributable_bin/osx/libsteam_api.dylib"),
                out_path.join("libsteam_api.dylib"),
            )
            .unwrap();
            println!("cargo:rustc-link-lib=dylib=steam_api");
        }
        // Windows 64-bit
        "x86_64-pc-windows-gnu" | "x86_64-pc-windows-msvc" => {
            fs::copy(
                sdk_path.join("redistributable_bin/win64/steam_api64.dll"),
                out_path.join("steam_api64.dll"),
            )
            .unwrap();
            fs::copy(
                sdk_path.join("redistributable_bin/win64/steam_api64.lib"),
                out_path.join("steam_api64.lib"),
            )
            .unwrap();
            println!("cargo:rustc-link-lib=dylib=steam_api64");
        }
        other => panic!("Unsupported target: {}", other),
    };

    let mut bindings_builder = bindgen::Builder::default()
        .header(
            sdk_path
                .join("public/steam/steam_api_flat.h")
                .to_string_lossy(),
        )
        .clang_arg(format!(
            "-I{}",
            Path::new(&sdk_path).join("public").to_string_lossy()
        ))
        .clang_args(&["-xc++", "-std=c++11"]);

    #[cfg(unix)]
    {
        if target == "x86_64-pc-windows-gnu" {
            bindings_builder = bindings_builder.clang_arg("-I/usr/x86_64-w64-mingw32/include/");
        }
    }

    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    #[cfg(feature = "link")]
    println!("cargo:rustc-link-search={}", out_path.to_string_lossy());
}
