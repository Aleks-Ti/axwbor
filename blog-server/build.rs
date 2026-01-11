fn main() -> std::io::Result<()> {

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["proto/blog.proto"], &["proto"])?;
    Ok(())
}
