#![allow(unreachable_code)]
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
    let bridge_processor = subscribes.pop().unwrap(); //get topic
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
    let username = config.agent_settings.name.to_owned();
    //spawn task
    task::spawn(async move {
        for _i in 0..1 {
            // example arg 1
            let arg0 = Argument::default()
                .assign_num(_i)
                .assign_string(config.agent_settings.name.to_owned())
                .finallize();
            // example arg 2
            let arg1 = Argument::default()
                .assign_num(_i)
                .assign_string("another_player".to_string())
                .finallize();
            // construct message
            let mut data_to_send = SenderWrapper::default();
            data_to_send = data_to_send
                // setup gameid
                .assign_gameid(42 as isize)
                .assign_obj_id(-1)
                .assign_name(&username)
                .assign_arg(0, arg0)
                .unwrap()
                .assign_arg(1, arg1)
                .unwrap()
                .assign_timestamp()
                .assign_dbg(_i as isize)
                // select operation from Object
                .assign_operation(OperationObj::InitializeGame);
            let data_to_send_transformed = data_to_send.transform_to_send();
            publish(&client, &bridge_processor, &data_to_send_transformed).await;

            println!("Sended message to bridge! [{}]", data_to_send);
            time::sleep(Duration::from_millis(100)).await;
        }
    });
    //polling eventloop
    while let Ok(_notification) = eventloop.poll().await {
        // println!("Received = {:?}", notification);
    }
    println!("finishing");
    Ok(())
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
