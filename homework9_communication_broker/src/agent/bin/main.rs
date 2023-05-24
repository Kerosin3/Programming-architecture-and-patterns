use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::error::Error;
use std::time::Duration;
use tokio::{task, time};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut mqttoptions = MqttOptions::new("agent2", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(60));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("hello/rumqtt", QoS::AtMostOnce)
        .await
        .unwrap();

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
    }
    Ok(())
}
