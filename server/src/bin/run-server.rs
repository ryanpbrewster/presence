use futures::{Future, Stream};
use tokio::executor::DefaultExecutor;
use tokio::net::TcpListener;
use tower_h2::Server;

use presence_server::broadcast::BroadcastLayer;
use presence_server::server::ServerImpl;
use std::time::Duration;

pub fn main() {
    let broadcast = BroadcastLayer::default();
    let kvstore_service = ServerImpl {
        broadcast: broadcast.clone(),
    };
    let mut server = Server::new(
        kvstore_service.into_service(),
        Default::default(),
        DefaultExecutor::current(),
    );

    let addr = "[::1]:50051".parse().unwrap();
    let bind = TcpListener::bind(&addr).expect("bind");

    println!("listening on {}...", addr);
    let serve = bind
        .incoming()
        .for_each(move |sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = server.serve(sock);
            tokio::spawn(serve.map_err(|e| eprintln!("h2 error: {:?}", e)));

            Ok(())
        })
        .map_err(|e| eprintln!("accept error: {}", e));

    let timer = tokio::timer::Interval::new_interval(Duration::from_millis(100))
        .for_each(move |_| futures::future::ok(broadcast.broadcast()))
        .map_err(|e| eprintln!("accept error: {}", e));

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn(serve);
    rt.spawn(timer);
    rt.shutdown_on_idle().wait().unwrap();
}
