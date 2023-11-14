fn main() -> Result<(), Box<dyn std::error::Error>> {

    tonic_build::compile_protos("protobuf_definitions/key_value_protos.proto")?;

    Ok(())
}