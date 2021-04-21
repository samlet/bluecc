use std::path::PathBuf;
use deles::delegators::pretty;

pub fn convert_seed_file(file:&str, format:&str) -> meta_gen::Result<()>{
    use std::fs;
    let path = PathBuf::from(file);

    let cnt = fs::read_to_string(path.as_path())?;
    seed::load_all_entities()?;
    let rs = meta_gen::process_seed(cnt.as_str())?;
    // println!("{}", pretty(&rs));
    let root_dir=PathBuf::from(".store");
    let output: PathBuf = root_dir.join(path.file_name().unwrap())
        .with_extension(format);
    println!("export to: {}", output.display());
    match format {
        "json" => fs::write(output.as_path(), pretty(&rs))?,
        "yaml" => {
            let yaml_str = serde_yaml::to_string(&rs)?;
            fs::write(output.as_path(), yaml_str)?
        },
        _ => {
            println!("don't support format {}", format);
        }
    }

    Ok(())
}
