use std::env;
use std::path::PathBuf;

fn main() {
    if let Some(nvtt_path) = std::env::var_os("NVTT_PATH") {
        let nvtt_path = PathBuf::from(nvtt_path);
        println!(
            "cargo:rustc-link-search={}",
            nvtt_path.to_str().expect("Invalid NVTT_PATH")
        );
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                println!("cargo:rustc-link-search={}", nvtt_path.join(r"lib/x64-v142").to_str().expect("Invalid NVTT_PATH"));
                println!("cargo:rustc-link-search={}", nvtt_path.join(r"include/nvtt").to_str().expect("Invalid NVTT_PATH"));
            }
        }
    } else {
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                unsafe {
                    let path_pw = windows::Win32::UI::Shell::SHGetKnownFolderPath(
                            &windows::Win32::UI::Shell::FOLDERID_ProgramFiles,
                            0,
                            windows::Win32::Foundation::HANDLE(0)
                        )
                        .expect("Failed to find Program Files");
                    let pf = PathBuf::from(path_pw.to_string().expect("Failed to make Program Files path into String"));
                    let nvtt_path = pf.join(r"NVIDIA Corporation/NVIDIA Texture Tools");
                    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("%OUT_DIR% not set"));
                    std::fs::copy(
                        nvtt_path.join(r"nvtt30106.dll"),
                        out_dir.join(r"nvtt30106.dll"),
                    )
                    .expect("unable to copy nvtt30106.dll to %OUT_DIR%");
                }

                println!("cargo:rustc-link-search={}", nvtt_path);
                println!("cargo:rustc-link-search={}", nvtt_path.join(r"lib/x64-v142"));
                println!("cargo:rustc-link-search={}", nvtt_path.join(r"include/nvtt"));
            }
        }
    }

    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            println!("cargo:rustc-link-lib=nvtt30106");
        } else if #[cfg(unix)] {
            println!("cargo:rustc-link-lib=nvtt");
            println!("cargo:rustc-link-lib=cudart");
        }
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Enums
        .rustified_enum("NvttBoolean")
        .rustified_enum("NvttValueType")
        .rustified_enum("NvttChannelOrder")
        .rustified_enum("NvttFormat")
        .rustified_enum("NvttPixelType")
        .rustified_enum("NvttQuality")
        .rustified_enum("NvttWrapMode")
        .rustified_enum("NvttTextureType")
        .rustified_enum("NvttInputFormat")
        .rustified_enum("NvttMipmapFilter")
        .rustified_enum("NvttResizeFilter")
        .rustified_enum("NvttRoundMode")
        .rustified_enum("NvttAlphaMode")
        .rustified_enum("NvttError")
        .rustified_enum("NvttContainer")
        .rustified_enum("NvttNormalTransform")
        .rustified_enum("NvttToneMapper")
        .rustified_enum("NvttCubeLayout")
        .rustified_enum("EdgeFixup")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
