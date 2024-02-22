fn main() {
    let n = 10;
    println!("fib({}) = {}", n, fib(n));
}

fn fib(n: u64) -> u64 {
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_test() {
        let n = 10;
        let expected = 55;
        let actual = fib(n);
        assert_eq!(expected, actual, "fib({}) = {}", n, expected)
    }
}