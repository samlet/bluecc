use tempfile::Builder as TempfileBuilder;

#[test]
fn tempfile_works() {
    let tempdir = TempfileBuilder::new().prefix("").tempdir().unwrap();
    println!("{:?}", tempdir);
}

