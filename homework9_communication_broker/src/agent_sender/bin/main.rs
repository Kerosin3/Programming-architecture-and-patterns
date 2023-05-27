use figment::{
    providers::{Env, Format, Json, Toml},
    Figment, Source,
};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tokio::{task, time};

use templates::sender::*;
use templates::*;
mod sender_implement;
use sender_implement::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("src/agent_sender/conf/conf.toml");
    let config: Config = Figment::new()
        .merge(Toml::file(config_path))
        .merge(Env::prefixed("CARGO_"))
        .extract()?;
    //     dbg!(config);
    let mut subscribes = config.agent_settings.subscribes.to_owned();
    let mut mqttoptions = MqttOptions::new(
        config.agent_settings.name.to_owned(),
        config.agent_settings.host,
        config.agent_settings.port as u16,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(60));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(subscribes.first().unwrap().clone(), QoS::AtMostOnce)
        .await
        .unwrap();
    let username = config.agent_settings.name.to_owned();
    task::spawn(async move {
        for i in 0..10 {
            let data_to_send = SenderDataContainer::new(
                1,
                1,
                username.to_owned(),
                OperationAdapter(Box::new(Playgame())),
            )
            .transform_to_send();

            client
                .publish(
                    subscribes.first().unwrap().clone(),
                    QoS::AtLeastOnce,
                    false,
                    data_to_send,
                )
                .await
                .unwrap();
            time::sleep(Duration::from_millis(100)).await;
        }
    });

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {:?}", notification);
    }
    println!("finishing");
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
