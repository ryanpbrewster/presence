use std::sync::{Arc, Mutex};

use futures::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Clone, Debug, Default)]
pub struct BroadcastLayer {
    listeners: Arc<Mutex<Vec<UnboundedSender<i32>>>>,
}

impl BroadcastLayer {
    pub fn register(&self) -> UnboundedReceiver<i32> {
        let (sender, receiver) = futures::sync::mpsc::unbounded();
        self.listeners.lock().unwrap().push(sender);
        receiver
    }

    pub fn broadcast(&self) {
        let mut buf = self.listeners.lock().unwrap();
        let n = buf.len();
        buf.retain(|sender| sender.unbounded_send(n as i32).is_ok());
    }
}
