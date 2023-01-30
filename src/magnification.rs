#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Magnification {
    pub value: f32,
}
impl Magnification {
    pub fn new(magnification: f32) -> Self {
        Self {
            value: magnification,
        }
    }
    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }
}
impl std::ops::Div for Magnification {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.value == 0. {
            panic!("Cannot devide by zero-valued `Magnification`!");
        }
        Self::new(self.value / rhs.value)
    }
}
impl From<f32> for Magnification {
    fn from(from: f32) -> Magnification {
        Magnification::new(from)
    }
}
