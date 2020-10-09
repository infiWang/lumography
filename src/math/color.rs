#[derive(Copy, Clone)]
pub struct Color
{
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Copy, Clone)]
pub struct RGB8
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[derive(Copy, Clone)]
pub struct RGB16
{
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

impl Color
{
    pub fn default() -> Self { Color{r: 0.0, g: 0.0, b: 0.0} }
    pub fn new(nr: f64, ng: f64, nb: f64) -> Color
    {
        Color { r: nr, g: ng, b: nb }
    }

    #[inline]
    pub fn tidy(&self) -> Color
    {
        let mut nc: Color = *self;
        if nc.r < 0.0 { nc.r = 0.0 };
        if nc.g < 0.0 { nc.g = 0.0 };
        if nc.b < 0.0 { nc.b = 0.0 };
        if nc.r > 1.0 { nc.r = 1.0 };
        if nc.g > 1.0 { nc.g = 1.0 };
        if nc.b > 1.0 { nc.b = 1.0 };

        nc
    }
    #[inline]
    pub fn makeTidy(&mut self)
    {
        if self.r < 0.0 { self.r = 0.0 };
        if self.g < 0.0 { self.g = 0.0 };
        if self.b < 0.0 { self.b = 0.0 };
        if self.r > 1.0 { self.r = 1.0 };
        if self.g > 1.0 { self.g = 1.0 };
        if self.b > 1.0 { self.b = 1.0 };
    }
}

impl std::ops::Add for Color
{
    type Output = Color;
    #[inline]
    fn add(self, color: Color) -> Color
    {
        Color::new(self.r + color.r, self.g + color.g, self.b + color.b).tidy()
    }
}
impl std::ops::AddAssign for Color
{
    #[inline]
    fn add_assign(&mut self, color: Color)
    {
        *self = Color::new(self.r + color.r, self.g + color.g, self.b + color.b).tidy()
    }
}
impl std::ops::Mul<f64> for Color
{
    type Output = Color;
    #[inline]
    fn mul(self, multiplier: f64) -> Color
    {
        Color::new(self.r * multiplier, self.g * multiplier, self.b * multiplier).tidy()
    }
}
impl std::ops::Mul<Color> for f64
{
    type Output = Color;
    #[inline]
    fn mul(self, color: Color) -> Color
    {
        Color::new(color.r * self, color.g * self, color.b * self).tidy()
    }
}

impl RGB8
{
    pub fn default() -> Self { RGB8{r: 0, g: 0, b: 0} }
    pub fn new(nr: u8, ng: u8, nb: u8) -> RGB8
    {
        RGB8 { r: nr, g: ng, b: nb }
    }
}
impl From<Color> for RGB8
{
    fn from(rgb: Color) -> RGB8
    {
        let mut c: Color = rgb;
        c.tidy();
        RGB8::new((c.r * 255.999).floor() as u8, (c.g * 255.999).floor() as u8, (c.b * 255.999).floor() as u8)
    }
}

impl RGB16
{
    pub fn default() -> Self { RGB16{r: 0, g: 0, b: 0} }
    pub fn new(nr: u16, ng: u16, nb: u16) -> RGB16
    {
        RGB16 { r: nr, g: ng, b: nb }
    }
}