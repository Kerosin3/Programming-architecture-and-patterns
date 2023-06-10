#![allow(unreachable_code)]
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use templates::args::Argument;
use templates::data_exchange::sender_interface::SenderDataInterface;
use templates::data_exchange::OperationObj;
use tokio::{task, time};
mod implement;
use implement::SenderWrapper;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    //read config
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("src/agent_sender/conf/conf.toml");
    let config: Config = Figment::new()
        .merge(Toml::file(config_path))
        .merge(Env::prefixed("CARGO_"))
        .extract()?;
    //dbg!(config);
    // setup mtqq broker
    let mut subscribes = config.agent_settings.subscribes.to_owned();
    let agent_name = config.agent_settings.name.to_owned();
    subscribes.reverse();
    let bridge_processor = subscribes.pop().unwrap(); //get topic
    let auth_service_processor = subscribes.pop().unwrap();
    let mut mqttoptions = MqttOptions::new(
        config.agent_settings.name.to_owned(),
        config.agent_settings.host,
        config.agent_settings.port as u16,
    );
    mqttoptions
        .set_keep_alive(Duration::from_secs(60))
        .set_manual_acks(false)
        .set_clean_session(true);
    // setup eventloop
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(bridge_processor.to_owned(), QoS::AtLeastOnce)
        .await
        .unwrap();
    client
        .subscribe(auth_service_processor.to_owned(), QoS::AtLeastOnce)
        .await
        .unwrap();
    let username = config.agent_settings.name.clone();
    //--------------------------
    //spawn task initialize game
    //--------------------------
    let client2 = client.to_owned();
    let bridge2 = bridge_processor.to_owned();
    task::spawn(async move {
        let data_to_send_transformed = initialize_game(config.agent_settings.name);
        publish(&client2, &bridge2, &data_to_send_transformed).await;
        time::sleep(Duration::from_millis(100)).await;
    });

    //--------------------------
    // wait till game initializes
    //--------------------------
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                let Ok(recv_auth_data) = serde_json::from_slice::<AuthToAgent>(&p.payload) else{
                    //skip all message that are not from auth processor service
                    continue;
                };
                // skip not ours tokens. ohh.....
                if &recv_auth_data.username != &username {
                    continue;
                }
                let token = recv_auth_data.token;
                let gameid = recv_auth_data.gameid;
                println!(
                    "Accepted token for user {}, our name is {}, assigned gameid: {}\ntoken: [{}]",
                    recv_auth_data.username, &username, gameid, token
                );
                // now publish!
                let data_to_send_transformed = send_game_command(&username, token, gameid);
                publish(&client, &bridge_processor, &data_to_send_transformed).await;
            }
            Ok(Event::Incoming(i)) => {
                println!("Incoming = {i:?}");
            }
            Ok(Event::Outgoing(o)) => println!("Outgoing = {o:?}"),
            Err(e) => {
                println!("Error = {e:?}");
                return Ok(());
            }
        }
    }

    //polling eventloop
    while let Ok(_notification) = eventloop.poll().await {
        // println!("Received = {:?}", notification);
    }
    println!("finishing");
    Ok(())
}

pub fn initialize_game(agent_username: String) -> Vec<u8> {
    let arg0 = Argument::default()
        .assign_num(1)
        .assign_string("player_1".to_string())
        .finallize();
    // example arg 2
    let arg1 = Argument::default()
        .assign_num(2)
        .assign_string("player_2".to_string())
        .finallize();
    // construct message
    let mut data_to_send = SenderWrapper::default();
    data_to_send = data_to_send
        // setup gameid
        .assign_gameid(-1 as isize)
        .assign_obj_id(-1)
        .assign_name(&agent_username)
        .assign_arg(0, arg0)
        .unwrap()
        .assign_arg(1, arg1)
        .unwrap()
        .assign_timestamp()
        .assign_dbg(1 as isize)
        // select operation from Object
        .assign_operation(OperationObj::InitializeGame);
    data_to_send.transform_to_send()
}

pub fn send_game_command(username: &str, token: String, gameid: isize) -> Vec<u8> {
    let mut data_to_send: SenderWrapper<i32> = SenderWrapper::default();
    data_to_send = data_to_send
        // setup gameid
        .assign_gameid(gameid)
        .assign_obj_id(1)
        .assign_name(username)
        //.assign_arg(0, arg0)
        //.unwrap()
        //.assign_arg(1, arg1)
        //.unwrap()
        .assign_timestamp()
        .assign_dbg(1 as isize)
        // select operation from Object
        .assign_operation(OperationObj::send_play_command(token));
    data_to_send.transform_to_send()
}

// put to library!
#[derive(Serialize, Deserialize, Debug, Default)]
struct AuthToAgent {
    username: String,
    gameid: isize,
    token: String,
}

async fn publish(client: &AsyncClient, topic: &str, data_to_send: &[u8]) {
    client
        .publish(topic, QoS::AtLeastOnce, false, data_to_send)
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
