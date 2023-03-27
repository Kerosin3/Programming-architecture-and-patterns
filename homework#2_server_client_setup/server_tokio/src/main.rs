use anyhow::anyhow;
use bytes::Bytes;
use futures::SinkExt;
use futures_util::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use socket2::SockRef;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::io::BufStream;
use tokio::net::{TcpListener, TcpStream};
//use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
mod ipc_message;
use ipc_message::*;
mod server_socket_struct;
mod server_termometer_struct;
use lib_shouse::home::home::home::*;
use server_socket_struct::*;
use server_termometer_struct::*;
use tokio::time::timeout;
use tracing::Level;
use tracing_subscriber;
use tracing_subscriber::fmt;

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() -> anyhow::Result<()> {
    let subscriber = fmt()
        .compact()
        //        .with_line_number(true) ?? not find
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("start main server loop");
    let mut some_house = SmartHouse::new();
    let room_0 = "room_0".to_string();
    some_house.append_room(&room_0).unwrap();
    let dev0 = wrap_device(SmartSocket::new());
    let dev1 = wrap_device(Termometer::new());
    let _dev0_handler = some_house.append_dev_to_a_room(&room_0, &dev0).unwrap(); // append dev0 to room0
    let _dev1_handler = some_house.append_dev_to_a_room(&room_0, &dev1).unwrap(); // append dev1 to room0
    _dev0_handler.property_change_state(9000_f32).unwrap();
    _dev1_handler.property_change_state(36.6_f32).unwrap();
    tracing::info!(
        "added device:{} to server",
        _dev0_handler.get_devname().unwrap()
    );
    tracing::info!(
        "added device:{} to server",
        _dev1_handler.get_devname().unwrap()
    );

    //-----------------event_loop-------------------
    let wrap_home = Arc::new(Mutex::new(some_house));
    tokio::spawn(async move { imitate_socket_power_change(_dev0_handler).await }); // change dev0
    tokio::spawn(async move { imitate_termo_data_achange(_dev1_handler).await }); // change dev1
                                                                                  // property
    if let Ok(tcp_listener) = TcpListener::bind("127.0.0.1:12345").await {
        while let Ok((tcp_stream, _socket_addr)) = tcp_listener.accept().await {
            let sh = Arc::clone(&wrap_home);
            tokio::spawn(async move {
                // -----------------------------SPAWN TASK--------------------------------------
                let handle =
                    timeout(Duration::from_secs(5), server_event_loop(tcp_stream, sh)).await;
                // anylyze task rezults
                match handle {
                    Ok(v) => {
                        tracing::info!("finished a task with no timeout");
                        match v {
                            Ok(_) => tracing::info!("task processed successfully!"),
                            Err(e) => tracing::error!("error while task execution:{}", e),
                        }
                    }
                    Err(e) => tracing::error!("error while executing the task: {e}"),
                }
            });
        }
    } else {
        tracing::error!("cant bind server!");
        std::process::exit(1);
    }
    Ok(())
}
async fn imitate_socket_power_change(handle: Device_Handler) {
    let mut rng: StdRng = SeedableRng::from_entropy();
    loop {
        handle
            .property_change_state(rng.gen_range(1000..5000))
            .unwrap();
        sleep(Duration::from_millis(100)).await;
    }
}
async fn imitate_termo_data_achange(handle: Device_Handler) {
    let mut rng: StdRng = SeedableRng::from_entropy();
    loop {
        handle.property_change_state(rng.gen_range(30..90)).unwrap();
        sleep(Duration::from_millis(100)).await;
    }
}
async fn server_event_loop(
    tcp_stream: TcpStream,
    sm_obj: Arc<Mutex<SmartHouse>>,
) -> anyhow::Result<()> {
    tracing::info!(
        "initialized server connection with {}",
        tcp_stream.peer_addr().unwrap()
    );
    let socket_ref = SockRef::from(&tcp_stream);
    socket_ref.set_nonblocking(true)?;
    socket_ref.set_nodelay(true).unwrap();
    socket_ref.listen(128); // panics on unwrap
    socket_ref.set_reuse_port(true)?;
    socket_ref.set_reuse_address(true)?;
    socket_ref.set_recv_buffer_size(2048)?;
    socket_ref.set_send_buffer_size(2048)?;
    std::mem::drop(socket_ref); // ok
    let codec = LengthDelimitedCodec::builder()
        .length_field_offset(0) // default value
        .length_field_type::<u16>()
        .length_adjustment(0) // default value
        .new_codec();
    let stream_buf = BufStream::new(tcp_stream);
    let mut framed_stream = Framed::new(stream_buf, codec);
    let frame = Bytes::from("hello from server, what do you want?");
    tracing::info!("sent hello to client");
    framed_stream.send(frame).await.unwrap();
    // INFINITE LOOP!
    let mut i = 0_usize;
    '_accept_client_request: while let Some(frame) = framed_stream.next().await {
        match frame {
            Ok(f) => {
                if i == 0 {
                    tracing::info!("zero frame: {f:?}");
                    if String::from_utf8_lossy(&f) == "client magic words!" {
                        let frame = Bytes::from("ASK");
                        framed_stream.send(frame).await?;
                        tracing::info!("sent akn to client");
                    } else {
                        tracing::error!("wrong key phrase from client");
                        anyhow::bail!(HandleError::KeyError) // wrong answer
                    }
                    i += 1;
                } else {
                    //--------------------------------------------------------------------//
                    //PROCESS INFO MESSAGE!
                    //println!("readed frame: {f:?}");
                    i += 1;
                    let msg_from_client: Message = Message::deserialize_message(&f);
                    let dev_name = msg_from_client.devname;
                    let room_dev = match sm_obj.try_lock() {
                        Err(_) => {
                            tracing::error!("error locking mutex");
                            anyhow::bail!(HandleError::MutexError(anyhow!("error locking mutex")))
                        }
                        Ok(guard) => guard.test_whether_a_dev_exists(&dev_name),
                    };
                    let mut message_to_client = Message::new(NetMsgType::SendServer);
                    message_to_client.assign_devname(dev_name.to_owned());
                    message_to_client.assign_command(Command::MsgBack);
                    let (room_name_found, dev_name_found) = if room_dev.is_some() {
                        tracing::info!("found valid dev {dev_name}");
                        room_dev.unwrap()
                    } else {
                        tracing::error!("not valid device found, aborting connection");
                        let msg_back = format!("devname:{} not found", &dev_name);
                        framed_stream.send(Bytes::from(msg_back)).await?;
                        anyhow::bail!(HandleError::NoSuchDeviceExists)
                    };
                    let mut info_property = String::new();
                    match msg_from_client.command.unwrap() {
                        Command::TurnOn => {
                            modify_house(
                                &sm_obj,
                                Command::TurnOn,
                                (&room_name_found, &dev_name_found),
                            )
                            .await?; //works?
                            info_property.push_str("device is turned ON");
                        }
                        Command::TurnOff => {
                            modify_house(
                                &sm_obj,
                                Command::TurnOff,
                                (&room_name_found, &dev_name_found),
                            )
                            .await?; //works?
                            info_property.push_str("device is turned OFF");
                        }
                        Command::GetProperty => {
                            info_property = modify_house(
                                &sm_obj,
                                Command::GetProperty,
                                (&room_name_found, &dev_name_found),
                            )
                            .await?;
                            message_to_client.assign_info(info_property.to_owned());
                        }
                        Command::MsgBack => todo!(),
                    }
                    // write answer
                    let frame = Bytes::from(info_property);
                    framed_stream.send(frame).await.unwrap();
                    tracing::info!("sended information back!");
                    tracing::info!("-----------compliting the task!------------");
                    return Ok(()); //run once
                }
            }
            Err(_) => {
                tracing::error!("cant process framing operation");
                anyhow::bail!(HandleError::FrameError) // cannot read from socket
            }
        }
    }
    tracing::error!("cant call next method on frame");
    anyhow::bail!(HandleError::WrongSeq)
}

