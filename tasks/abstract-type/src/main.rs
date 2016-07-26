trait Shape {
    fn area(&self) -> i32;

    fn is_shape(&self) -> bool {
        true
    }
}

struct Square {
    side_length: i32,
}

impl Shape for Square {
    fn area(&self) -> i32 {
        self.side_length * self.side_length
    }
}

fn main() {
    let square = Square { side_length: 2 };
    println!("The square's area is: {}", square.area());
}

#[cfg(test)]
mod tests {
    use super::{Square, Shape};

    #[test]
    fn area() {
        let square = Square { side_length: 2 };
        assert_eq!(square.area(), 4);
    }

    #[test]
    fn is_shape() {
        let square = Square { side_length: 2 };
        assert!(square.is_shape())
    }
}
