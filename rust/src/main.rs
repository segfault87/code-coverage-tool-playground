fn romanize(v: i32) -> &'static str {
    match v {
        1 => &"one",
        2 => &"two",
        3 => &"three",
        4 => &"four",
        5 => &"five",
        6 => &"six",
        7 => &"seven",
        _ => panic!("only 1 to 7 is allowed."),
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
