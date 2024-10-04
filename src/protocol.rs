use std::io::Write as _;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::{Color, Pixel};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Protocol {
    Plaintext,
    BinFlurry,
    SBinFlurry,
}

#[inline]
pub fn pixels_to_cmds(
    protocol: Protocol,
    canvas: u8,
    pixels: &[Pixel],
    offset_x: usize,
    offset_y: usize,
) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(pixels.len() * 8);
    for pixel in pixels {
        let x = (pixel.x + offset_x) as u16;
        let y = (pixel.y + offset_y) as u16;

        protocol.encode(&mut result, canvas, pixel.color, x, y);
    }
    result
}

impl Protocol {
    pub fn preamble(&self, buf: &mut Vec<u8>, canvas: u8) {
        match self {
            Protocol::Plaintext => writeln!(buf, "CANVAS {}", canvas).unwrap(),
            Protocol::BinFlurry => {writeln!(buf, "PROTOCOL binary").unwrap(), buf.push(value);},
            Protocol::SBinFlurry => todo!(),
        }
    }

    pub fn encode(&self, buf: &mut Vec<u8>, canvas: u8, color: Color, x: u16, y: u16) {
        let Color::RGB24(r, g, b) = color;
        match self {
            Protocol::Plaintext => {
                writeln!(buf, "PX {} {} {:02X}{:02X}{:02X}", x, y, r, g, b).unwrap();
            }
            Protocol::SBinFlurry => {
                buf.push(0x80);
                buf.extend_from_slice(&x.to_le_bytes());
                buf.extend_from_slice(&y.to_le_bytes());
                buf.push(r);
                buf.push(g);
                buf.push(b);
            }
            Protocol::BinFlurry => {
                buf.push(0x80);
                buf.push(canvas);
                buf.extend_from_slice(&x.to_le_bytes());
                buf.extend_from_slice(&y.to_le_bytes());
                buf.push(r);
                buf.push(g);
                buf.push(b);
            }
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::Plaintext
    }
}
