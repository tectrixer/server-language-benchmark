mod benchmark;
pub mod commands;

use std::error::Error;

use std::str::FromStr;

use crate::commands::CompleteCommand;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;

const CONCURRENT_CONNS: usize = 200;

const BENCH_COUNT: usize = 100_000;

/*
#[tokio::main]
async fn main() {
    let socket = TcpStream::connect("127.0.0.1:8080").await.expect("Error during connection");
    let (socket_read, socket_write) = socket.into_split();
    let (mut stream_read, mut stream_write) = (BufReader::new(socket_read), BufWriter::new(socket_write));
    loop {
        println!("Enter input => ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error during read");
        println!("Writing => {:?}", input);
        stream_write.write_all((input).as_bytes()).await.expect("Error during connection write");
        stream_write.flush().await.unwrap();
        println!("Starting to read");
        let mut back = String::new();
        stream_read.read_line(&mut back).await.expect("Error");
        println!("=> {:?}", back);
    }
}
*/

fn get_key() -> Result<String, Box<dyn std::error::Error>> {
    let res = dialoguer::Input::new()
        .with_prompt("Enter Key")
        .interact()?;
    Ok(res)
}

fn get_value() -> Result<String, Box<dyn std::error::Error>> {
    let res = dialoguer::Input::new()
        .with_prompt("Enter Value")
        .interact()?;
    Ok(res)
}

fn get_duration() -> Result<chrono::Duration, Box<dyn std::error::Error>> {
    let hours_string = dialoguer::Input::new()
        .with_prompt("Enter hours:")
        // .default("0")
        .validate_with(|input: &String| -> Result<(), String> {
            let num = i64::from_str(input);
            match num {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        })
        .interact()?;
    let hours = i64::from_str(&hours_string)?;
    let minutes_string = dialoguer::Input::new()
        .with_prompt("Enter minutes:")
        // .default("0")
        .validate_with(|input: &String| -> Result<(), String> {
            let num = i64::from_str(input);
            match num {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        })
        .interact()?;
    let minutes = i64::from_str(&minutes_string)?;
    let seconds_string = dialoguer::Input::new()
        .with_prompt("Enter seconds:")
        // .default("10")
        .validate_with(|input: &String| -> Result<(), String> {
            let num = i64::from_str(input);
            match num {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        })
        .interact()?;
    let seconds = i64::from_str(&seconds_string)?;
    let hours_duration = chrono::Duration::hours(hours);
    let minutes_duration = chrono::Duration::minutes(minutes);
    let seconds_duration = chrono::Duration::seconds(seconds);
    Ok(hours_duration + minutes_duration + seconds_duration)
}

fn command_string() -> Result<String, Box<dyn Error>> {
    let command_type = &[
        "Get",
        "Set",
        "Del",
        "GetCounter",
        "SetCounter",
        "DelCounter",
        "GetDump",
        "NewDump",
        "DumpInterval",
        "SetTTL",
    ];

    let selection = dialoguer::Select::new()
        .with_prompt("Select type")
        .items(command_type)
        .interact()?;

    let command = match selection {
        0 => CompleteCommand::Get { key: get_key()? },
        1 => {
            let key = get_key()?;
            let value = get_value()?;
            CompleteCommand::Set { key, value }
        }
        2 => {
            let key = get_key()?;
            CompleteCommand::Del { key }
        }
        3 => CompleteCommand::GetCounter,
        4 => CompleteCommand::SetCounter,
        5 => CompleteCommand::DelCounter,
        6 => CompleteCommand::GetDump,
        7 => CompleteCommand::NewDump,
        8 => {
            let duration = get_duration()?;
            CompleteCommand::DumpInterval { duration }
        }
        9 => {
            let key = get_key()?;
            let value = get_key()?;
            let duration = get_duration()?;
            CompleteCommand::SetTTL {
                key,
                value,
                duration,
            }
        }
        _ => {
            panic!("This shouldn't have happened.")
        }
    };

    Ok(String::from(command))
}

async fn interactive_mode() {
    loop {
        let socket = TcpStream::connect("127.0.0.1:8080")
            .await
            .expect("Error during connection");
        let (socket_read, socket_write) = socket.into_split();
        let (mut stream_read, mut stream_write) =
            (BufReader::new(socket_read), BufWriter::new(socket_write));
        let input = command_string().unwrap();
        println!("Writing => {:?}", input);
        stream_write
            .write_all((input + "\n").as_bytes())
            .await
            .expect("Error during connection write");
        stream_write.flush().await.unwrap();
        println!("Starting to read");
        let mut back = String::new();
        stream_read.read_line(&mut back).await.expect("Error");
        println!("=> {:?}", back);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // console_subscriber::init();

    let mode_selection = &["Interactive", "Generate", "Perform Benchmark"];

    let selection = dialoguer::Select::new()
        .with_prompt("Select type")
        .items(mode_selection)
        .interact()?;

    match selection {
        0 => {
            interactive_mode().await;
        }
        1 => {
            benchmark::generate_data().await;
        }
        2 => {
            benchmark::perform_benchmark().await;
        }
        _ => {
            unimplemented!()
        }
    }
    Ok(())
}
