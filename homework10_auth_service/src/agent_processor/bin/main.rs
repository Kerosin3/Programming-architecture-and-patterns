#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
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
// mod processor;
// use processor::*;
use templates::auth::*;
use templates::gameserver::*;
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
    //dbg!(config);
    //setup mtqq broker
    config.agent_settings.subscribes.reverse();
    //bridge processor
    let agent_player = config.agent_settings.subscribes.pop().unwrap();
    // game agent
    let game_server = config.agent_settings.subscribes.pop().unwrap();
    // auth agent
    let auth_server = config.agent_settings.subscribes.pop().unwrap();
    // auth response
    let auth_response = config.agent_settings.subscribes.pop().unwrap();
    //setup mqtt
    let mut mqttoptions = MqttOptions::new(
        config.agent_settings.name.clone(),
        config.agent_settings.host.clone(),
        config.agent_settings.port as u16,
    );
    //setup mqtt
    mqttoptions
        .set_keep_alive(Duration::from_secs(60))
        .set_manual_acks(false)
        .set_clean_session(true);
    //initialize agent player
    let (client, mut eventloop) = AsyncClient::new(mqttoptions.to_owned(), 10);
    //subscribe to bridge service (players)
    client
        .subscribe(agent_player, QoS::AtLeastOnce)
        .await
        .unwrap();
    // subscribe to gameserver
    client
        .subscribe(game_server.to_owned(), QoS::AtLeastOnce)
        .await
        .unwrap();
    client
        .subscribe(auth_server.to_owned(), QoS::AtLeastOnce)
        .await
        .unwrap();
    client
        .subscribe(auth_response.to_owned(), QoS::AtLeastOnce)
        .await
        .unwrap();

    // main loop
    loop {
        let notification = eventloop.poll().await.unwrap();
        match notification {
            Event::Incoming(Packet::Publish(p)) => {
                let topic_check = p.topic.to_owned();
                //process gameserver message
                // match message topic
                match topic_check.as_str() {
                    "auth_processor" => {
                        println!("SENDING MESSAGE TO AUTH SERVER [{}]", auth_server);
                    }
                    "gameserver_processor" => {
                        println!("SENDING MESSAGE TO GAMESERVER [{}]", game_server);
                    }
                    "auth_response" => {
                        // auth server registered users
                        println!("GOT REGISTERED USER");
                    }

                    "bridge_processor" => {
                        if let Ok(_rez) = deserialize_player_agent_msg(&p, &client).await {
                            println!("succesfully published!");
                        } else {
                            println!("error while publishing!");
                        }
                    }
                    _ => {
                        println!("RECEIVER FROM UNREGISTERED IN BRIDGE TOPIC");
                    }
                }
            }
            Event::Outgoing(_) => {
                //                 println!("Outgoing");
            }
            _ => {
                //                 println!("Other");
            }
        }
    }

    Ok(())
}

pub enum ProcessingErrors {
    ErrorDeserialization,
    ErrorResolvingCommand,
}

async fn deserialize_player_agent_msg(
    published: &rumqttc::Publish,
    client: &AsyncClient,
) -> Result<(), ProcessingErrors> {
    let recv_data = RecvWrapper::<usize>::deserialize_data(&published);
    match recv_data {
        Ok(d) => {
            //black magic
            let mut services = ServiceCollection::new();
            services.service(d.get_operation()); // get operation type
            let Ok(argz) = d.get_all_args_pairs() else { // Vector of all args
                                println!("error getting arg!");
                                return Err(ProcessingErrors::ErrorDeserialization)
                            };
            println!(
                "GOT VALID MESSAGE FROM A PLAYER {}, gameid [{}], args: {:?}",
                d.get_name(),
                d.get_gameid(),
                d.get_all_args_pairs()
            );
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
            let Ok(cmd_to_server) = provider.get::<AgentCommand>() else {
                                println!("error while resolvig command!");
                                return Err(ProcessingErrors::ErrorDeserialization)
                            };
            //resolve command and inject into server command
            println!("command is [{:?}]", cmd_to_server.cmd);
            // provide behaviour

            let cmd_server_transform: ServerCommand = (*cmd_to_server).clone().into();
            match cmd_to_server.cmd {
                OperationObj::InitializeGame => {
                    println!("PUBLISHING COMMAND TO AUTH SERVER");
                    client
                        .publish(
                            "auth_processor",
                            QoS::AtLeastOnce,
                            false,
                            serde_json::to_vec(&cmd_server_transform).unwrap(),
                        )
                        .await
                        .unwrap();
                }
                OperationObj::Play => todo!(),
                OperationObj::Test => todo!(),
                OperationObj::Dgb => todo!(),
                _ => todo!(),
            }
        }
        Err(e) => {
            println!("error while deserializing! err: {}", e);
            return Err(ProcessingErrors::ErrorDeserialization);
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
