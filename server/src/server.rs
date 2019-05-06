use futures::future;
use futures::Stream;
use std::ops::Bound;
use tower_grpc::{Code, Request, Response, Status};

use crate::proto::server;
use crate::proto::{HelloRequest, HelloResponse, PutRequest, PutResponse, ScanRequest, ScanResponse};
use crate::storage::StorageLayer;

#[derive(Clone, Debug, Default)]
pub struct ServerImpl<T: StorageLayer> {
    storage: T,
}

impl<T: StorageLayer> ServerImpl<T> {
    pub fn new(storage: T) -> Self {
        ServerImpl { storage }
    }
    pub fn into_service(self) -> server::PresenceServer<Self> {
        server::PresenceServer::new(self)
    }
}

impl<T: StorageLayer> server::Presence for ServerImpl<T> {
    type SayHelloFuture = future::FutureResult<Response<HelloResponse>, Status>;
    type PutFuture = future::FutureResult<Response<PutResponse>, Status>;
    type ScanStream = Box<Stream<Item = ScanResponse, Error = Status> + Send>;
    type ScanFuture = future::FutureResult<Response<Self::ScanStream>, Status>;

    fn say_hello(&mut self, request: Request<HelloRequest>) -> Self::SayHelloFuture {
        println!("HelloRequest = {:?}", request);

        let response = Response::new(HelloResponse {
            message: "Zomg, it works!".to_string(),
        });

        future::ok(response)
    }

    fn put(&mut self, request: Request<PutRequest>) -> Self::PutFuture {
        println!("PutRequest = {:?}", request);
        let result = self.storage.put(
            request.get_ref().key.clone(),
            request.get_ref().value.clone(),
        );
        match result {
            Ok(_) => future::ok(Response::new(PutResponse {})),
            Err(err) => future::err(err.into()),
        }
    }

    fn scan(&mut self, request: Request<ScanRequest>) -> Self::ScanFuture {
        println!("ScanRequest = {:?}", request);
        let start = to_bound(request.get_ref().start.to_owned());
        let end = to_bound(request.get_ref().end.to_owned());
        let receiver = self.storage.scan(start, end);
        future::ok(Response::new(Box::new(
            receiver
                .map(|(k, v)| ScanResponse { key: k, value: v })
                .map_err(|_| Status::new(Code::Unknown, "scan failed".to_owned())),
        )))
    }
}

fn to_bound(key: String) -> Bound<String> {
    if key.is_empty() {
        Bound::Unbounded
    } else {
        Bound::Included(key)
    }
}
