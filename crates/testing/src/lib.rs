#![cfg_attr(test, feature(coverage_attribute))]

fn add(a: u32, b:u32) -> u32 {
    a + b
}


#[cfg(test)]
mod tests {
    use rsttl_macros::test;
    use tracing::*;

    #[test]
    #[coverage(off)]
    async fn async_is_transitive(#[values(0, 1, 2, 3, 4, 5, 6, 7, 8, 9)] a: u32, #[values(0, 1, 2, 3, 4, 5, 6, 7, 8, 9)] b: u32) {
        trace!("adding");
        let ab = a + b;
        let ba = b + a;
        warn!(ab, ba);
        assert_eq!(ab, ba);
    }

    proptest::proptest! {
        #[test]
        fn is_transitive(some_a in 0u32..10, b in 0u32..10) {
            let a = some_a;
            //trace!("waiting just to be async");
            //tokio::time::sleep(Duration::from_millis(100)).await;
            debug!("done waiting");

            let ab = super::add(a, b);
            let ba = b + a;
            assert_eq!(ab, ba);
        }
    }
}
