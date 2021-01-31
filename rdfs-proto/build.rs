use anyhow::Context;
use cargo_emit::rerun_if_changed;
use protobuf_codegen_pure::Customize;

fn main() -> Result<(), anyhow::Error> {
    let walker = walkdir::WalkDir::new("proto").into_iter();
    let mut files = Vec::new();
    for i in walker.filter_map(Result::ok) {
        rerun_if_changed!(i.path().to_string_lossy());
        if let Some(ext) = i.path().extension() {
            if ext == "proto" {
                files.push(i.path().to_owned());
            }
        }
    }
    protobuf_codegen_pure::Codegen::new()
        .includes(&["proto"])
        .inputs(&files)
        /*
        .inputs(&[
            "proto/hdfs.proto",
            "proto/erasurecoding.proto",
            "proto/HdfsServer.proto",
            "proto/DatanodeProtocol.proto",
        ])*/
        .customize(Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir("src/gen")
        .run()
        .context("Failed to generate code from protobuf (in build.rs)")
}
