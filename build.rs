use anyhow::Context;
use cargo_emit::rerun_if_changed;

fn main() -> Result<(), anyhow::Error> {
    protobuf_codegen_pure::Codegen::new()
        .includes(&["proto"])
        .inputs(&[
            "proto/hdfs.proto",
            "proto/DatanodeProtocol.proto",
        ])
        .run()
        .context("Failed to generate code from protobuf (in build.rs)")
}
