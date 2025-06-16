trait Shape {
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
}

struct Triangle {
    sides_lens: [f64; 3],
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Triangle {
    fn get_area(&self) -> f64 {
        let [a, b, c] = self.sides_lens;
        let p = (a + b + c) / 2.0;
        (p * (p - a) * (p - b) * (p - c)).sqrt()
    }

    fn get_perimeter(&self) -> f64 {
        self.sides_lens.iter().sum()
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn get_perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)
    }

    fn get_perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

fn perimeter_by_area(shape: Box<dyn Shape>) -> f64 {
    shape.get_perimeter() / shape.get_area()
}
fn main() {
    let triangle = Box::new(Triangle {
        sides_lens: [3.0, 4.0, 5.0],
    });
    let rectangle = Box::new(Rectangle {
        width: 2.0,
        height: 3.0,
    });
    let circle = Box::new(Circle { radius: 2.0 });

    println!(
        "Triangle perimeter by area: {}",
        perimeter_by_area(triangle)
    );
    println!(
        "Rectangle perimeter by area: {}",
        perimeter_by_area(rectangle)
    );
    println!("Circle perimeter by area: {}", perimeter_by_area(circle));
}
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test() {
        assert_relative_eq!(
            perimeter_by_area(Box::new(Triangle {
                sides_lens: [3.0, 4.0, 5.0]
            })),
            2.0
        );
        assert_relative_eq!(
            perimeter_by_area(Box::new(Circle { radius: 2.0 })),
            1.0
        );
        assert_relative_eq!(
            perimeter_by_area(Box::new(Rectangle {
                width: 2.0,
                height: 3.0,
            })),
            1.6666666666666667
        );
    }
}
