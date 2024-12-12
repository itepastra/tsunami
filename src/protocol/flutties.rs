use std::time::Duration;

use rand::Rng;
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

    async fn spray_frame<W: AsyncWriteExt + std::marker::Unpin, R: Rng>(
        &mut self,
        writer: &mut W,
        canvas: u8,
        rng: &mut R,
        size: &CanvasSize,
    ) -> Result<()> {
        let Color::RGB24(r, g, b) = rng.gen();
        let CanvasSize { x, y } = size;
        let set_px_rgb_bin: u8 = 176 + canvas;
        let mut intrval = interval(Duration::from_millis(1));
        for _j in 0..*y {
            for _i in 0..*x {
                intrval.tick().await;
                let lx = rng.gen_range(0..*x);
                let ly = rng.gen_range(0..*y);
                writer
                    .write_all(&[
                        set_px_rgb_bin,
                        lx.to_le_bytes()[0],
                        lx.to_le_bytes()[1],
                        ly.to_le_bytes()[0],
                        ly.to_le_bytes()[1],
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
}
