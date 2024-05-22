fn main() {
    let rectangle = create_shape!(Rectangle { width: 5.0, height: 10.0 });
    let circle = create_shape!(Circle { radius: 3.0 });

    println!("Rectangle area: {}", rectangle.area());
    println!("Circle area: {}", circle.area());
}
#[macro_export]
macro_rules! create_shape {
    (Rectangle { width: $width:expr, height: $height:expr }) => {
        Rectangle {
            width: $width,
            height: $height,
        }
    };
    (Circle { radius: $radius:expr }) => {
        Circle {
            radius: $radius,
        }
    };
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}