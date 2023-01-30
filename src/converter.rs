use crate::error::Error;
use crate::magnification::Magnification;
use crate::quantity::IQuantityFactory;
use crate::quantity::Quantity;
use crate::quantity::QuantityAmount;
use crate::rate::Rate;
use crate::rate::RateResult;
use crate::unit::UnitId;

use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Converter {
    from: UnitId,
    to: BTreeSet<Rate>,
}
impl Converter {
    pub fn new(from: UnitId) -> Self {
        let mut tree = Self {
            from: from,
            to: BTreeSet::new(),
        };
        tree.to
            .insert(Rate::new(from, from, Magnification::new(1.0))); // #1
        tree
    }
    pub fn already_added(&self, to: UnitId) -> bool {
        let meaningless = 1.0;
        let check = Rate::new(self.from, to, Magnification::new(meaningless));
        self.to.contains(&check)
    }
    pub fn add_or_update(
        &mut self,
        to: UnitId,
        magnification: impl Into<Magnification>,
    ) -> RateResult<()> {
        let magnification = magnification.into();
        if magnification == Magnification::new(0.0) {
            return Err(Error::DivideByZero);
        }
        if self.from == to {
            return Err(Error::AlreadyAdded);
        }
        let new_rate = Rate::new(self.from, to, magnification);
        // add
        if !self.to.insert(new_rate.clone()) {
            // or update
            self.to.remove(&new_rate);
            self.to.insert(new_rate);
        }
        Ok(())
    }
    pub fn remove(&mut self, to: UnitId) -> RateResult<()> {
        let meaningless = 1.0;
        let maybe_saved_rate = Rate::new(self.from, to, Magnification::new(meaningless));
        if !self.to.remove(&maybe_saved_rate) {
            return Err(Error::NotAdded(self.from, to));
        }
        Ok(())
    }
    #[allow(dead_code)]
    pub fn magnification(&self, from: UnitId, to: UnitId) -> RateResult<Magnification> {
        let meaningless = 1.0;
        let base_from = self
            .to
            .get(&Rate::new(self.from, from, Magnification::new(meaningless)));
        let base_to = self
            .to
            .get(&Rate::new(self.from, to, Magnification::new(meaningless)));
        if base_from.is_none() || base_to.is_none() {
            return Err(Error::NotAdded(from, to));
        }
        // #1 の処理によりself.from -> fromの変換およびself.from -> toの変換が同一だった場合
        // magnificationは1.0倍となる
        if let (Some(from), Some(to)) = (base_from, base_to) {
            if from.is_zero() {
                return Err(Error::DivideByZero);
            }
            return to.devide_magnification(from);
        }
        todo!()
    }
    #[allow(dead_code)]
    pub fn convert(
        &self,
        quantity_factory: &dyn IQuantityFactory,
        from: Quantity,
        to: UnitId,
    ) -> RateResult<Quantity> {
        let Quantity {
            // association: association,
            amount,
            unit_id: from,
            ..
        } = from;
        let result = self.magnification(from, to).and_then(|magnification| {
            quantity_factory
                .create(
                    // association,
                    QuantityAmount::new(magnification.value) * amount,
                    to,
                )
                .ok_or(Error::FactoryError)
        });
        result
    }
}

