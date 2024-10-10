use tokio::io::AsyncWriteExt;

use crate::{Color, Result};

use super::{CanvasSize, Proto};

pub struct Protocol {
    pub count: u64,
}

impl Proto for Protocol {
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
        self.count += 1;
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
        self.count += 1;
        Ok(())
    }
}
