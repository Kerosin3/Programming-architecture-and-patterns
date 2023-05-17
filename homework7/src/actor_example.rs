use libsystem::{Actor, System};
use std::sync::mpsc::Sender;
use std::thread;
//use std::time::Duration;

fn main() {
    let mut system = System::default();
    let recv_rez = system.init_result_channel();
    let mut store = CommandStore::new();
    store.push(Commands::Command1);
    store.push(Commands::Command1);
    store.push(Commands::Command1);
    store.push(Commands::Command1);
    store.push(Commands::Command2);
    store.push(Commands::CommandHardStop);
    store.push(Commands::Command3);
    store.push(Commands::Command3);
    store.push(Commands::CommandSoftStop);

    let runner_sender = RunnerActor::new();
    let runner_tx = system.run(runner_sender);

    let arbiter_actor = InputActor::new(runner_tx, store);
    let input_tx = system.run(arbiter_actor);
    input_tx.send(()).unwrap();
    drop(system);
    while let Ok(rez) = recv_rez.recv() {
        println!("returned: {:?}", rez);
    }
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
    CommandHardStop,
    CommandSoftStop,
}

// **** INPUT ****
// Этот актор будет кидать команды на исполнение актору
struct InputActor {
    cmd_store: CommandStore,
    snd: Sender<Commands>,
}

impl InputActor {
    pub fn new(ping_tx: Sender<Commands>, cmd_s: CommandStore) -> Self {
        Self {
            snd: ping_tx,
            cmd_store: cmd_s,
        }
    }
}

type InputMessage = ();

impl Actor for InputActor {
    type Message = InputMessage;

    fn process_message(self, _: Self::Message) -> Option<Self> {
        let mut cmd_store_t = self.cmd_store.cmd;
        cmd_store_t.reverse();
        // stops when all command are sended
        loop {
            while let Some(cmd) = cmd_store_t.pop() {
                println!("reading command!");
                self.snd.send(cmd).ok()?;
                println!("command has been sent to an actor");
            }
            return None;
        }
    }
}

//  Command Runner

struct RunnerActor(usize);

impl RunnerActor {
    pub fn new() -> Self {
        Self(0)
    }
}

impl Actor for RunnerActor {
    type Message = Commands;

    fn process_message(self, msg: Self::Message) -> Option<Self> {
        let rmsg = match msg {
            Commands::CommandHardStop => {
                println!("Hardstopping!");
                return None;
            }
            // non sense
            Commands::CommandSoftStop => {
                println!("Softstopping!");
                return Some(self);
            }

            _ => msg,
        };
        println!(
            "[{:?}], Running command: {:?}",
            thread::current().id(),
            rmsg
        );
        Some(self)
    }
}
