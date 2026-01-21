fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=proto/blog.proto");
    println!("cargo:rerun-if-changed=proto/auth.proto");

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_client(true)
        .build_server(false)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["proto/blog.proto", "proto/auth.proto"], &["proto"])?;
    Ok(())
}
