use crate::unit::UnitId;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    AlreadyAdded,
    NotAdded(UnitId, UnitId),
    DivideByZero,
    FactoryError,
}
