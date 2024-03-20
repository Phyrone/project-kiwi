const PROTO_DIR: &str = "../../proto";

fn main() {
    println!("cargo:rerun-if-changed={}/*.proto", PROTO_DIR);

    let dir = std::fs::read_dir(PROTO_DIR).expect("Failed to read proto directory");
    let paths = dir
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect::<Vec<_>>();

    let proto_files = paths
        .iter()
        .map(|p| p.to_str().unwrap())
        .collect::<Vec<_>>();

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .build_transport(true)
        .compile_well_known_types(false)
        .use_arc_self(false)
        .include_file("_all.rs")
        .out_dir("src")
        .compile(
            /* &["../../proto/snowflake.proto"]*/ &proto_files,
            &[PROTO_DIR],
        )
        .expect("Failed to compile gRPC definitions!");
}
