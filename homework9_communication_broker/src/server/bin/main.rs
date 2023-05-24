use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::error::Error;
use std::time::Duration;
use tokio::{task, time};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut mqttoptions = MqttOptions::new("rumqtt-async1", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("hello/rumqtt", QoS::AtMostOnce)
        .await
        .unwrap();

    task::spawn(async move {
        for i in 0..10 {
            client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
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
