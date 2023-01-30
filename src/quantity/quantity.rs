use crate::quantity::QuantityAmount;
// use crate::quantity::QuantityAssociation;
use crate::quantity::QuantityId;
use crate::unit::UnitId;

#[derive(Debug, Clone)]
pub struct Quantity {
    id: QuantityId,
    // pub association: QuantityAssociation,

    // 必ずQuantityAmount:Unit = 1:1
    // 配合要素（材料及び製品）のそれぞれでの数量の種類は、(円、個、グラム諸々と）可変であるが
    // ひとつのQuantityには1種類の分量を持つだけにし、参照元が外部キーでアクセスするように扱う
    // さらに言うと、配合要素それぞれが持っているQuantityの種類数は一様ではないことへも注意のこと
    pub amount: QuantityAmount,
    pub unit_id: UnitId,
}
impl Quantity {
    pub fn new(
        id: QuantityId,
        // associtation: QuantityAssociation,
        amount: QuantityAmount,
        unit_id: UnitId,
    ) -> Self {
        Self {
            id: id,
            // association: associtation,
            amount: amount,
            unit_id: unit_id,
        }
    }
}
impl PartialEq for Quantity {
    fn eq(&self, other: &Quantity) -> bool {
        self.id == other.id
    }
}
