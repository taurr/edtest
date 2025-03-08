#![allow(unused)]
fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod test {

    use edtest::{fixture, serial, test};
    use tracing::*;

    #[test]
    fn sync_test() {
        info!("Tracing output is captured and part of the test output");
    }

    #[test]
    #[serial]
    async fn async_value_test(
        #[values(0, 1, 2, 3, 4, 5, 6, 7, 8, 9)] a: u32,
        #[values(0, 1, 2, 3, 4, 5, 6, 7, 8, 9)] b: u32,
    ) {
        use edtest::assert_cfg;
        // `static_assertions` are re-exported for convenience
        assert_cfg!(test);

        trace!(a, b);
        let ab = a + b;
        let ba = b + a;
        assert_eq!(ab, ba);

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
