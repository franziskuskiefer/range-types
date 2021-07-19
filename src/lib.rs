use std::ops::{Add, Div, Mul, Sub};

pub struct Uint8<const MIN: u8, const MAX: u8> {
    val: u8,
}

impl<const MIN: u8, const MAX: u8> Uint8<MIN, MAX> {
    const MIN: u8 = MIN;
    const MAX: u8 = MAX;

    fn check(val: u8) -> Self {
        debug_assert!(
            val >= Self::MIN && val <= Self::MAX,
            "{} <= {} <= {}",
            Self::MIN,
            val,
            Self::MAX
        );
        Self { val }
    }

    pub fn new(val: u8) -> Self {
        Self::check(val)
    }
}

impl<const MIN: u8, const MAX: u8> From<u8> for Uint8<MIN, MAX> {
    fn from(val: u8) -> Self {
        Self::check(val)
    }
}

macro_rules! implement_op {
    ($op:tt, $op_name:ident, $func_op:ident) => {
        impl<const MIN: u8, const MAX: u8> $op_name for Uint8<MIN, MAX> {
            type Output = Self;
            fn $func_op(self, rhs: Self) -> Self {
                let val = self.val $op rhs.val;
                Self::check(val)
            }
        }
    }
}

implement_op!(+, Add, add);
implement_op!(-, Sub, sub);
implement_op!(*, Mul, mul);
implement_op!(/, Div, div);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! range {
        ($name:ident, u8, $min:literal, $max:literal) => {
            type $name = Uint8<$min, $max>;
        };
    }

    #[test]
    fn it_works_macro() {
        range!(Uint8_20_50, u8, 20, 50);
        let _x = Uint8_20_50::new(25);
        let _y = Uint8_20_50::new(30);

        let _x: Uint8_20_50 = 32.into();
    }

    #[test]
    fn it_works() {
        let _x = Uint8::<5, 10>::new(5);
        let _y = Uint8::<5, 10>::new(6);
    }

    #[test]
    #[should_panic]
    fn it_works_no_add() {
        let x = Uint8::<5, 10>::new(5);
        let y = Uint8::<5, 10>::new(6);
        let _r = x + y;
    }

    #[test]
    #[should_panic]
    fn it_works_no_mul() {
        let x = Uint8::<5, 10>::new(5);
        let y = Uint8::<5, 10>::new(5);
        let _r = x * y;
    }

    #[test]
    #[should_panic]
    fn it_works_no_sub() {
        let x = Uint8::<5, 10>::new(9);
        let y = Uint8::<5, 10>::new(5);
        let _r = x - y;
    }

    #[test]
    #[should_panic]
    fn it_works_not() {
        let _x = Uint8::<5, 10>::new(4);
    }
}
