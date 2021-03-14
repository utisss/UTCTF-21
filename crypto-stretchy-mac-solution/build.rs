use std::{
    error::Error,
    path::Path,
    process::Command,
};

const USE_OPENCL_VAR: &str = "USE_OPENCL";

fn main() -> Result<(), Box<dyn Error>> {
    let c_source = "src/c";
    let out_dir = &std::env::var("OUT_DIR").unwrap();
    let out_c_source = &format!("{}/c", out_dir);

    let use_opencl =
        &std::env::var("USE_OPENCL").unwrap_or_else(|_| "".to_string()) == "y";
    println!(
        "{}",
        &std::env::var("USE_OPENCL").unwrap_or_else(|_| "".to_string())
    );

    Command::new("cp")
        .arg("-r")
        .arg(c_source)
        .arg(out_dir)
        .spawn()?
        .wait()?;

    let mut make = Command::new("make");

    if use_opencl {
        make.env("USE_OPENCL", "y");
    }

    #[cfg(not(debug_assertions))]
    {
        make.env("DEBUG_DEFINITION", "NDEBUG");
    };

    make.current_dir(out_c_source).spawn()?.wait()?;

    println!("cargo:rerun-if-env-changed={}", USE_OPENCL_VAR);
    println!("cargo:rerun-if-changed={}", c_source);
    for entry in Path::new(c_source).read_dir()? {
        println!("cargo:rerun-if-changed={}", entry?.path().to_string_lossy());
    }

    println!("cargo:rustc-link-lib=static=sha224_length_extension_attack");
    println!("cargo:rustc-link-search=native={}", out_c_source);

    println!("cargo:rustc-link-lib=dylib=crypto");
    if use_opencl {
        println!("cargo:rustc-link-lib=dylib=OpenCL");
    }

    Ok(())
}
