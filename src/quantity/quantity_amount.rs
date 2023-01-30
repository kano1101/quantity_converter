#[derive(Debug, Clone)]
pub struct QuantityAmount {
    pub value: f32,
}
impl QuantityAmount {
    pub fn new(amount: f32) -> Self {
        Self { value: amount }
    }
}
impl std::ops::Mul for QuantityAmount {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}
