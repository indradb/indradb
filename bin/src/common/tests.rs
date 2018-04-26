// use client_datastore::ClientDatastore;
// use std::sync::Arc;

// const TEST_PORT: u16 = 27616;

// lazy_static! {
//     static ref ENVIRONMENT: Arc<grpcio::Environment> = Arc::new(grpcio::Environment::new(1));
//     static ref SERVER: grpcio::Server = grpc_server::start_server((*ENVIRONMENT).clone(), "127.0.0.1", TEST_PORT);
// }

// full_test_impl!({
//     println!("Server: {:?}", *SERVER);
//     GrpcClientDatastore::new((*ENVIRONMENT).clone(), TEST_PORT)
// });
