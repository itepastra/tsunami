use rand::{distr::StandardUniform, prelude::Distribution};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Color {
    RGB24(u8, u8, u8),
}

impl Distribution<Color> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Color {
        let (r, g, b) = rng.random();
        Color::RGB24(r, g, b)
    }
}
