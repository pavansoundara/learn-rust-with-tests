const REPEAT_COUNT: i32 = 6;

fn repeat(c: char) -> String {
    let mut repeated = String::new();
    for _ in 0..REPEAT_COUNT {
        repeated.push(c);
    }
 
    return repeated;
}

fn main() {
    println!("{}", repeat('a'));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat() {
        assert_eq!(repeat('a'), "aaaaaa");
    }
}