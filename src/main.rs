use structopt::StructOpt;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use serde::{Serialize};

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
            println!("Input any text here.");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            let button = buf.trim();

            if button == "exit" {
                break;
            }

            let event = ButtonEvent {
                button: button.to_string(),
                pressed: true,
            };

            let json = serde_json::to_string(&event).unwrap();
            println!("SEND: {}", json);
            write.send(json.into()).await.unwrap();
        }
    }
}
