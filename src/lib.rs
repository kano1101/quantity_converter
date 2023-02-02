mod converter;
mod error;
mod magnification;
mod quantity;
mod rate;
mod unit;

pub use converter::*;
pub use error::*;
pub use magnification::*;
pub use quantity::*;
pub use unit::*;

#[allow(dead_code)]
fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
