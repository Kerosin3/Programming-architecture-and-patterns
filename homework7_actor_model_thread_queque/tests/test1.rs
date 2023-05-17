#[cfg(test)]
#[allow(unused_imports)]
pub mod test {
    use libsystem::*;
    use std::sync::mpsc::Sender;
    use std::thread;
    #[test]
    fn test_hardstop() {
        // push 5 command and test that 5 commands were executed
        let mut system = System::default();
        // used to recv data from join handlers
        let recv_rez = system.init_result_channel();
        // command storage
        let mut store = CommandStore::new();

        store.push(Commands::Command1); // 1
        store.push(Commands::Command1); // 2
        store.push(Commands::Command1); // 3
        store.push(Commands::Command1); // 4
        store.push(Commands::Command2); // 5
        store.push(Commands::CommandHardStop); // 6 HARDSTOP
        store.push(Commands::Command3); // 7
        store.push(Commands::Command3); //8

        let runner_sender = RunnerActor::new();
        let runner_tx = system.run(runner_sender);
        let arbiter_actor = InputActor::new(runner_tx, store);
        let input_tx = system.run(arbiter_actor);
        input_tx.send(()).unwrap(); // to ignite
        drop(system); // drop all now
                      // get results
        let mut rezz = vec![];
        while let Ok(rez) = recv_rez.recv() {
            rezz.push(rez);
        }
        assert_eq!(rezz.pop().unwrap(), 1);
        assert_eq!(rezz.pop().unwrap(), 6); // 6 command total (+1 to stop)
    }
    #[test]
    fn test_softstop() {
        // push 10 command and test that 10 commands were executed after Soft Interrupt
        let mut system = System::default();
        // used to recv data from join handlers
        let recv_rez = system.init_result_channel();
        // command storage
        let mut store = CommandStore::new();

        store.push(Commands::Command1); // 1
        store.push(Commands::Command1); // 2
        store.push(Commands::Command1); // 3
        store.push(Commands::Command1); // 4
        store.push(Commands::Command2); // 5
        store.push(Commands::CommandSoftStop); // 6 SOFTSTOP
        store.push(Commands::Command3); // 7
        store.push(Commands::Command3); //8
        store.push(Commands::Command3); //9
        store.push(Commands::Command3); //10

        let runner_sender = RunnerActor::new();
        let runner_tx = system.run(runner_sender);
        let arbiter_actor = InputActor::new(runner_tx, store);
        let input_tx = system.run(arbiter_actor);
        input_tx.send(()).unwrap(); // to ignite
        drop(system); // drop all now
                      // get results
        let mut rezz = vec![];
        while let Ok(rez) = recv_rez.recv() {
            rezz.push(rez);
        }
        // VERIVY THAT ALL COMMANDS WERE DONE
        assert_eq!(rezz.pop().unwrap(), 1);
        assert_eq!(rezz.pop().unwrap(), 10); //  10 command total
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
}
