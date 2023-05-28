use ddi::*;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
mod implement;
use implement::*;
use templates::data_exchange::recv_interface::RecvDataInterface;
use templates::data_exchange::OperationObj;
mod processor;
use processor::*;
//-------------------------------------------

//-------------------------------------------
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    // read agent config
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("src/agent_processor/conf/conf.toml");
    let config: Config = Figment::new()
        .merge(Toml::file(config_path))
        .merge(Env::prefixed("CARGO_"))
        .extract()?;
    //     dbg!(config);
    // setup mtqq broker
    let subscribes = config.agent_settings.subscribes.to_owned();
    let mut mqttoptions = MqttOptions::new(
        config.agent_settings.name,
        config.agent_settings.host,
        config.agent_settings.port as u16,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(60));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(subscribes.first().unwrap().clone(), QoS::AtLeastOnce)
        .await
        .unwrap();
    // begin eventloop
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
                        let Ok(argz) = d.get_all_args_pairs() else { // Vector of all args 
                            println!("error getting arg!");
                            continue;
                        };
                        //register number
                        services.service(argz);
                        //register agent info
                        services.service(AgentInfo {
                            username: d.get_name().to_owned(),
                            gameid: d.get_gameid(),
                            objectid: d.get_obj_id(),
                        });
                        /* setup factory */
                        services.service_factory(
                            |cmd: &OperationObj, arg: &Vec<(usize, String)>, info: &AgentInfo| {
                                Ok({
                                    AgentCommand {
                                        cmd: *cmd,
                                        arg: arg.clone(),
                                        info: info.clone(),
                                    }
                                })
                            },
                        );
                        //extract injected structure
                        let provider = services.provider();
                        // test
                        let Ok(cmd_to_server) = provider.get::<AgentCommand>() else {
                            println!("error while resolvig command!");
                            continue;
                        };
                        //resolve command and inject into server command
                        let cmd_server_transform: ServerCommand = (*cmd_to_server).clone().into();
                        println!("--------->{:?}", cmd_server_transform);
                        //pass to gameserver
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
