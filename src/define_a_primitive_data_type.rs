// http://rosettacode.org/wiki/Define_a_primitive_data_type
// Implements a custom type named CustomInt.
// This type only implements a subset of all traits within std::ops.

mod custom_int {
    use std::ops;

    #[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
    pub struct CustomInt {
        value: u8,
    }

    // constructor function
    pub fn custom_int(v: u8) -> CustomInt {
        let rval = CustomInt { value: v };
        rval.in_bounds();
        rval
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
            fn panic(v: u8) -> ! {
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
}

fn main() {
    use custom_int::*;
    let cint_2 = custom_int(2);
    let cint_3 = custom_int(3);
    let cint_4 = custom_int(4);
    cint_2 + cint_4; // CustomInt { value: 6 }
    cint_4 - cint_2; // CustomInt { value: 2 }
    cint_4 * cint_2; // CustomInt { value: 8 }
    cint_4 / cint_2; // CustomInt { value: 2 }
    cint_3 & cint_2; // CustomInt { value: 2 }
    cint_3 | cint_2; // CustomInt { value: 3 }
    cint_3 ^ cint_2; // CustomInt { value: 1 }
}

#[cfg(test)]
mod tests {
    use custom_int::*;

    #[test]
    fn add_test() {
        let cint_2 = custom_int(2);
        let cint_4 = custom_int(4);
        assert_eq!(custom_int(6), cint_2 + cint_4);
    }

    #[test]
    fn sub_test() {
        let cint_2 = custom_int(2);
        let cint_4 = custom_int(4);
        assert_eq!(custom_int(2), cint_4 - cint_2);
    }

    #[test]
    fn mul_test() {
        let cint_2 = custom_int(2);
        let cint_4 = custom_int(4);
        assert_eq!(custom_int(8), cint_4 * cint_2);
    }

    #[test]
    fn div_test() {
        let cint_2 = custom_int(2);
        let cint_4 = custom_int(4);
        assert_eq!(custom_int(2), cint_4 / cint_2);
    }

    #[test]
    fn and_test() {
        let cint_2 = custom_int(2);
        let cint_3 = custom_int(3);
        assert_eq!(custom_int(2), cint_3 & cint_2);
    }

    #[test]
    fn or_test() {
        let cint_2 = custom_int(2);
        let cint_3 = custom_int(3);
        assert_eq!(custom_int(3), cint_3 | cint_2);
    }

    #[test]
    fn xor_test() {
        let cint_2 = custom_int(2);
        let cint_3 = custom_int(3);
        assert_eq!(custom_int(1), cint_3 ^ cint_2);
    }
}
