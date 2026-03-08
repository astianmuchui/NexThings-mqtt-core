use rumqttc::{ Client, MqttOptions, QoS };
use std::time::Duration;

fn main() {
    println!("Starting MQTT Subscriber...\n");

    let mut mqttoptions = MqttOptions::new(
        "",
        "",
        1883
    );

    mqttoptions.set_keep_alive(Duration::from_secs(60));
    mqttoptions.set_credentials("", "");

    let (client, mut connection) = Client::new(mqttoptions, 10);

    let topic = "#";
    println!("Subscribing to topic: {}", topic);

    client.subscribe(topic, QoS::AtLeastOnce).unwrap();

    println!("Connected! Waiting for messages...\n");
    println!("Press Ctrl+C to exit\n");
    println!("{:-<50}", ""); // Separator line

    for notification in connection.iter() {
        match notification {
            Ok(event) => {

                if let rumqttc::Event::Incoming(packet) = event {
                    if let rumqttc::Packet::Publish(publish) = packet {
                        let payload = String::from_utf8_lossy(&publish.payload);

                        println!(" Message received!");
                        println!("   Topic: {}", publish.topic);
                        println!("   Payload: {}", payload);
                        println!("   QoS: {:?}", publish.qos);
                        println!("{:-<50}", "");
                    }
                }
            }
            Err(e) => {
                eprintln!(" Error: {:?}", e);
                eprintln!("Attempting to reconnect...");

            }
        }
    }
}
