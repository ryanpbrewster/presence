use futures::future;
use futures::Stream;
use tower_grpc::{Code, Request, Response, Status};

use crate::broadcast::BroadcastLayer;
use crate::proto::server;
use crate::proto::{HelloRequest, HelloResponse, ParkRequest, ParkResponse};

#[derive(Clone, Debug, Default)]
pub struct ServerImpl {
    pub broadcast: BroadcastLayer,
}

impl ServerImpl {
    pub fn into_service(self) -> server::PresenceServer<Self> {
        server::PresenceServer::new(self)
    }
}

impl server::Presence for ServerImpl {
    type SayHelloFuture = future::FutureResult<Response<HelloResponse>, Status>;
    type ParkStream = Box<Stream<Item = ParkResponse, Error = Status> + Send>;
    type ParkFuture = future::FutureResult<Response<Self::ParkStream>, Status>;

    fn say_hello(&mut self, request: Request<HelloRequest>) -> Self::SayHelloFuture {
        future::ok(Response::new(HelloResponse {
            message: format!("Hello, {}!", request.get_ref().name),
        }))
    }

    fn park(&mut self, _request: Request<ParkRequest>) -> Self::ParkFuture {
        future::ok(Response::new(Box::new(
            self.broadcast
                .register()
                .map(|n| ParkResponse { num_connections: n })
                .map_err(|_| Status::new(Code::Unknown, "park failed".to_owned())),
        )))
    }
}
