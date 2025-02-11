use bptest::{serial, test};
use tracing::*;

#[test]
async fn async_value_test(
    #[values(0, 1, 2, 3, 4, 5, 6, 7, 8, 9)] a: u32,
    #[values(0, 1, 2, 3, 4, 5, 6, 7, 8, 9)] b: u32,
) {
    trace!("adding");
    let ab = a + b;
    let ba = b + a;
    warn!(ab, ba);
    assert_eq!(ab, ba);

    bptest::assert_cfg!(test);
}

proptest::proptest! {
    #[test]
    #[serial]
    fn do_proptest(some_a in 0u32..10, b in 0u32..10) {
        let a = some_a;
        //trace!("waiting just to be async");
        //tokio::time::sleep(Duration::from_millis(100)).await;
        debug!("done waiting");

        let ab = add(a, b);
        let ba = b + a;
        assert_eq!(ab, ba);
    }
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}
