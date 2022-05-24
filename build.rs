/// Generate the stubs in `src/example.rs` based on proto/example.proto.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = &["proto/example.proto"];
    let dirs = &["."];

    tonic_build::configure()
        .build_client(true)
        .out_dir("src")
        .compile(files, dirs)
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

    for file in files {
        println!("cargo:rerun-if-changed={}", file);
    }
    Ok(())
}
