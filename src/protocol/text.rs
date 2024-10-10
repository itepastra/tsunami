use tokio::io::AsyncWriteExt;
use ufmt::uwriteln;

use crate::{Color, Result};

use super::{CanvasSize, Proto};

pub struct Protocol {
    pub str: String,
    pub count: u64,
}

impl Proto for Protocol {
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
        self.count += 1;
        Ok(())
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
        self.count += 1;
        Ok(())
    }
}
