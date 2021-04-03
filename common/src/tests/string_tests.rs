#[test]
fn trim_leading_zero_works() -> anyhow::Result<()> {
    assert_eq!("01".trim_start_matches('0'), "1");
    assert_eq!("0028".trim_start_matches('0'), "28");
    Ok(())
}
