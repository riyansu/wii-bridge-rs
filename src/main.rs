use structopt::StructOpt;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use serde::{Serialize};
use std::io::{stdin, stdout, Write};

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long)]
    mock: bool,
}

#[derive(Serialize)]
struct ButtonEvent {
    button: String,
    pressed: bool,
}

#[derive(Serialize)]
struct PointerEvent {
    x: f32,
    y: f32,
    valid: bool,
}

#[derive(Serialize)]
struct WsEvent<'a> {
    #[serde(rename = "type")]
    event_type: &'a str,
    data: serde_json::Value,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    if args.mock {
        println!("Running in mock input mode...");
        mock_input_loop().await;
    } else {
        println!("Wii remote connection mode is not yet implemented.");
    }
}

async fn mock_input_loop() {
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    println!("WebSocket >  ws://127.0.0.1:9001");

    if let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.unwrap();
        let (mut write, _) = ws_stream.split();

        loop {
            print!("Input 'button: <name>' or 'pointer: <x> <y>' (or 'exit'):\n> ");
            stdout().flush().unwrap();

            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            let line = buf.trim();

            if line == "exit" {
                break;
            }

            if let Some(rest) = line.strip_prefix("button:") {
                let button = rest.trim();
                let event = ButtonEvent {
                    button: button.to_string(),
                    pressed: true,
                };
                let json_value = serde_json::to_value(event).unwrap();
                let ws_event = WsEvent {
                    event_type: "button",
                    data: json_value,
                };
                let json = serde_json::to_string(&ws_event).unwrap();
                println!("SEND: {}", json);
                write.send(json.into()).await.unwrap();
            } else if let Some(rest) = line.strip_prefix("pointer:") {
                let coords: Vec<&str> = rest.trim().split_whitespace().collect();
                if coords.len() == 2 {
                    let x = coords[0].parse::<f32>().unwrap_or(0.0);
                    let y = coords[1].parse::<f32>().unwrap_or(0.0);
                    let event = PointerEvent { x, y, valid: true };
                    let json_value = serde_json::to_value(event).unwrap();
                    let ws_event = WsEvent {
                        event_type: "pointer",
                        data: json_value,
                    };
                    let json = serde_json::to_string(&ws_event).unwrap();
                    println!("SEND: {}", json);
                    write.send(json.into()).await.unwrap();
                } else {
                    println!("Invalid pointer input. Usage: pointer: <x> <y>");
                }
            } else {
                println!("Unknown input format.");
            }
        }
    }
}
