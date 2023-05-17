use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

pub trait Actor: Sized + Send + 'static {
    type Message: Send + 'static;

    fn process_message(self, msg: Self::Message) -> Option<Self>;
}

#[derive(Debug, Default)]
pub struct System {
    handles: Vec<JoinHandle<()>>,
}
// accept actor and execute process_message method
// returns transceive endpoint
impl System {
    pub fn run<A: Actor>(&mut self, actor: A) -> Sender<A::Message> {
        let (tx, rx) = mpsc::channel();
        // create channel and spawn new thread
        let jh = thread::spawn(move || {
            let mut actor = actor;
            // continue receive messages
            while let Ok(msg) = rx.recv() {
                actor = match actor.process_message(msg) {
                    Some(a) => a,
                    None => break,
                }
            }
        });
        self.handles.push(jh);

        tx // return transceive endpoint
    }
}
// cleanup
impl Drop for System {
    /// Waits when all actors finish their work.
    fn drop(&mut self) {
        let handles = std::mem::take(&mut self.handles);
        for jh in handles {
            jh.join().unwrap();
        }
    }
}
