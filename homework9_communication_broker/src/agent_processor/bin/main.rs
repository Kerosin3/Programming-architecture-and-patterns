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

use templates::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("src/agent_processor/conf/conf.toml");
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

    task::spawn(async move {
        for i in 0..10 {
            /*           let mut json: Mydata = Mydata {
                            data: "bebebe".to_string(),
                            iter: i,
                        };
                        let json_string = serde_json::to_vec(&json).unwrap();
            */
            let s = Sender("test_text".to_string());
            let my_data = Mydata { operation: s };

            let x = my_data.operation.new();
            client
                .publish(
                    subscribes.first().unwrap().clone(),
                    QoS::AtLeastOnce,
                    false,
                    my_data.operation.metamorphose(),
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

struct Sender(String);

impl OperationConstructor for Sender {
    // assign String
    fn new(&self) -> OperationObj {
        OperationObj::Test(self.0.to_owned())
    }
}

impl OperationSender for Sender {
    // convert to json
    fn metamorphose(&self) -> Vec<u8> {
        serde_json::to_vec(&self.new()).unwrap()
    }
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
