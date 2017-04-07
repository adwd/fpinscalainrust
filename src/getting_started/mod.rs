fn hello<'a>() -> &'a str {
    "hello"
}

#[test]
fn works() {}

#[test]
fn test_hello() {
    assert_eq!(hello(), "hello");
}

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
}
