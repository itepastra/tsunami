use rand::Rng;
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
        let Color::RGB24(r, _, _) = color;
        let CanvasSize { x, y } = size;
        const SET_PX_PALETTE_BIN: u8 = 0x21;
        for j in 0..*y {
            for i in 0..*x {
                writer
                    .write_all(&[
                        SET_PX_PALETTE_BIN,
                        canvas,
                        i.to_be_bytes()[0],
                        i.to_be_bytes()[1],
                        j.to_be_bytes()[0],
                        j.to_be_bytes()[1],
                        r,
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
                        i.to_be_bytes()[0],
                        i.to_be_bytes()[1],
                        j.to_be_bytes()[0],
                        j.to_be_bytes()[1],
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
        let r = rng.gen();
        let CanvasSize { x, y } = size;
        const SET_PX_PALETTE_BIN: u8 = 0x21;
        for _j in 0..*y {
            for _i in 0..*x {
                let lx = rng.gen_range(0..*x);
                let ly = rng.gen_range(0..*y);
                writer
                    .write_all(&[
                        SET_PX_PALETTE_BIN,
                        canvas,
                        lx.to_be_bytes()[0],
                        lx.to_be_bytes()[1],
                        ly.to_be_bytes()[0],
                        ly.to_be_bytes()[1],
                        r,
                    ])
                    .await?;
            }
        }
        self.count += 1;
        Ok(())
    }
}
