use crate::quantity::Quantity;
use crate::quantity::QuantityAmount;
// use crate::quantity::QuantityAssociation;
use crate::unit::UnitId;

pub trait IQuantityFactory {
    fn create(
        &self,
        // associtation: QuantityAssociation,
        amount: QuantityAmount,
        unit_id: UnitId,
    ) -> Option<Quantity>;
}
