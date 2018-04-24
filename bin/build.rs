extern crate protoc_grpcio;

fn main() {
    protoc_grpcio::compile_grpc_protos(
        &[
            "indradb/autogen/edges.proto",
            "indradb/autogen/metadata.proto",
            "indradb/autogen/queries.proto",
            "indradb/autogen/request.proto",
            "indradb/autogen/response.proto",
            "indradb/autogen/service.proto",
            "indradb/autogen/vertices.proto",
        ],
        &["./proto"],
        "src/common/autogen"
    ).expect("failed to compile gRPC definitions");
}
