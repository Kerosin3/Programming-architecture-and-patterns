#![allow(unreachable_code)]
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::default::Default;
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use templates::auth::*;
use templates::gameserver::{GameServerCommands, ServerCommand};
mod implements;
use implements::*;
use std::sync::Arc;
//-------------------------------------------
//-------------------------------------------
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    // read agent config
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("src/agent_auth/conf/conf.toml");
    let config: Config = Figment::new()
        .merge(Toml::file(config_path))
        .merge(Env::prefixed("CARGO_"))
        .extract()?;
    // setup mtqq broker
    let mut subscribes = config.agent_settings.subscribes.to_owned();
    subscribes.reverse();
    let mut mqttoptions = MqttOptions::new(
        config.agent_settings.name,
        config.agent_settings.host,
        config.agent_settings.port as u16,
    );
    mqttoptions
        .set_keep_alive(Duration::from_secs(60))
        .set_manual_acks(false)
        .set_clean_session(true);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    let auth_transport = subscribes.pop().unwrap();
    subscribe_to(&client, &auth_transport).await;
    let auth_response = subscribes.pop().unwrap();
    subscribe_to(&client, &auth_response).await;
    let mut auth_users = Database::default();
    let mut games_initialized = Arc::new(Vec::<isize>::new());
    loop {
        let notification = eventloop.poll().await.unwrap();
        match notification {
            Event::Incoming(Packet::Publish(publisher)) => {
                println!(
                    " RECEIVED MESSAGE: Topic: {}, Payload: {:?}",
                    publisher.topic, publisher.payload
                );
                if publisher.topic != auth_transport {
                    continue;
                }
                let Ok(recv_data) = serde_json::from_slice::<ServerCommand>(&publisher.payload) else {
                    println!("error while deserializing data!");
                    continue;
                };
                let cmd_from_bridge = recv_data.cmd;
                match cmd_from_bridge {
                    GameServerCommands::SrvDbg => {}
                    GameServerCommands::SrvRotateObject => {}
                    GameServerCommands::SrvGameInit => {
                        //register users
                        //creator
                        let creator_username = recv_data.info.username.to_owned();
                        //other users
                        let mut users = Vec::<String>::new();
                        for user in recv_data.args.iter() {
                            users.push(user.1.to_owned());
                        }
                        users.push(creator_username);
                        // preparing answer
                        for user in users.iter() {
                            if auth_users.test_whether_user_already_registered(user) {
                                println!("User is already registered!");
                                continue;
                            }
                            //answer default struct
                            let mut answ = AuthMessageWrapper::default();
                            let gameid = 4242_isize;
                            answ.gen_key();
                            answ.gen_token();
                            answ.set_username(user.to_owned());
                            answ.assign_gameid(gameid); //sets status OK
                                                        //register in BD
                            let auth_data = answ.get_auth_data_copy();
                            //insert to table
                            if let Err(e) =
                                auth_users.insert_to_db(&answ.0.username, answ.get_auth_data_copy())
                            {
                                println!("error during db insertion: {}", e);
                            }
                            client
                                .publish(
                                    "auth_response",
                                    QoS::AtLeastOnce,
                                    false,
                                    answ.get_serialized(),
                                )
                                .await
                                .unwrap();
                        }
                        let gameid = 4242_isize;
                        {
                            let vt = Arc::get_mut(&mut games_initialized).unwrap();
                            if vt.contains(&gameid) {
                                println!("game already initialized!");
                                // return error
                                continue;
                            } else {
                                vt.push(gameid);
                            }
                        }
                    }
                    _ => todo!(),
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

// implement generator
struct AuthMessageWrapper(AuthMessage, isize);
impl Default for AuthMessageWrapper {
    fn default() -> Self {
        Self(Default::default(), 0)
    }
}
impl GeneratorAuth for AuthMessageWrapper {
    fn gen_key(&mut self) {
        self.0.generate_key();
    }
    fn gen_token(&mut self) {
        self.0.generate_token();
    }

    fn assign_gameid(&mut self, id: isize) {
        self.1 = id;
        self.0.assign_status_ok(id);
    }
    fn get_auth_data_copy(&self) -> AuthData {
        AuthData {
            uname: self.0.username.to_owned(),
            key: self.0.key.to_owned(),
            token: self.0.token.to_owned(),
            gameid: self.1,
        }
    }
    fn get_serialized(&self) -> Vec<u8> {
        serde_json::to_vec(&self.0).unwrap()
    }

    fn set_username(&mut self, name: String) {
        self.0.username = name.to_owned();
    }
}

async fn subscribe_to(client: &AsyncClient, topic: &str) {
    client
        .subscribe(topic.to_owned(), QoS::AtLeastOnce)
        .await
        .unwrap();
}

#[allow(dead_code)]
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
