#[cfg(test)]
mod lib_tests {
    use super::*;

    use std::error::Error;
    use std::io;
    use std::process;
    use std::fs::File;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Record {
        city: String,
        region: String,
        country: String,
        population: Option<u64>,
    }

    fn example_file(file_path: &str) -> anyhow::Result<()> {
        // let mut rdr = csv::Reader::from_reader(io::stdin());
        let file = File::open(file_path)?;
        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let record: Record = result?;
            println!("{:?}", record);
        }
        Ok(())
    }

    #[test]
    fn example_file_works() -> anyhow::Result<()> {
        example_file("./fixtures/smallpop.csv")?;

        Ok(())
    }
}



