use std::{io::ErrorKind, time::Duration};

use atoi_radix10::parse_from_str;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWriteExt, BufReader},
    time::timeout,
};
use ufmt::uwriteln;

use crate::{Color, Error, Result};

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
    Plaintext: TextProtocol{str: String::with_capacity(18), count: 0},
    BinFlurry: BinProtocol{count: 0},
}

pub trait Proto {
    async fn send_frame<W: AsyncWriteExt + std::marker::Unpin>(
        &mut self,
        writer: &mut W,
        canvas: u8,
        color: Color,
        size: &CanvasSize,
    ) -> Result<()>;

    async fn get_frame<W: AsyncWriteExt + std::marker::Unpin>(
        &mut self,
        writer: &mut W,
        canvas: u8,
        size: &CanvasSize,
    ) -> Result<()>;
}

pub struct CanvasSize {
    pub x: u16,
    pub y: u16,
}

pub struct TextProtocol {
    pub str: String,
    pub count: u64,
}
pub struct BinProtocol {
    pub count: u64,
}

impl Proto for TextProtocol {
    async fn send_frame<W: AsyncWriteExt + std::marker::Unpin>(
        &mut self,
        writer: &mut W,
        _canvas: u8,
        color: Color,
        size: &CanvasSize,
    ) -> Result<()> {
        let Color::RGB24(r, g, b) = color;
        let CanvasSize { x, y } = size;
        uwriteln!(&mut self.str, "PX {} {} {:02X}{:02X}{:02X}", x, y, r, g, b).unwrap();
        writer.write_all(self.str.as_bytes()).await?;
        self.str.clear();
        return Ok(());
    }

    async fn get_frame<W: AsyncWriteExt + std::marker::Unpin>(
        &mut self,
        writer: &mut W,
        _canvas: u8,
        size: &CanvasSize,
    ) -> Result<()> {
        let CanvasSize { x, y } = size;
        for j in 0..*y {
            for i in 0..*x {
                uwriteln!(&mut self.str, "PX {} {}", i, j).unwrap();
                writer.write_all(self.str.as_bytes()).await?;
            }
        }
        Ok(())
    }
}

impl Proto for BinProtocol {
    async fn send_frame<W: AsyncWriteExt + std::marker::Unpin>(
        &mut self,
        writer: &mut W,
        canvas: u8,
        color: Color,
        size: &CanvasSize,
    ) -> Result<()> {
        let Color::RGB24(r, g, b) = color;
        let CanvasSize { x, y } = size;
        const SET_PX_RGB_BIN: u8 = 0x80;
        for j in 0..*y {
            for i in 0..*x {
                writer
                    .write_all(&[
                        SET_PX_RGB_BIN,
                        canvas,
                        i.to_le_bytes()[0],
                        i.to_le_bytes()[1],
                        j.to_le_bytes()[0],
                        j.to_le_bytes()[1],
                        r,
                        g,
                        b,
                    ])
                    .await?;
            }
        }
        Ok(())
    }

    async fn get_frame<W>(&mut self, writer: &mut W, canvas: u8, size: &CanvasSize) -> Result<()>
    where
        W: AsyncWriteExt + std::marker::Unpin,
    {
        let CanvasSize { x, y } = size;
        const GET_PX_BIN: u8 = 0x20;
        for j in 0..*y {
            for i in 0..*x {
                writer
                    .write_all(&[
                        GET_PX_BIN,
                        canvas,
                        i.to_le_bytes()[0],
                        i.to_le_bytes()[1],
                        j.to_le_bytes()[0],
                        j.to_le_bytes()[1],
                    ])
                    .await?;
            }
        }

        Ok(())
    }
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
                let x = reader.read_u16_le().await?;
                let y = reader.read_u16_le().await?;
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
