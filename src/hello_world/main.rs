const SPANISH: &str = "Spanish";
const FRENCH: &str = "French";
const ENGLISH_HELLO_PREFIX: &str = "Hello";
const SPANISH_HELLO_PREFIX: &str = "Hola";
const FRENCH_HELLO_PREFIX: &str = "Bonjour";

fn hello(name: &str, language: &str) -> String {
    let mut _name = name;
    if _name == "" {
        _name = "World";
    }

    return format!("{}, {}!", greeting_prefix(language), _name);
}

fn greeting_prefix(language: &str) -> String {
    let prefix = match language {
        SPANISH => SPANISH_HELLO_PREFIX,
        FRENCH => FRENCH_HELLO_PREFIX,
        _ => ENGLISH_HELLO_PREFIX,
    };

    return prefix.to_string();
}

fn main() {
    println!("{}", hello("", ""));
    println!("{}", hello("Elodie", "Spanish"));
    println!("{}", hello("Chris", ""));
    println!("{}", hello("Louise", "French"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(hello("", ""), "Hello, World!");
    }

    #[test]
    fn test_empty_language() {
        assert_eq!(hello("Chris", ""), "Hello, Chris!");
    }

    #[test]
    fn test_empty_name() {
        assert_eq!(hello("", "Spanish"), "Hola, World!");
    }

    #[test]
    fn test_spanish_language() {
        assert_eq!(hello("Elodie", "Spanish"), "Hola, Elodie!");
    }

    #[test]
    fn test_french_language() {
        assert_eq!(hello("Louise", "French"), "Bonjour, Louise!");
    }
}
