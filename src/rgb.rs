#[derive(Default)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub fn to_array(self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }
}
