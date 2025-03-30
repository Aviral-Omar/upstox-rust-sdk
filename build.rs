use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .pure()
        .cargo_out_dir("protos")
        .include("src")
        .inputs(vec![
            "src/protos/market_data_feed.proto",
            "src/protos/market_data_feed_v3.proto",
        ])
        .run_from_script();
}
