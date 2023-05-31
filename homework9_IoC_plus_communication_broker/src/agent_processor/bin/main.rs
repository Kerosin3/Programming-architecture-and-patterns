#![allow(unreachable_code)]
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
    let mut config: Config = Figment::new()
        .merge(Toml::file(config_path))
        .merge(Env::prefixed("CARGO_"))
        .extract()?;
    //     dbg!(config);
    // setup mtqq broker
    config.agent_settings.subscribes.reverse();
    let agent_player = config.agent_settings.subscribes.pop().unwrap();
    let game_server = config.agent_settings.subscribes.pop().unwrap();
    println!("agent {},server {}", agent_player, game_server);
    //receiver!
    let mut mqttoptions = MqttOptions::new(
        config.agent_settings.name.clone(),
        config.agent_settings.host.clone(),
        config.agent_settings.port as u16,
    );
    //sender!
    let mut current_agent_sender_name = config.agent_settings.name;
    current_agent_sender_name.push_str("_sender");
    let mut mqttoptions_sender = MqttOptions::new(
        current_agent_sender_name,
        config.agent_settings.host,
        config.agent_settings.port as u16,
    );
    mqttoptions_sender
        .set_keep_alive(Duration::from_secs(60))
        .set_manual_acks(false)
        .set_clean_session(true);

    mqttoptions
        .set_keep_alive(Duration::from_secs(60))
        .set_manual_acks(false)
        .set_clean_session(true);
    //initialize agent player
    let (client, mut eventloop) = AsyncClient::new(mqttoptions.to_owned(), 10);
    client
        .subscribe(agent_player, QoS::AtLeastOnce)
        .await
        .unwrap();
    client
        .subscribe(game_server.to_owned(), QoS::AtLeastOnce)
        .await
        .unwrap();

    loop {
        let notification = eventloop.poll().await.unwrap();
        match notification {
            Event::Incoming(Packet::Publish(p)) => {
                let topic_check = p.topic.to_owned();
                //process gameserver message
                if topic_check.eq(&game_server.to_owned()) {
                    println!("SENDING MESSAGE TO GAMESERVER");
                } else {
                    let recv_data = RecvWrapper::<usize>::deserialize_data(&p);
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
                                |cmd: &OperationObj,
                                 arg: &Vec<(usize, String)>,
                                 info: &AgentInfo| {
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
                            let Ok(cmd_to_server) = provider.get::<AgentCommand>() else {
                            println!("error while resolvig command!");
                            continue;
                        };
                            //resolve command and inject into server command
                            let cmd_server_transform: ServerCommand =
                                (*cmd_to_server).clone().into();
                            println!("PUBLISHING COMMAND TO GAMESERVER\n");
                            client
                                .publish(
                                    "gameserver_processor",
                                    QoS::AtLeastOnce,
                                    false,
                                    serde_json::to_vec(&cmd_server_transform).unwrap(),
                                )
                                .await
                                .unwrap();
                        }
                        Err(e) => {
                            println!("error while deserializing! err: {}", e);
                        }
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
