/// 🏗️ Example: Bounded Actor Pattern
/// Prevents system collapse via backpressure.
use tokio::sync::mpsc;

pub struct SovereignActor {
    receiver: mpsc::Receiver<Command>,
}

impl SovereignActor {
    pub async fn run(&mut self) {
        while let Some(cmd) = self.receiver.recv().await {
            // Process command
        }
    }
}
