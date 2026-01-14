fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=proto/blog.proto");
    println!("cargo:rerun-if-changed=proto/auth.proto");
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["proto/blog.proto", "proto/auth.proto"], &["proto"])?;
    Ok(())
}
