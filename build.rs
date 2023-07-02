use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let out_dir = env::var("OUT_DIR")
        .map_err(|_| "Environmental variable `OUT_DIR` not defined.".to_string())?;

    let fsmbdd_lace_path = PathBuf::from("fsmbdd_lace");
    let mut cfg = Config::new(fsmbdd_lace_path);

    let is_debug = env::var_os("DEBUG").unwrap_or_else(|| "true".into());
    if is_debug == "true" {
        cfg.cflag("-Werror");
    }
    cfg.build();

    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(out_dir).join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=fsmbdd_lace");

    Ok(())
}
