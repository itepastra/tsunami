use std::{
    io::{self, Write},
    time::Duration,
};

const COUNTDOWN_START: usize = 5;

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
    const COUNTDOWN_MULTIPLY: usize = 8;
    if line.starts_with("y") {
        print!("Starting in: ");
        for i in (0..=COUNTDOWN_START * COUNTDOWN_MULTIPLY).rev() {
            if i % COUNTDOWN_MULTIPLY == 0 {
                print!("{}", i / COUNTDOWN_MULTIPLY);
            } else {
                print!(".");
            }
            cout.flush().unwrap();
            tokio::time::sleep(Duration::from_millis(1000 / COUNTDOWN_MULTIPLY as u64)).await;
        }
        return true;
    } else {
        return false;
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    if !usage_warn().await {
        return Ok(());
    }

    return Ok(());
}
