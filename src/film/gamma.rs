use crate::math::*;

impl Color
{
    #[inline]
    pub fn gammaCorrection(self, gamma: f64) -> Color
    {
        Color::new(self.r.powf( 1.0/gamma ), self.g.powf( 1.0/gamma ), self.b.powf( 1.0/gamma ))
    }
}