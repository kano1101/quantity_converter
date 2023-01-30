#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitId {
    value: i32,
}
impl UnitId {
    fn new(value: i32) -> Self {
        Self { value }
    }
}
#[allow(dead_code)]
pub(crate) enum Unit {
    Undefine,
    Lots,
    Yen,
    Grams,
    Pieces,
}
impl From<Unit> for UnitId {
    fn from(from: Unit) -> UnitId {
        match from {
            Unit::Undefine => UnitId::new(0),
            Unit::Lots => UnitId::new(1),
            Unit::Yen => UnitId::new(2),
            Unit::Grams => UnitId::new(3),
            Unit::Pieces => UnitId::new(4),
        }
    }
}
