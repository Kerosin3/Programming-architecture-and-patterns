use figment::{
    providers::{Env, Format, Json, Toml},
    Figment,
};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;
use tokio::{task, time};

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    publish: Option<bool>,
    edition: String, // ... and so on ...
}
#[derive(Deserialize, Debug)]
struct Config {
    package: Package,
    rustc: Option<String>,
    // ... and so on ...
}
const TOPIC: &str = "test_topic";
const Agent_name: &str = "agent1";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Figment::new()
        .merge(Toml::file("my.toml"))
        .merge(Env::prefixed("CARGO_"))
        //         .merge(Env::raw().only(&["RUSTC", "RUSTDOC"]))
        //         .join(Json::file("Cargo.json"))
        .extract()?;
    dbg!(config);
    /*
    let mut mqttoptions = MqttOptions::new(Agent_name, "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(60));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(TOPIC, QoS::AtMostOnce).await.unwrap();

    loop {
        let notification = eventloop.poll().await.unwrap();
        match notification {
            Event::Incoming(Packet::Publish(p)) => {
                println!("Received: {:?}", p.payload);
            }
            Event::Outgoing(_) => {
                println!("Outgoing");
            }
            _ => {
                println!("Other");
            }
        }
    }*/
    Ok(())
}
