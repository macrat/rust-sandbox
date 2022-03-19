fn fizzbuzz(i: &i64) -> String {
    match (i % 3, i % 5) {
        (0, 0) => String::from("fizzbuzz"),
        (0, _) => String::from("fizz"),
        (_, 0) => String::from("buzz"),
        _ => format!("{}", i),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        let tests = [
            (1, "1"),
            (3, "fizz"),
            (4, "4"),
            (5, "buzz"),
            (15, "fizzbuzz"),
        ];

        for (input, expect) in tests {
            assert_eq!(fizzbuzz(&input), expect);
        }
    }
}

fn main() {
    for i in 1..30 {
        println!("{}", fizzbuzz(&i))
    }
}
