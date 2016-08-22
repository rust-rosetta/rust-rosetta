#[derive(Clone, Copy, PartialEq, Debug)]
struct Vector {
    x: f32,
    y: f32,
    z: f32
}

impl Vector {
    fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {x: x, y: y, z: z}
    }
}

fn dot(a:Vector, b:Vector) -> f32 {
    a.x*b.x + a.y*b.y + a.z*b.z
}

fn cross(a:Vector, b:Vector) -> Vector {
    Vector{x: {a.y*b.z - a.z*b.y},
        y: {a.z*b.x - a.x*b.z},
        z: {a.x*b.y - a.y*b.x}
    }
}

fn scalar3(a:Vector, b:Vector, c:Vector) -> f32 {
    dot(a, cross(b, c))
}

fn vector3(a:Vector, b:Vector, c:Vector) -> Vector {
    cross(a, cross(b, c))
}

fn main() {
    let a = Vector::new(3.0, 4.0, 5.0);
    let b = Vector::new(4.0, 3.0, 5.0);
    let c = Vector::new(-5.0, -12.0, -13.0);

    println!("Dot product: {}", dot(a,b));
    println!("Cross product: {:?}", cross(a,b));
    println!("Scalar triple product: {}", scalar3(a,b,c));
    println!("Vector triple product: {:?}", vector3(a,b,c));
}

#[test]
fn vector_dot_product() {
    let a = Vector::new(1.0, 3.0, -5.0);
    let b = Vector::new(4.0, -2.0, -1.0);
    assert_eq!(dot(a,b),3.0);
}

#[test]
fn vector_cross_product() {
    let a = Vector::new(2.0, 3.0, 4.0);
    let b = Vector::new(5.0, 6.0, 7.0);
    assert_eq!(cross(a,b),Vector {x:-3.0, y:6.0, z:-3.0});
}
