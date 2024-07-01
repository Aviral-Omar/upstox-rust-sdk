use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .pure()
        .cargo_out_dir("protos")
        .include("src")
        .input("src/protos/market_data_feed.proto")
        .run_from_script();
}
