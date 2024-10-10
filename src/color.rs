use rand::{distributions::Standard, prelude::Distribution};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Color {
    RGB24(u8, u8, u8),
}

impl Distribution<Color> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Color {
        let (r, g, b) = rng.gen();
        Color::RGB24(r, g, b)
    }
}
