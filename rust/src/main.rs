fn romanize(v: i32) -> &'static str {
    match v {
        1 => &"one",
        2 => &"two",
        3 => &"three",
        4 => &"four",
        5 => &"five",
        6 => &"six",
        _ => panic!("only 1 to 6 is allowed."),
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::romanize;

    #[test]
    fn test_romanize() {
        assert_eq!(romanize(1), "one");
        assert_ne!(romanize(3), "two");
    }
}
