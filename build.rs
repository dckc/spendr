fn main() -> Result<(), std::io::Error> {
    // Build rchain protos
    tower_grpc_build::Config::new()
        .enable_client(true)
        .build(
            &[
                "proto/DeployService.proto",
                // "proto/CasperMessage.proto",
                // "proto/ProposeService.proto",
                // "proto/RhoTypes.proto",
                // "proto/Either.proto",
            ],
            &["proto/"],
        )?;

    println!("cargo:rerun-if-changed=proto/CasperMessage.proto");

    Ok(())
}
