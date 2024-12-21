use std::f64::consts::PI;

pub fn exercise_geometric_forms() {
    let square = Shape::Square(Square::new(5.0));
    let circle = Shape::Circle(Circle::new(3.0));
    let ellipse = Shape::Ellipse(Ellipse::new(4.0, 2.0));
    let triangle = Shape::Triangle(Triangle::new(3.0, 4.0, 5.0).unwrap()); 
    let cube = Shape::Cube(Cube::new(2.0));
    let cylinder = Shape::Cylinder(Cylinder::new(2.0, 4.0));
    let sphere = Shape::Sphere(Sphere::new(3.0));
    let shapes = vec![&square, &circle, &ellipse, &triangle, &cube, &cylinder, &sphere];

    for shape in shapes {
        println!("\nShape: {:?}", shape);
        println!("Area/Surface Area: {}", shape.calculate_area());
        
        if let Some(perimeter) = shape.calculate_perimeter() {
            println!("Perimeter: {}", perimeter);
        }
        
        if let Some(volume) = shape.calculate_volume() {
            println!("Volume: {}", volume);
        }
    }
}

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

impl Square {
    fn new(side: f64) -> Self{
        Self {side}
    }

    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }

}

impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

impl Ellipse {
    fn new(radius_a: f64, radius_b: f64) -> Self {
        Self { radius_a, radius_b }
    }

    fn area(&self) -> f64 {
        PI * self.radius_a * self.radius_b
    }

    fn perimeter(&self) -> f64 {
        let a = self.radius_a;
        let b = self.radius_b;
        let h = ((a - b) * (a - b)) / ((a + b) * (a + b));
        PI * (a + b) * (1.0 + (3.0 * h) / (10.0 + (4.0 - 3.0 * h).sqrt()))
    }
}

impl Triangle {
    fn new(side_a: f64, side_b: f64, side_c: f64) -> Result<Self, &'static str> {
        if side_a + side_b <= side_c || 
           side_b + side_c <= side_a || 
           side_a + side_c <= side_b {
            return Err("Invalid triangle");
        }
        
        Ok(Self { side_a, side_b, side_c })
    }

    fn area(&self) -> f64 {
        let s = (self.side_a + self.side_b + self.side_c) / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
}

impl Cube {
    fn new(side: f64) -> Self {
        Self { side }
    }

    fn surface_area(&self) -> f64 {
        6.0 * self.side * self.side
    }

    fn volume(&self) -> f64 {
        self.side.powf(3.0)
    }
}

impl Cylinder {
    fn new(radius: f64, height: f64) -> Self {
        Self { radius, height }
    }

    fn surface_area(&self) -> f64 {
        2.0 * PI * self.radius * (self.radius + self.height)
    }

    fn volume(&self) -> f64 {
        PI * self.radius * self.radius * self.height
    }
}

impl Sphere {
    fn new(radius: f64) -> Self {
        Self { radius }
    }

    fn surface_area(&self) -> f64 {
        4.0 * PI * self.radius * self.radius
    }

    fn volume(&self) -> f64 {
        (4.0 / 3.0) * PI * self.radius.powf(3.0)
    }
}

impl Shape {
    fn calculate_area(&self) -> f64 {
        match self {
            Shape::Square(square) => square.area(),
            Shape::Circle(circle) => circle.area(),
            Shape::Ellipse(ellipse) => ellipse.area(),
            Shape::Triangle(triangle) => triangle.area(),
            Shape::Cube(cube) => cube.surface_area(),
            Shape::Cylinder(cylinder) => cylinder.surface_area(),
            Shape::Sphere(sphere) => sphere.surface_area(),
        }
    }

    fn calculate_perimeter(&self) -> Option<f64> {
        match self {
            Shape::Square(square) => Some(square.perimeter()),
            Shape::Circle(circle) => Some(circle.perimeter()),
            Shape::Ellipse(ellipse) => Some(ellipse.perimeter()),
            Shape::Triangle(triangle) => Some(triangle.perimeter()),
            _ => None, 
        }
    }

    fn calculate_volume(&self) -> Option<f64> {
        match self {
            Shape::Cube(cube) => Some(cube.volume()),
            Shape::Cylinder(cylinder) => Some(cylinder.volume()),
            Shape::Sphere(sphere) => Some(sphere.volume()),
            _ => None, 
        }
    }
}
