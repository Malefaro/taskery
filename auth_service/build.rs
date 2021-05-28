fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/auth.proto")?;
    tonic_build::configure()
        .out_dir("src/proto") // you can change the generated code's location
        .compile(
            &["./protos/auth.proto"],
            &["./protos"], // specify the root location to search proto dependencies
        )
        .unwrap();
    Ok(())
}
