#[macro_export]
macro_rules! test {
    ($name:ident, $block:block) => {
        #[test]
        fn $name() {
            println!("Starting test: {}", stringify!($name)); // Line 1
            $block // Injected test logic
            println!("Test completed: {}", stringify!($name)); // Line 2
        }
    };
}

