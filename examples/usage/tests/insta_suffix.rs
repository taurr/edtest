#[rstest::rstest]
fn insta_suffix_example() {
    let input = "world";
    // set a snapshot suffix so the snapshot file name includes the input
    edtest::set_snapshot_suffix!("{}", input);

    // snapshot the generated output using an inline snapshot so the test is
    // deterministic on first run and doesn't require checked-in snapshot files.
    let output = format!("hello {}", input);
    insta::assert_snapshot!(output, @"hello world");
}
