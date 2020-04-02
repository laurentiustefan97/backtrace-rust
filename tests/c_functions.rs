#[cfg(test)]
mod tests {
    use backtrace::get_function_name;

    #[test]
    fn basic_test1() {
        assert_eq!(get_function_name("./tests/basic", 0x5fb).unwrap(), "foo");
    }

    #[test]
    fn basic_test2() {
        assert_eq!(get_function_name("./tests/basic", 0x5ff).unwrap(), "foo");
    }

    #[test]
    fn basic_test3() {
        assert_eq!(get_function_name("./tests/basic", 0x601).unwrap(), "bar");
    }

    #[test]
    fn basic_test4() {
        assert_eq!(get_function_name("./tests/basic", 0x605).unwrap(), "bar");
    }

    #[test]
    fn basic_test5() {
        assert_eq!(get_function_name("./tests/basic", 0x608).unwrap(), "tar");
    }

    #[test]
    fn basic_test6() {
        assert_eq!(get_function_name("./tests/basic", 0x60e).unwrap(), "tar");
    }

    #[test]
    fn basic_test7() {
        assert_eq!(get_function_name("./tests/basic", 0x60f).unwrap(), "main");
    }

    #[test]
    fn basic_test8() {
        assert_eq!(get_function_name("./tests/basic", 0x619).unwrap(), "main");
    }

    #[test]
    fn basic_test9() {
        // <__libc_csu_init> function
        assert_eq!(get_function_name("./tests/basic", 0x620).unwrap(), "Name unknown");
    }
}