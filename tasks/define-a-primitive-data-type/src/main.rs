//! Implements a custom type named `CustomInt`.
//! This type only implements a subset of all traits within `std::ops`.

use std::ops;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
struct CustomInt {
    value: i8,
}

#[derive(Debug)]
enum CustomIntError {
    OutOfBoundsAssn,
}

impl CustomInt {
    fn new(v: u8) -> Result<CustomInt, CustomIntError> {
        if v < 1 || v > 10 {
            Err(CustomIntError::OutOfBoundsAssn)
        } else {
            Ok(CustomInt { value: v as i8 })
        }
    }
}

// custom trait to specify bounds
trait Bounded {
    fn in_bounds(&self);
}

impl Bounded for CustomInt {
    fn in_bounds(&self) {
        if self.value < 1 || self.value > 10 {
            panic(self.value);
        }
        #[cold]
        #[inline(never)]
        fn panic(v: i8) -> ! {
            panic!("CustomInt is out of bounds! {} was value", v);
        }
    }
}

impl ops::Add for CustomInt {
    type Output = CustomInt;
    fn add(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value + rhs.value) };
        rval.in_bounds();
        rval
    }
}

impl ops::Sub for CustomInt {
    type Output = CustomInt;
    fn sub(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value - rhs.value) };
        rval.in_bounds();
        rval
    }
}

impl ops::Mul for CustomInt {
    type Output = CustomInt;
    fn mul(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value * rhs.value) };
        rval.in_bounds();
        rval
    }
}

impl ops::Div for CustomInt {
    type Output = CustomInt;
    fn div(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value / rhs.value) };
        rval.in_bounds();
        rval
    }
}

impl ops::BitAnd for CustomInt {
    type Output = CustomInt;
    fn bitand(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value & rhs.value) };
        rval.in_bounds();
        rval
    }
}

impl ops::BitOr for CustomInt {
    type Output = CustomInt;
    fn bitor(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value | rhs.value) };
        rval.in_bounds();
        rval
    }
}

impl ops::BitXor for CustomInt {
    type Output = CustomInt;
    fn bitxor(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value ^ rhs.value) };
        rval.in_bounds();
        rval
    }
}

fn main() {
    let cint_2: CustomInt = CustomInt::new(2).unwrap();
    let cint_3: CustomInt = CustomInt::new(3).unwrap();
    let cint_4: CustomInt = CustomInt::new(4).unwrap();
    assert_eq!(cint_2 + cint_4, CustomInt { value: 6 });
    assert_eq!(cint_4 - cint_2, CustomInt { value: 2 });
    assert_eq!(cint_4 * cint_2, CustomInt { value: 8 });
    assert_eq!(cint_4 / cint_2, CustomInt { value: 2 });
    assert_eq!(cint_3 & cint_2, CustomInt { value: 2 });
    assert_eq!(cint_3 | cint_2, CustomInt { value: 3 });
    assert_eq!(cint_3 ^ cint_2, CustomInt { value: 1 });
}

#[cfg(test)]
mod tests {
    use super::CustomInt;

    #[test]
    fn add_test() {
        let cint_2: CustomInt = CustomInt::new(2).unwrap();
        let cint_4: CustomInt = CustomInt::new(4).unwrap();
        assert_eq!(CustomInt::new(6).unwrap(), cint_2 + cint_4);
    }

    #[test]
    fn sub_test() {
        let cint_2: CustomInt = CustomInt::new(2).unwrap();
        let cint_4: CustomInt = CustomInt::new(4).unwrap();
        assert_eq!(CustomInt::new(2).unwrap(), cint_4 - cint_2);
    }

    #[test]
    fn mul_test() {
        let cint_2: CustomInt = CustomInt::new(2).unwrap();
        let cint_4: CustomInt = CustomInt::new(4).unwrap();
        assert_eq!(CustomInt::new(8).unwrap(), cint_4 * cint_2);
    }

    #[test]
    fn div_test() {
        let cint_2: CustomInt = CustomInt::new(2).unwrap();
        let cint_4: CustomInt = CustomInt::new(4).unwrap();
        assert_eq!(CustomInt::new(2).unwrap(), cint_4 / cint_2);
    }

    #[test]
    fn and_test() {
        let cint_2: CustomInt = CustomInt::new(2).unwrap();
        let cint_3: CustomInt = CustomInt::new(3).unwrap();
        assert_eq!(CustomInt::new(2).unwrap(), cint_3 & cint_2);
    }

    #[test]
    fn or_test() {
        let cint_2: CustomInt = CustomInt::new(2).unwrap();
        let cint_3: CustomInt = CustomInt::new(3).unwrap();
        assert_eq!(CustomInt::new(3).unwrap(), cint_3 | cint_2);
    }

    #[test]
    fn xor_test() {
        let cint_2: CustomInt = CustomInt::new(2).unwrap();
        let cint_3: CustomInt = CustomInt::new(3).unwrap();
        assert_eq!(CustomInt::new(1).unwrap(), cint_3 ^ cint_2);
    }

    #[test]
    fn assn_out_of_bounds_test() {
        let cint_error = CustomInt::new(0);
        assert!(cint_error.is_err());
    }

    #[test]
    #[should_panic(expected = "CustomInt is out of bounds! 11 was value")]
    fn above_out_of_bounds_test() {
        let cint_10: CustomInt = CustomInt::new(10).unwrap();
        let cint_1: CustomInt = CustomInt::new(1).unwrap();
        let _ = cint_10 + cint_1; // should panic here
    }

    #[test]
    #[should_panic(expected = "CustomInt is out of bounds! -9 was value")]
    fn below_out_of_bounds_test() {
        let cint_1: CustomInt = CustomInt::new(1).unwrap();
        let cint_10: CustomInt = CustomInt::new(10).unwrap();
        let _ = cint_1 - cint_10; // should panic here
    }
}
