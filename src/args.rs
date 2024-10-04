use clap_serde_derive::ClapSerde;
use serde::{Deserialize, Serialize};

use crate::Protocol;

#[derive(ClapSerde, Clone, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target section from config file to use
    #[clap(long)]
    #[serde(skip_serializing)]
    pub target: Option<String>,

    /// Host to connect to
    #[clap(long)]
    #[serde(skip_serializing)]
    pub host: Option<String>,

    /// Protocol to use for sending frames
    #[clap(long)]
    #[serde(default)]
    pub protocol: Protocol,

    /// Target canvas (if supported)
    #[clap(long)]
    #[serde(default)]
    pub canvas: u8,

    /// Horizontal offset (in px)
    #[clap(short)]
    #[serde(default)]
    pub x_offset: usize,

    /// Vertical offset (in px)
    #[clap(short)]
    #[serde(default)]
    pub y_offset: usize,

    /// Width (in px) [default: same as source]
    #[clap(long)]
    pub width: Option<u16>,

    /// Height (in px) [default: same as source]
    #[clap(long)]
    pub height: Option<u16>,

    /// Number of threads to use for sending pixels
    #[clap(long)]
    pub send_threads: usize,

    /// Enable debug output
    #[clap(long, action=clap::ArgAction::SetTrue)]
    #[serde(default)]
    pub debug: bool,
}

impl Args {
    pub fn config_default() -> Self {
        Self {
            host: None,
            target: None,
            x_offset: 0,
            y_offset: 0,
            width: None,
            height: None,
            protocol: Protocol::default(),
            canvas: 0,
            debug: false,
            send_threads: 4,
        }
    }
}
