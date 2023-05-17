use libsystem::{Actor, System};
use std::io::stdin;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

fn main() {
    let mut system = System::default();

    let mut store = CommandStore::new();
    store.push(Commands::Command1);
    store.push(Commands::Command1);
    store.push(Commands::Command1);

    let ping = RunnerActor::new(String::from("Bob"));
    let ping_tx = system.run(ping);
    // accept input and send msg to RunnerActor
    let input = InputActor::new(ping_tx);
    let input_tx = system.run(input);

    input_tx.send(()).unwrap();
}
#[derive(Debug)]
struct CommandStore {
    cmd: Vec<Commands>,
}
impl CommandStore {
    fn new() -> Self {
        Self { cmd: vec![] }
    }
    fn push(&mut self, cmd: Commands) {
        self.cmd.push(cmd)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
enum Commands {
    Command1,
    Command2,
    Command3,
}

// **** INPUT ****

struct InputActor(Sender<PingMessage>);

impl InputActor {
    pub fn new(ping_tx: Sender<PingMessage>) -> Self {
        Self(ping_tx)
    }
}

type InputMessage = ();

impl Actor for InputActor {
    type Message = InputMessage;

    fn process_message(self, _: Self::Message) -> Option<Self> {
        loop {
            println!();
            println!("Enter message for ping-pong:");
            let mut msg = String::new();
            stdin().read_line(&mut msg).ok()?;
            let msg = msg.trim().to_string();

            if msg == "exit" {
                return None;
            }

            self.0.send(PingMessage::new(msg)).ok()?;
            thread::sleep(Duration::from_secs(1));
        }
    }
}

// **** PING ****

struct RunnerActor(String);

impl RunnerActor {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

struct PingMessage(String);

impl PingMessage {
    pub fn new(text: String) -> Self {
        Self(text)
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Actor for RunnerActor {
    type Message = PingMessage;

    fn process_message(self, msg: Self::Message) -> Option<Self> {
        let str = msg.into_string();
        println!("ping with message: {}", str);
        //         self.1.send(PongMessage::new(str)).ok()?;
        Some(self)
    }
}

// **** PONG ****

struct PongActor(String);

impl PongActor {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

struct PongMessage(String);

impl PongMessage {
    pub fn new(text: String) -> Self {
        Self(text)
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Actor for PongActor {
    type Message = PongMessage;

    fn process_message(self, msg: Self::Message) -> Option<Self> {
        let str = msg.into_string();
        println!("pong with message: {}", str);
        Some(self)
    }
}
