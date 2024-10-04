use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use toml::from_str;

use crate::{Color, Result};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Protocol {
    Plaintext,
    BinFlurry,
}

pub struct CanvasSize {
    pub x: u16,
    pub y: u16,
}

impl Protocol {
    pub async fn preamble<
        W: AsyncWriteExt + std::marker::Unpin,
        R: AsyncReadExt + AsyncBufReadExt + std::marker::Unpin,
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

                let size: Vec<u16> = line
                    .split(' ')
                    .map(|part| from_str::<u16>(part).unwrap())
                    .take(2)
                    .collect();
                return Ok(CanvasSize {
                    x: size[0],
                    y: size[1],
                });
            }
            Protocol::BinFlurry => {
                const SIZE_BIN: u8 = 115;
                writer.write_all(b"PROTOCOL binary\n").await?;
                writer.write_all(&[SIZE_BIN, canvas]).await?;
                writer.flush().await?;
                let x = reader.read_u16_le().await?;
                let y = reader.read_u16_le().await?;
                return Ok(CanvasSize { x, y });
            }
        }
    }

    pub async fn send_frame<W: AsyncWriteExt + std::marker::Unpin>(
        &self,
        writer: &mut W,
        canvas: u8,
        color: Color,
        size: &CanvasSize,
    ) -> Result<()> {
        let Color::RGB24(r, g, b) = color;
        let CanvasSize { x, y } = size;
        match self {
            Protocol::Plaintext => {
                writer
                    .write_all(format!("PX {} {} {:02X}{:02X}{:02X}\n", x, y, r, g, b).as_bytes())
                    .await?;
                return Ok(());
            }
            Protocol::BinFlurry => {
                const SET_PX_RGB_BIN: u8 = 0x80;
                writer
                    .write_all(&[
                        SET_PX_RGB_BIN,
                        canvas,
                        x.to_le_bytes()[0],
                        x.to_le_bytes()[1],
                        y.to_le_bytes()[0],
                        y.to_le_bytes()[1],
                        r,
                        g,
                        b,
                    ])
                    .await?;
                return Ok(());
            }
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::Plaintext
    }
}
