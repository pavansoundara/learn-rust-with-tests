fn add_integers(v1: i32, v2: i32) -> i32 {
    return v1 + v2;
}

fn add_floats(v1: f32, v2: f32) -> f32 {
    return v1 + v2;
}

fn main() {
    println!("{}", add_integers(1, 5));
    println!("{}", add_floats(10.0, 20.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_integers() {
        assert_eq!(add_integers(2, 2), 4);
    }

    #[test]
    fn test_add_floats() {
        assert_eq!(add_floats(10.0, 10.0), 20.0);
    }
}
