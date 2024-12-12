use rand::Rng;
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
        for j in 0..*y {
            for i in 0..*x {
                uwriteln!(&mut self.str, "PX {} {} {:02X}{:02X}{:02X}", i, j, r, g, b).unwrap();
                writer.write_all(self.str.as_bytes()).await?;
                self.str.clear();
            }
        }
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

    async fn spray_frame<W: AsyncWriteExt + std::marker::Unpin, R: Rng>(
        &mut self,
        writer: &mut W,
        _canvas: u8,
        rng: &mut R,
        size: &CanvasSize,
    ) -> Result<()> {
        let Color::RGB24(r, g, b) = rng.gen();
        let CanvasSize { x, y } = size;
        for _j in 0..*y {
            for _i in 0..*x {
                let lx = rng.gen_range(0..*x);
                let ly = rng.gen_range(0..*y);
                uwriteln!(
                    &mut self.str,
                    "PX {} {} {:02X}{:02X}{:02X}",
                    lx,
                    ly,
                    r,
                    g,
                    b
                )
                .unwrap();
                writer.write_all(self.str.as_bytes()).await?;
                self.str.clear();
            }
        }
        self.count += 1;
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::needless_return)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_frame() {
        let size = CanvasSize { x: 3, y: 2 };
        let mut protocol = Protocol {
            str: String::new(),
            count: 0,
        };
        let color = Color::RGB24(0x34, 0xac, 0x49);

        let mut writer = tokio_test::io::Builder::new()
            .write(b"PX 0 0 34AC49\n")
            .write(b"PX 1 0 34AC49\n")
            .write(b"PX 2 0 34AC49\n")
            .write(b"PX 0 1 34AC49\n")
            .write(b"PX 1 1 34AC49\n")
            .write(b"PX 2 1 34AC49\n")
            .build();

        assert!(protocol
            .send_frame(&mut writer, 0, color, &size)
            .await
            .is_ok());
    }
}
