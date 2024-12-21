#[derive(Debug)]
struct Square {
    side: f64
}

#[derive(Debug)]
struct Circle {
    radius: f64
}

#[derive(Debug)]
struct Ellipse {
    radius_a: f64,
    radius_b: f64
}

#[derive(Debug)]
struct Triangle {
 base: f64,
    height: f64,
    side_a: f64,
    side_b: f64,
    side_c: f64,
}

#[derive(Debug)]
struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
}

#[derive(Debug)]
struct Cube {
    side: f64,
}

#[derive(Debug)]
struct Cylinder {
    radius: f64,
    height: f64,
}

#[derive(Debug)]
struct Sphere {
    radius: f64,
}

#[derive(Debug)]
enum Shape {
    Square(Square),
    Circle(Circle),
    Ellipse(Ellipse),
    Triangle(Triangle),
    Cube(Cube),
    Cylinder(Cylinder),
    Sphere(Sphere),
}
