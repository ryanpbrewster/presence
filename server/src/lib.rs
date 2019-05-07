pub mod proto {
    use prost_derive::Message;
    include!(concat!(env!("OUT_DIR"), "/presence.rs"));
}

pub mod broadcast;
pub mod server;
