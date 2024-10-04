use clap_serde_derive::ClapSerde;
use std::{
    io::{self, Write},
    time::Duration,
};

use colored::Colorize;
use rand::random;
use tokio::{
    io::{AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
    task::JoinHandle,
};
use tsunami::*;

const COUNTDOWN_START_SECONDS: usize = 5;
const COUNTDOWN_START_SUBSTEPS: usize = 8;

struct Context {
    args: Args,
}

async fn usage_warn() -> bool {
    const USAGE_WARNING: &str = "***** WARNING *****
Tsunami is a tool designed to stress-test pixelflut servers,
when you use this tool, you take full responsibility for any
consequences that it might bring.

Do not run this on public server instances without explicit
consent from the instance owner.
";
    println!("{}", USAGE_WARNING);
    let mut cout = std::io::stdout();

    let mut line = "".to_owned();
    print!("Continue? (y/N): ");
    cout.flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();
    if line.starts_with("y") {
        print!("Starting in: ");
        for i in (0..=COUNTDOWN_START_SECONDS * COUNTDOWN_START_SUBSTEPS).rev() {
            if i % COUNTDOWN_START_SUBSTEPS == 0 {
                print!("{}", i / COUNTDOWN_START_SUBSTEPS);
            } else {
                print!(".");
            }
            cout.flush().unwrap();
            tokio::time::sleep(Duration::from_millis(
                1000 / COUNTDOWN_START_SUBSTEPS as u64,
            ))
            .await;
        }
        println!("\nStarting now");
        return true;
    } else {
        return false;
    }
}

pub fn verify_args(args: &Args) -> Result<()> {
    if args.send_threads == 0 {
        return Err(Error::InvalidConfig(
            "send_threads must be greater than 0".to_string(),
        ));
    }
    if args.host.is_none() && args.target.is_none() {
        return Err(Error::InvalidConfig(
            "host or target must be specified".to_string(),
        ));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    if !usage_warn().await {
        return Ok(());
    }

    println!("Loading config");
    let config = Config::load().unwrap_or_else(
    |e| {
            eprintln!("Failed to load config:\n{}", e.to_string().red());
            eprintln!(
                "Edit the config file at [{}] to fix the problem.", 
                paths::config_file().to_str().unwrap().cyan()
            );
            eprintln!(
                "If you recently updated tsunami, you may need to add missing fields to the config file. See the latest README for details."
            );
            std::process::exit(1);

        });

    println!("Finished loading config");

    let mut args = config.args.clone().merge_clap();
    verify_args(&args)?;

    match &args.target {
        Some(target) => {
            let target = config.targets.get(target).unwrap_or_else(|| {
                eprintln!("Target '{}' not found in config", target);
                std::process::exit(1);
            });

            args.host = Some(target.host.clone());
            args.protocol = target.protocol.clone();
            args.canvas = target.canvas;
        }
        None => {}
    }

    let context = Context { args };
    let host = context.args.host.clone().unwrap();
    let protocol = context.args.protocol.clone();
    let canvas = context.args.canvas.clone();

    let mut handles = vec![];
    let threads = context.args.send_threads.clone();
    println!("Spawning threads");
    for thread in 0..threads {
        let mut socket = TcpStream::connect(host.clone()).await?;
        println!("Thread {} connected", thread);
        handles.push(tokio::spawn(async move {
            let (reader, writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut writer = BufWriter::new(writer);
            let size = protocol
                .preamble(&mut writer, &mut reader, canvas)
                .await
                .unwrap();
            let mut frames: u64 = 0;
            println!("Thread {} got canvas size ({}, {})", thread, size.x, size.y);
            loop {
                let color = Color::RGB24(random(), random(), random());
                match protocol.send_frame(&mut writer, canvas, color, &size).await {
                    Ok(_) => frames += 1,
                    Err(_) => return frames,
                }
            }
        }));
    }
    println!("Spawned threads");

    for handle in handles {
        match handle.await {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }

    return Ok(());
}
