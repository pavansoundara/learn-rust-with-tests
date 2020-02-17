fn add_integers(v1: i32, v2: i32) -> i32 {
    return v1 + v2;
}

fn main() {
    println!("{}", add_integers(1, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_integers() {
        assert_eq!(add_integers(2, 2), 4);
    }
}
