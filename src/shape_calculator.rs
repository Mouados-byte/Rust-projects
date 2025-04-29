// Shape Calculator

use std::f32::consts::PI;

enum Shape {
    Circle{radius: f32},
    Rectangle{width: f32, height: f32},
    Triangle{a: f32, b: f32, c: f32},
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Circle {radius} => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { a, b, c } => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn perimeter(&self) -> f32 {
        match self {
            Shape::Circle { radius } => 2.0 * radius * PI,
            Shape::Rectangle { width, height } => 2.0 * (width + height),
            Shape::Triangle { a, b, c } => a + b + c 
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle {radius: _} => "Circle",
            Shape::Rectangle {width: _, height: _} => "Rectangle",
            Shape::Triangle {a: _, b: _, c: _} => "Triangle",
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle{ radius: 1.0 },
        Shape::Rectangle{ width: 2.0, height: 4.0},
        Shape::Triangle{ a: 5.0, b: 3.0, c: 7.0}
    ];

    for shape in shapes {
        println!("--------------");
        println!("Shape: {}", shape.name());
        println!("Area: {}, Perimeter: {}", shape.area(), shape.perimeter());
    }
}
