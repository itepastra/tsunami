pub enum Color {
    RGB24(u8, u8, u8),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub color: Color,
}
