#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuantityId {
    pub value: i32,
}
impl QuantityId {
    pub fn new(id: impl Into<i32>) -> Self {
        Self { value: id.into() }
    }
}
