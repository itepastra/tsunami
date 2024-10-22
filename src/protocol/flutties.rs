use std::time::Duration;

use tokio::{io::AsyncWriteExt, time::interval};

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
        let set_px_rgb_bin: u8 = 176 + canvas;
        let mut intrval = interval(Duration::from_millis(1));
        for j in 0..*y {
            for i in 0..*x {
                intrval.tick().await;
                writer
                    .write_all(&[
                        set_px_rgb_bin,
                        i.to_le_bytes()[0],
                        i.to_le_bytes()[1],
                        j.to_le_bytes()[0],
                        j.to_le_bytes()[1],
                        r,
                        g,
                        b,
                    ])
                    .await?;
                writer.flush().await?;
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
        let get_px_bin: u8 = 128 + canvas;
        for j in 0..*y {
            for i in 0..*x {
                writer
                    .write_all(&[
                        get_px_bin,
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
