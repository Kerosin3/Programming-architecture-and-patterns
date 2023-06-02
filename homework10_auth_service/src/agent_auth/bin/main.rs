#![allow(unreachable_code)]
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use jwt_simple::prelude::*;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use templates::gameserver::ServerCommand;
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
    let gameserver_agent = subscribes.pop().unwrap();
    subscribe_to(&client, &gameserver_agent).await;
    loop {
        let notification = eventloop.poll().await.unwrap();
        match notification {
            Event::Incoming(Packet::Publish(publisher)) => {
                println!(
                    "Topic: {}, Payload: {:?}",
                    publisher.topic, publisher.payload
                );

                let Ok(recv_data) = serde_json::from_slice::<ServerCommand>(&publisher.payload) else {
                    println!("error while deserializing data!");
                    continue;
                };
                println!("got data: {}", recv_data);
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
