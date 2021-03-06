use std::error::Error;

fn simple_err_proc() -> Result<(), Box<dyn Error + Send + Sync>> {
    use std::fmt::{Debug, Display};

    trait Error: Debug + Display {
        /// A short description of the error.
        fn description(&self) -> &str;
        /// The lower level cause of this error, if any.
        fn cause(&self) -> Option<&dyn Error> { None }
    }
    Ok(())
}

#[test]
fn simple() {
    simple_err_proc().unwrap();
}
