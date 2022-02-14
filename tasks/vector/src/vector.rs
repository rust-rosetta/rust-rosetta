use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector { x, y }
    }
}

impl Vector<f64> {
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Vector {
            x: r * theta.cos(),
            y: r * theta.sin(),
        }
    }
}

impl<T> fmt::Display for Vector<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "[{:.*}, {:.*}]", precision, self.x, precision, self.y)
        } else {
            write!(f, "[{}, {}]", self.x, self.y)
        }
    }
}

impl<T> ops::Add for Vector<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> ops::Sub for Vector<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> ops::Mul<T> for Vector<T>
where
    T: ops::Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T> ops::Div<T> for Vector<T>
where
    T: ops::Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}
