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
use templates::*;
use tokio::{task, time};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("src/agent/conf/conf.toml");
    let config: Config = Figment::new()
        .merge(Toml::file(config_path))
        .merge(Env::prefixed("CARGO_"))
        .extract()?;
    //     dbg!(config);
    let mut subscribes = config.agent_settings.subscribes.to_owned();
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

    loop {
        let notification = eventloop.poll().await.unwrap();
        match notification {
            Event::Incoming(Packet::Publish(p)) => {
                let x: Payload = serde_json::from_slice(&p.payload.to_vec()).unwrap();
                println!("json is {:?}", x);
            }
            /*
            Event::Incoming(Packet::Publish(p)) => {
                println!("Received: {:?}", p.payload);
            }*/
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
struct Agent_settings {
    name: String,
    version: String,
    subscribes: Vec<String>,
    host: String,
    port: isize,
}
#[derive(Deserialize, Debug)]
struct Config {
    agent_settings: Agent_settings,
}
