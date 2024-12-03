#[macro_export]
macro_rules! run_puzzle {
    ($label:expr, $func:expr) => {{
        use std::time::Instant;

        let start = Instant::now();
        let result = $func();
        let duration = start.elapsed();

        println!(
            "{} result is {:?} and took: {:?} to execute",
            $label, result, duration
        );
        result
    }};
}
