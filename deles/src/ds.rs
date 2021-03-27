#[cfg(test)]
mod lib_tests {
    use super::*;

    use std::error::Error;
    use std::io;
    use std::process;
    use std::fs::File;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Serialize)]
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

    #[test]
    fn serialize_works() -> anyhow::Result<()> {
        let mut wtr = csv::Writer::from_writer(io::stdout());

        // When writing records with Serde using structs, the header row is written
        // automatically.
        wtr.serialize(Record {
            city: "Southborough".to_string(),
            region: "MA".to_string(),
            country: "United States".to_string(),
            population: Some(9686),
        })?;
        wtr.serialize(Record {
            city: "Northbridge".to_string(),
            region: "MA".to_string(),
            country: "United States".to_string(),
            population: Some(14061),
        })?;
        wtr.flush()?;

        Ok(())
    }
}



