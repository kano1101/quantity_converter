use crate::quantity::Quantity;
use crate::quantity::QuantityId;

pub trait IQuantityRepository: std::fmt::Debug {
    fn add(&self, new_quantity: Quantity) -> Option<Quantity>;
    fn find(&self, id: QuantityId) -> Option<Box<Quantity>>;
    fn all(&self) -> Vec<Quantity>;
    fn update(&self, update_quantity: &Quantity) -> Option<Box<Quantity>>;
    fn remove(&self, id: QuantityId) -> Option<()>;
}
