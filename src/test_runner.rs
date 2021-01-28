#[cfg(test)]
pub fn run_all_tests(tests: &[&dyn Fn()]) {
    use crate::println;

    println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }
}
