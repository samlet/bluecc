use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::GenericError;
use seed::{EntityGenerator, ModelField};

trait ResourceGenerator{
    fn write_to(buffer: &dyn Write) -> Result<(), GenericError>;
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use chrono::{DateTime, Utc};

    const PROJ_ROOT: &'static str = "../../deles";
    const RESOURCE_ROOT: &'static str = "../../deles/src/resources";

    #[test]
    fn write_tmp_works() -> anyhow::Result<()> {
        let path = Path::new(PROJ_ROOT);
        let mut buffer = File::create(path.join(".store/foo.txt"))?;
        writeln!(buffer, "just a test")?;
        Ok(())
    }

    #[test]
    fn srv_ent_works() -> anyhow::Result<()> {
        let ent="Example";
        let mut generator = EntityGenerator::new(vec![ent.to_string()]);
        generator.tera.add_raw_template("srv_ent", include_str!("incls/srv_ent.j2"))?;
        let result=generator.entity_gen_works(ent, "srv_ent")?;
        println!("{}", result);
        Ok(())
    }


}