#[allow(unused_imports)]
use speculate::speculate;
#[cfg(test)]
speculate! {
    use crate::unit::Unit;
    use crate::unit::UnitId;
    use crate::error::Error;
    use crate::converter::Converter;
    use crate::quantity::Quantity;
    use crate::quantity::QuantityId;
    use crate::quantity::QuantityAmount;
    use crate::quantity::IQuantityFactory;
    struct QuantityFactory;
    impl IQuantityFactory for QuantityFactory {
        fn create(&self, amount: QuantityAmount, unit: UnitId) -> Option<Quantity> {
            Some(Quantity::new(QuantityId::new(0), amount, unit))
        }
    }
    context "usage" {
        it "create and use" {
            let factory = QuantityFactory;
            let from = Unit::Grams.into();
            let to = Unit::Pieces.into();
            let mut converter = Converter::new(from);
            let _ = converter.add_or_update(to, Magnification::new(1.5));
            let quantity = factory.create(QuantityAmount::new(20.), from).unwrap();
            assert_eq!(Some(factory.create(QuantityAmount::new(30.), to).unwrap()), converter.convert(&factory, quantity, to).ok());
        }
    }

    context "with Unit creating" {
        before {
            let from = Unit::Grams.into();
            #[allow(unused_variables)]
            let to1 = Unit::Pieces.into();
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let mut converter = Converter::new(from);
        }
        it "root" {
            let magnification = converter.magnification(from, from);
            assert_eq!(Ok(Magnification::new(1.0)), magnification);
            assert_eq!(1, converter.to.len());

            let mistakes = Magnification::new(2.0);
            let result = converter.add_or_update(from, mistakes);
            assert_eq!(Err(Error::AlreadyAdded), result);

            // #1
            let magnification = converter.magnification(from, from);
            assert_eq!(Ok(Magnification::new(1.0)), magnification);
            assert_eq!(1, converter.to.len());

            let magnification = converter.magnification(from, to1);
            assert_eq!(Err(Error::NotAdded(from, to1)), magnification);

            let magnification = converter.magnification(to1, from);
            assert_eq!(Err(Error::NotAdded(to1, from)), magnification);

            let miss = converter
                .to
                .get(&Rate::new(from, from, Magnification::new(1.0)));
            assert_eq!(Some(&Rate::new(from, from, Magnification::new(1.0))), miss);
        }
        context "basic" {
            before {
                let from_to1_magnification = Magnification::new(2.0);
                let _ = converter.add_or_update(to1, from_to1_magnification);
            }
            it "normal" {
                let magnification = converter.magnification(from, to1);
                assert_eq!(Ok(from_to1_magnification), magnification);
                let magnification = converter.magnification(to1, from);
                assert_eq!(Ok(Magnification::new(1.0) / from_to1_magnification), magnification);
            }
            it "equality" {
                let magnification = converter.magnification(from, from);
                assert_eq!(Ok(Magnification::new(1.0)), magnification);
                let magnification = converter.magnification(to1, to1);
                assert_eq!(Ok(Magnification::new(1.0)), magnification);
            }
            it "update" {
                let count_from_and_to1 = 2;
                let magnification = converter.magnification(from, to1);
                assert_eq!(Ok(from_to1_magnification), magnification);
                assert_eq!(count_from_and_to1, converter.to.len());
                let from_to1_magnification = Magnification::new(3.0);
                let _ = converter.add_or_update(to1, from_to1_magnification);
                let magnification = converter.magnification(from, to1);
                assert_eq!(Ok(from_to1_magnification), magnification);
                assert_eq!(count_from_and_to1, converter.to.len());
            }
        }
        context "indirectly" {
            before {
                let to2 = Unit::Lots.into();
            }
            context "from or to is base" {
                before {
                    let from_to1_magnification = Magnification::new(2.0);
                    let from_to2_magnification = Magnification::new(10.0);
                    let _ = converter.add_or_update(to1, from_to1_magnification);
                    let _ = converter.add_or_update(to2, from_to2_magnification);
                }
                it "from" {
                    let magnification = converter.magnification(from, to1);
                    assert_eq!(Ok(from_to1_magnification), magnification);
                    let magnification = converter.magnification(to1, from);
                    assert_eq!(Ok(Magnification::new(1.0) / from_to1_magnification), magnification);
                }
                it "to" {
                    let magnification = converter.magnification(from, to2);
                    assert_eq!(Ok(from_to2_magnification), magnification);
                    let magnification = converter.magnification(to2, from);
                    assert_eq!(Ok(Magnification::new(1.0) / from_to2_magnification), magnification);
                }
            }
            context "addition" {
                it "converter addition" {
                    let from_to_magnification = Magnification::new(2.0);
                    let from_target_magnification = Magnification::new(10.0);
                }
                it "converter addition at the other value" {
                    let mut converter = Converter::new(from);
                    let from_to_magnification = Magnification::new(2.0);
                    let from_target_magnification = Magnification::new(8.0);
                }
                after {
                    let result = converter.add_or_update(to1, from_to_magnification);
                    assert_eq!(Ok(()), result);

                    let result = converter.add_or_update(to2, from_target_magnification);
                    assert_eq!(Ok(()), result);

                    let magnification = converter.magnification(to1, to2);
                    assert_eq!(Ok(from_target_magnification / from_to_magnification), magnification);
                }
            }
        }
        it "converter zero addition" {
            let magnification = Magnification::new(0.0);

            let result = converter.add_or_update(to1, magnification);
            assert_eq!(Err(Error::DivideByZero), result);
        }
        it "remove rate" {
            let magnification = Magnification::new(2.0);

            assert!(!converter.already_added(to1));
            let _ = converter.add_or_update(to1, magnification);
            assert!(converter.already_added(to1));

            let result = converter.remove(to1);
            assert_eq!(Ok(()), result);
            assert!(!converter.already_added(to1));

            let result = converter.remove(to1);
            assert_eq!(Err(Error::NotAdded(from, to1)), result);
        }
    }
}
