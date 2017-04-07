#[allow(dead_code)]
fn hello<'a>() -> &'a str {
    "hello"
}

#[test]
fn works() {}

#[test]
fn test_hello() {
    assert_eq!(hello(), "hello");
}

#[allow(dead_code)]
pub mod my_module {
    fn abs(n: i32) -> i32 {
        if n < 0 { -n } else { n }
    }

    pub fn format_abs(x: i32) -> String {
        format!("The absolute value of {} is {}", x, abs(x))
    }

    #[test]
    fn test_abs() {
        assert_eq!(abs(3), 3);
        assert_eq!(abs(-42), 42);
        assert_eq!(abs(0), 0);
    }

    #[test]
    fn test_format_abs() {
        assert_eq!(format_abs(3), "The absolute value of 3 is 3");
        assert_eq!(format_abs(-42), "The absolute value of -42 is 42");
        assert_eq!(format_abs(0), "The absolute value of 0 is 0");
    }

    fn factorial(n: i32) -> i32 {
        // 末尾最適化は今はないっぽい
        // https://www.rust-lang.org/en-US/faq.html#does-rust-do-tail-call-optimization
        fn go(nn: i32, acc: i32) -> i32 {
            if nn <= 0 {
                acc
            } else {
                go(nn - 1, nn * acc)
            }
        }

        go(n, 1)
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(5), 120);
    }

    fn fib(n: i32) -> i32 {
        // TODO: 末尾再帰で書く
        fn go(current: i32) -> i32 {
            match current {
                0 => 0,
                1 => 1,
                _ => go(current - 1) + go(current - 2),
            }
        }

        go(n)
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(10), 55);
    }

    fn find_first(ss: Vec<&str>, key: &str) -> i32 {
        // Rustの関数は外側の変数をキャプチャできないので、
        // 関数/クロージャの引数に入れる必要がある
        fn go(n: usize, ss: Vec<&str>, key: &str) -> i32 {
            if n >= ss.len() {
                -1
            } else if ss[n] == key {
                n as i32
            } else {
                go(n + 1, ss, key)
            }
        }

        go(0, ss, key)
    }

    #[test]
    fn test_find_first() {
        assert_eq!(find_first(vec!["foo", "bar"], "foo"), 0);
        assert_eq!(find_first(vec!["foo", "bar", "baz"], "baz"), 2);
        assert_eq!(find_first(vec!["foo", "bar", "baz"], "bax"), -1);
    }
}
