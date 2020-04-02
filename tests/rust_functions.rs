#[cfg(test)]
mod tests {
    use backtrace::get_function_name;

    #[test]
    fn basic_test1() {
        assert_eq!(get_function_name("./tests/multithreaded_server", 0xb670).unwrap(), "main");
    }

    #[test]
    fn basic_test2() {
        assert_eq!(get_function_name("./tests/multithreaded_server", 0xb920).unwrap(), "handle_connection");
    }

    #[test]
    fn basic_test3() {
        assert_eq!(get_function_name("./tests/multithreaded_server", 0xb920 + 0x47f).unwrap(), "handle_connection");
    }

    #[test]
    fn basic_test4() {
        assert_eq!(get_function_name("./tests/multithreaded_server", 0xb670 + 0x2ac).unwrap(), "main");
    }
}