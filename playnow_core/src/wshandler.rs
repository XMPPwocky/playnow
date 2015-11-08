use backend;
use websocket;
use websocket::{Message, Sender, Receiver};
use websocket::message::Type;
use steamid::SteamId;
use std;

pub type WebSocketConnection = websocket::server::Connection<
websocket::stream::WebSocketStream,
websocket::stream::WebSocketStream>;

pub type WebSocketClient = websocket::client::Client<websocket::dataframe::DataFrame,
websocket::server::Sender<websocket::stream::WebSocketStream>,
websocket::server::Receiver<websocket::stream::WebSocketStream>>;

pub fn handler(connection: WebSocketConnection) {
    use websocket::server::request::RequestUri;

    println!("connect");

    // FIXME: this is terrible
    let mut backend = backend::Backend::new();

    let request = connection.read_request().unwrap();
    request.validate().unwrap();

    let sessionid = match request.url {
        RequestUri::AbsolutePath(ref path) if path.len() > 1 => {
            Some(path[1..].to_owned())
        },
        _ => None
    };

    // FIXME: better error handling
    let steamid = sessionid.and_then(|sessionid| backend.auth_request(&sessionid).ok());
    match steamid {
        Some(Some(steamid)) => {
            let client = request.accept().send().unwrap();
            handler_mainloop(backend, client, steamid);
        },
        _ => {
            request.fail().send().unwrap();
        }
    }
}

fn handler_mainloop(mut backend: backend::Backend, client: WebSocketClient, steamid: SteamId) {
    let (mut sender, mut receiver) = client.split();

    for message in receiver.incoming_messages() {
        let message: Result<Message, _> = message;
        if let Ok(message) = message {
            match message.opcode {
                Type::Close => {
                    let message = Message::close();
                    sender.send_message(&message).unwrap();
                    return;
                },
                Type::Ping => {
                    let message = Message::pong(message.payload);
                    sender.send_message(&message).unwrap();
                }
                Type::Text => {
                    let payload_str = std::str::from_utf8(&message.payload);
                    if let Ok(payload_str) = payload_str {
                        let message = Message::text(
                            if payload_str == "start_playing" {
                                format!("{:?}", backend.start_playing(steamid))
                            } else {
                                format!("{:?}", backend.get_queue_status(steamid))
                            }
                            );

                        sender.send_message(&message).unwrap();
                    }
                }
                _ => sender.send_message(&message).unwrap(),
            }
        }
    }
}
