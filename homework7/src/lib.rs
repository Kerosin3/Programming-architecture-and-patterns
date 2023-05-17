use std::default::Default;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

pub trait Actor: Sized + Send + 'static {
    type Message: Send + 'static;
    fn process_message(self, msg: Self::Message) -> Option<Self>;
}

#[derive(Debug)]
pub struct System {
    handles: Vec<JoinHandle<usize>>,
    pub tx_rez_enpoint: Option<Arc<Sender<usize>>>,
}

impl Default for System {
    fn default() -> Self {
        Self {
            handles: Default::default(),
            tx_rez_enpoint: None,
        }
    }
}
// accept actor and execute process_message method
// returns transceive endpoint
impl System {
    pub fn run<A: Actor>(&mut self, actor: A) -> Sender<A::Message> {
        let (tx, rx) = mpsc::channel();
        // create channel and spawn new thread
        let jh = thread::spawn(move || {
            let mut i = 0;
            let mut actor = actor;
            // continue receive messages
            while let Ok(msg) = rx.recv() {
                i += 1;
                actor = match actor.process_message(msg) {
                    Some(a) => a,
                    None => break,
                };
            }
            i
        });
        self.handles.push(jh);
        tx // return transceive endpoint
    }
    pub fn init_result_channel(&mut self) -> Receiver<usize> {
        let (tx, rx) = mpsc::channel();
        self.tx_rez_enpoint = Some(Arc::new(tx));
        rx
    }
}
// cleanup
impl Drop for System {
    /// Waits when all actors finish their work.
    fn drop(&mut self) {
        let handles = std::mem::take(&mut self.handles);
        let mut results = vec![];
        for jh in handles {
            let rez = jh.join().unwrap();
            results.push(rez);
        }
        let endpoint = self.tx_rez_enpoint.as_mut().unwrap();
        for _i in results.iter() {
            endpoint.send(*_i).unwrap();
        }
    }
}
