trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return std::f64::consts::PI * self.radius * self.radius;
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        return 0.5 * (self.base * self.height);
    }
}

fn print_area<T: Shape>(shape: T) {
    println!("{}", shape.area());
}

fn main() {
    print_area(Rectangle{ width: 10.0, height: 20.0});
    print_area(Circle{radius: 10.0});
    print_area(Triangle{base: 10.0, height: 20.0});
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle() {
        assert_eq!(Rectangle{ width: 10.0, height: 10.0}.area(), 100.0);
    }

    #[test]
    fn test_circle() {
        assert_eq!(Circle{radius: 10.0}.area(), 314.1592653589793);
    }

    #[test]
    fn test_triangle() {
        assert_eq!(Triangle{base: 10.0, height: 20.0}.area(), 100.0);
    }
}