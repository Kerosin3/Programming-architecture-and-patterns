use ddi::*;
use figment::{
    providers::{Env, Format, Json, Toml},
    Figment, Source,
};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use tokio::{task, time};
mod implement;
use implement::*;
use templates::data_exchange::recv_interface::RecvDataInterface;
use templates::data_exchange::OperationObj;
mod processor;
use processor::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("src/agent_processor/conf/conf.toml");
    let config: Config = Figment::new()
        .merge(Toml::file(config_path))
        .merge(Env::prefixed("CARGO_"))
        .extract()?;
    //     dbg!(config);
    let subscribes = config.agent_settings.subscribes.to_owned();
    let mut mqttoptions = MqttOptions::new(
        config.agent_settings.name,
        config.agent_settings.host,
        config.agent_settings.port as u16,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(60));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(subscribes.first().unwrap().clone(), QoS::AtMostOnce)
        .await
        .unwrap();
    let mut jk = 0_usize;
    loop {
        let notification = eventloop.poll().await.unwrap();
        match notification {
            Event::Incoming(Packet::Publish(p)) => {
                let recv_data = RecvWrapper::<usize>::deserialize_data(p);
                match recv_data {
                    Ok(d) => {
                        //black magic
                        let mut services = ServiceCollection::new();
                        services.service(d.get_operation()); // get operation type
                        let Ok(arg_0) = d.get_args(0) else { // take zero arg
                            println!("error getting arg!");
                            continue;
                        };
                        services.service(arg_0.0); // register number
                                                   // setup factory Operation and arg (number)
                        services.service_factory(|cmd: &OperationObj, uval: &usize| {
                            Ok(ServerCommand {
                                cmd: *cmd,
                                arg: *uval,
                            })
                        });
                        let provider = services.provider();
                        let Ok(cmd_to_server) = provider.get::<ServerCommand>() else {
                            println!("error while resolvig command!");
                            continue;
                        };
                        //resolve command and inject into server command
                        let game_server_cmd = GameServerCommands::command_parser(cmd_to_server.cmd);
                        println!("---------->{:?}", game_server_cmd);
                        jk += 1;
                        println!(
                            "command {:?},args: {:?}",
                            d.get_operation(),
                            d.get_args(0).unwrap()
                        );
                    }
                    Err(e) => {
                        println!("error while deserializing! err: {}", e);
                    }
                }
            }
            Event::Outgoing(_) => {
                println!("Outgoing");
            }
            _ => {
                println!("Other");
            }
        }
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct AgentSettings {
    name: String,
    version: String,
    subscribes: Vec<String>,
    host: String,
    port: isize,
}
#[derive(Deserialize, Debug)]
struct Config {
    agent_settings: AgentSettings,
}
