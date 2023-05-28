use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use tokio::{task, time};

use templates::args::Argument;
use templates::data_exchange::sender_interface::SenderDataInterface;
use templates::data_exchange::OperationObj;
mod implement;
use implement::SenderWrapper;
use num::Num;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("src/agent_sender/conf/conf.toml");
    let config: Config = Figment::new()
        .merge(Toml::file(config_path))
        .merge(Env::prefixed("CARGO_"))
        .extract()?;
    //     dbg!(config);
    let subscribes = config.agent_settings.subscribes.to_owned();
    let mut mqttoptions = MqttOptions::new(
        config.agent_settings.name.to_owned(),
        config.agent_settings.host,
        config.agent_settings.port as u16,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(60));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(subscribes.first().unwrap().clone(), QoS::AtLeastOnce)
        .await
        .unwrap();
    let username = config.agent_settings.name.to_owned();
    task::spawn(async move {
        for _i in 0..10 {
            let arg0 = Argument::default()
                .assign_num(_i)
                .assign_string("some".to_string())
                .finallize();
            let arg1 = Argument::default()
                .assign_num(_i)
                .assign_string("some1".to_string())
                .finallize();

            let data_to_send = SenderWrapper::default();
            let data_to_send = data_to_send
                .assign_gameid(1)
                .assign_obj_id(10)
                .assign_name(&username)
                .assign_operation(OperationObj::Dgb)
                .assign_arg(0, arg0)
                .unwrap()
                .assign_arg(1, arg1)
                .unwrap()
                .assign_timestamp()
                .assign_dbg(_i as isize)
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
