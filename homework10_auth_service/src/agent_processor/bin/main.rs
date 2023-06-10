#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use ddi::*;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
mod implement;
use implement::*;
use templates::data_exchange::recv_interface::RecvDataInterface;
use templates::data_exchange::OperationObj;
mod auth_processor;
use auth_processor::*;
// mod processor;
// use processor::*;
use jwt_simple::prelude::*;
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
    // processor auth
    let auth_service_processor = config.agent_settings.subscribes.pop().unwrap();
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
    client
        .subscribe(auth_service_processor.to_owned(), QoS::AtLeastOnce)
        .await
        .unwrap();

    let mut registered_users = AuthUsersDB::default();
    // main loop
    loop {
        let notification = eventloop.poll().await.unwrap();
        match notification {
            Event::Incoming(Packet::Publish(p)) => {
                let topic_check = p.topic.to_owned();
                //process gameserver message
                // match message topic
                //println!("topic is {}", topic_check);
                match topic_check.as_str() {
                    // sending auth request to auth service
                    "auth_processor" => {
                        println!("SENDING MESSAGE TO AUTH SERVER [{}]", auth_server);
                    }
                    // sending command to gameserver service
                    "gameserver_processor" => {
                        println!("SENDING MESSAGE TO GAMESERVER [{}]", game_server);
                    }
                    // process auth response from auth service
                    "auth_response" => {
                        // auth server registered users
                        println!("GOT REGISTERED USER");
                        let Ok(auth_message_back) = serde_json::from_slice::<AuthMessage>(&p.payload) else {
                            println!("error while deserializing message from auth agent");
                            continue;
                        };
                        // register user in processor here
                        println!(
                            "message: {} {}",
                            auth_message_back.username, auth_message_back.token
                        );
                        let gameid = match auth_message_back.status {
                            AuthError::Okey(id) => id,
                            _ => {
                                println!("NOT OK ANSWER FROM AUTH, IGNORING");
                                continue;
                            }
                        };
                        //register user in processor
                        registered_users
                            .insert_user(auth_message_back.username.to_owned(), &auth_message_back);
                        let mut back_to_agent = AuthToAgent::default();
                        // create auth message back to agent
                        back_to_agent.assign(
                            auth_message_back.username.to_owned(),
                            gameid,
                            auth_message_back.token.to_owned(),
                        );
                        //publish auth token to agent
                        client
                            .publish(
                                "auth_service_processor", // back to agent!
                                QoS::AtLeastOnce,
                                false,
                                serde_json::to_vec(&back_to_agent).unwrap(),
                            )
                            .await
                            .unwrap();
                    }
                    "bridge_processor" => {
                        if let Ok(_rez) =
                            // deserialize and match agent message, send response
                            deserialize_player_agent_msg(&p, &client, &registered_users)
                                    .await
                        {
                            println!("succesfully published message to gameserver!");
                        } else {
                            println!("error while publishing message to gameserver!");
                        }
                    }
                    _ => {
                        println!("RECEIVED A MESSAGE FROM UNREGISTERED IN BRIDGE TOPIC, SKIPPING");
                    }
                }
            }
            Event::Outgoing(_) => {
                //println!("Outgoing");
            }
            _ => {
                //println!("Other");
            }
        }
    }

    Ok(())
}

//---------------------------------------------------------
//---------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Default)]
struct AuthToAgent {
    username: String,
    gameid: isize,
    token: String,
}
impl AuthToAgent {
    fn assign(&mut self, username: String, gameid: isize, token: String) {
        self.username = username;
        self.gameid = gameid;
        self.token = token;
    }
}
//---------------------------------------------------------
//---------------------------------------------------------

pub enum ProcessingErrors {
    ErrorDeserialization,
    ErrorResolvingCommand,
    ErrorNoSuchUserRegistered,
}

async fn deserialize_player_agent_msg(
    published: &rumqttc::Publish,
    client: &AsyncClient,
    db: &AuthUsersDB,
) -> Result<(), ProcessingErrors> {
    let recv_data = RecvWrapper::<usize>::deserialize_data(&published);
    match recv_data {
        Ok(d) => {
            //black magic
            match d.get_operation() {
                // GAME INIT
                OperationObj::InitializeGame => {
                    let mut services = ServiceCollection::new();
                    services.service(d.get_operation()); // get operation type

                    let Ok(argz) = d.get_all_args_pairs() else { // Vector of all args
                                println!("error getting args!");
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
                                    cmd: cmd.to_owned(),
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
                        OperationObj::Play(_) => todo!(),
                        OperationObj::Test => todo!(),
                        OperationObj::Dgb => todo!(),
                        _ => todo!(),
                    }
                }
                // PLAY
                OperationObj::Play(token) => {
                    // now playing!
                    // check token and gameid

                    let req_uname = d.get_name();
                    let token = token;
                    let gameid = d.get_gameid();
                    println!("requested name {}, {}, {}", req_uname, token, gameid);
                    let key_for_this_user = db.get_key_for_user(req_uname);
                    let Some(restored_key) = key_for_this_user else {
                        println!("no such user in database!");
                        return Err(ProcessingErrors::ErrorResolvingCommand);
                    };
                    if let Ok(_claim) = restored_key.verify_token::<NoCustomClaims>(&token, None) {
                        println!("TOKEN VALIDATION FOR USER {} PASSED!, PROCEED", req_uname);
                        //-----------------------
                        // add logic to publish further to gameserver
                        //-----------------------
                    } else {
                        println!("TOKEN VALIDATION FOR USER {} NOT PASSED!", req_uname);
                    }

                    //                     let claims = key.verify_token::<NoCustomClaims>(&token, None)?;
                }
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