fn wrap_device<T: 'static + lib_shouse::home::home::home::Device + Send + Sync>(
    some_device: T,
) -> Arc<Mutex<dyn Device + Send>> {
    Arc::new(Mutex::new(some_device))
}

async fn modify_house(
    sm_obj: &Arc<Mutex<SmartHouse>>,
    cmd: Command,
    (room, dev): (&str, &str),
) -> anyhow::Result<String> {
    match cmd {
        Command::TurnOn => {
            if let Ok(mut guard) = sm_obj.try_lock() {
                guard.change_dev_state_in_room(room, dev, true)?;
                Ok(String::new())
            } else {
                anyhow::bail!(HandleError::ErrorHouseLocking)
            }
        }
        Command::TurnOff => {
            if let Ok(mut guard) = sm_obj.try_lock() {
                guard.change_dev_state_in_room(room, dev, false)?;
                Ok(String::new())
            } else {
                anyhow::bail!(HandleError::ErrorHouseLocking)
            }
        }
        Command::GetProperty => {
            if let Ok(guard) = sm_obj.try_lock() {
                Ok(guard.get_device_property(dev)?) // HOW?????
            } else {
                anyhow::bail!(HandleError::ErrorHouseLocking)
            }
        }
        Command::MsgBack => todo!(),
    }
}

#[derive(Debug, Error)]
pub enum HandleError {
    #[error("Error locking smart house object")]
    ErrorHouseLocking,
    #[error("Error reading from socket")]
    SockError,
    #[error("Error keyprase")]
    KeyError,
    #[error("No such device")]
    NoSuchDeviceExists,
    #[error("Wrong sequence")]
    WrongSeq,
    #[error("frame processing error")]
    FrameError,
    #[error("frame semding error")]
    FrameErrorSend,
    #[error(transparent)]
    TokioError(#[from] anyhow::Error),
    //#[error(transparent)]
    //mutex_error(#[from] std::sync::TryLockError<T>),
    #[error(transparent)]
    MutexError(anyhow::Error),
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_something_async() {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg("echo hello command")
            .output()
            .expect("error command");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        assert!(output.status.success());
    }
}
