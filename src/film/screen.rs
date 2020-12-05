use crate::math::*;

pub struct Screen
{
    pub x: u16,
    pub y: u16,
}

impl Screen
{
    pub fn default() -> Screen { Screen{ x: 640, y: 480 } }
    pub fn new(n_x: u16, n_y: u16) -> Screen { Screen{ x: n_x, y: n_y } }

    pub fn ratio(&self) -> f64 { self.x as f64 / self.y as f64 }
    #[inline] pub fn get_vec(&self, pixel: Pos2) -> Vec2
    {
        let horizontal = Vec2::i() * 256f64; let vertical = Vec2::j() * 256f64 / self.ratio();
        let pos_upper_left_corner: Pos2 = Pos2::new(0f64, 0f64) - horizontal/2.0 + vertical/2.0;

        return (pos_upper_left_corner + (pixel.x as f64/self.x as f64)*horizontal - (pixel.y as f64/self.y as f64)*vertical) - Pos2::default();
    }
}