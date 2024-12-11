use atoi_radix10::parse_from_str;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

use crate::{Color, Result};

pub mod binary;
pub mod flutties;
pub mod palette;
pub mod text;

macro_rules! build_protocol_mode_enum {
    ($($name:ident: $t:expr,)*) => {

        #[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, Hash)]
        #[serde(rename_all = "kebab-case")]
        pub enum Protocol {
            $($name,)*
        }

        #[macro_export]
        macro_rules! match_parser {
            ($pident:ident: $parser:expr => $f:expr) => (
                match &$parser {
                    $(
                        Protocol::$name => {
                            let mut $pident = $t;
                            loop {
                                $f
                            }
                        },
                    )*
                }
            )
        }
    };
}

build_protocol_mode_enum! {
    Plaintext: text::Protocol{str: String::with_capacity(18), count: 0},
    BinFlurry: binary::Protocol{count: 0},
    BinFlutties: flutties::Protocol{count: 0},
    Palette: palette::Protocol{count: 0},
}

pub trait Proto {
    #[allow(async_fn_in_trait)]
    async fn send_frame<W: AsyncWriteExt + std::marker::Unpin>(
        &mut self,
        writer: &mut W,
        canvas: u8,
        color: Color,
        size: &CanvasSize,
    ) -> Result<()>;

    #[allow(async_fn_in_trait)]
    async fn get_frame<W: AsyncWriteExt + std::marker::Unpin>(
        &mut self,
        writer: &mut W,
        canvas: u8,
        size: &CanvasSize,
    ) -> Result<()>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct CanvasSize {
    pub x: u16,
    pub y: u16,
}

impl Protocol {
    pub async fn preamble<
        W: AsyncWriteExt + std::marker::Unpin,
        R: AsyncBufReadExt + std::marker::Unpin,
    >(
        &self,
        writer: &mut W,
        reader: &mut R,
        canvas: u8,
    ) -> Result<CanvasSize> {
        match self {
            Protocol::Plaintext => {
                writer
                    .write_all(format!("CANVAS {}\nSIZE\n", canvas).as_bytes())
                    .await?;
                writer.flush().await?;
                let mut line = "".to_string();
                reader.read_line(&mut line).await?;

                let mut split = line.trim().split(" ");
                let _command = split.next();
                let x = split.next().unwrap();
                let y = split.next().unwrap();

                let x = parse_from_str(x).unwrap();
                let y = parse_from_str(y).unwrap();

                return Ok(CanvasSize { x, y });
            }
            Protocol::BinFlurry => {
                const SIZE_BIN: u8 = 115;
                writer.write_all(b"PROTOCOL binary\n").await?;
                writer.write_all(&[SIZE_BIN, canvas]).await?;
                writer.flush().await?;
                let x = reader.read_u16().await?;
                let y = reader.read_u16().await?;
                return Ok(CanvasSize { x, y });
            }
            Protocol::BinFlutties => {
                const SIZE_BIN: u8 = 32;
                writer.write_all(&[SIZE_BIN, canvas]).await?;
                writer.flush().await?;
                let x = reader.read_u16().await?;
                let y = reader.read_u16().await?;
                return Ok(CanvasSize { x, y });
            }
            Protocol::Palette => {
                const SIZE_BIN: u8 = 115;
                writer.write_all(b"PROTOCOL palette\n").await?;
                writer.write_all(&[SIZE_BIN, canvas]).await?;
                writer.flush().await?;
                let x = reader.read_u16().await?;
                let y = reader.read_u16().await?;
                return Ok(CanvasSize { x, y });
            }
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::Plaintext
    }
}
