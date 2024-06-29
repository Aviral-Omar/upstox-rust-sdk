use {
    protobuf_codegen::Codegen,
    std::{env, ffi::OsString, fs::read_to_string, path::Path},
};

fn main() {
    let out_dir_env: OsString = env::var_os("OUT_DIR").unwrap();
    let out_dir: &Path = Path::new(&out_dir_env);
    Codegen::new()
        .pure()
        .out_dir(out_dir)
        .input("src/protos/market_data_feed.proto")
        .include("src/protos")
        .run_from_script();

    let path = out_dir.join("market_data_feed.rs");
    let code: String = read_to_string(&path).expect("Failed to read generated file");
    println!("{}", code.lines().count());
}
