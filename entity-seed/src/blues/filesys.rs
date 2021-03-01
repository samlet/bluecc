use std::{env, ffi::OsStr, fs::File, io::{prelude::*, Write},
          iter::Iterator, path::{Path, PathBuf}, fs};

use anyhow::Result;
use walkdir::WalkDir;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

const DEFAULT_UNIX_PERMISSIONS: u32 = 0o755;

/// ```
// std::process::exit(
//         match zip_dir(&template_dir, &dst_file, CompressionMethod::Stored) {
//             Ok(_) => {
//                 println!(
//                     "done: {} written to {}",
//                     template_dir.display(),
//                     dst_file.display()
//                 );
//                 0
//             }
//             Err(e) => {
//                 eprintln!("Error: {:?}", e);
//                 1
//             }
//         },
//     );
/// ```
pub fn zip_dir(src_dir: &Path, dst_file: &Path, method: CompressionMethod) -> Result<()> {
    if !src_dir.exists() {
        anyhow::bail!("src_dir '{}' does not exist", src_dir.display());
    }
    if !src_dir.is_dir() {
        anyhow::bail!("src_dir '{}' is not a directory", src_dir.display());
    }

    let file = File::create(dst_file)?;

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter().filter_map(|e| e.ok());

    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(DEFAULT_UNIX_PERMISSIONS);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let mut name = path.strip_prefix(&src_dir)?.to_path_buf();

        // Cargo.toml files cause the folder to excluded from `cargo package` so need to be renamed
        if name.file_name() == Some(OsStr::new("_Cargo.toml")) {
            name.set_file_name("Cargo.toml");
        }

        let file_path = name.as_os_str().to_string_lossy();

        if path.is_file() {
            zip.start_file(file_path, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(file_path, options)?;
        }
    }
    zip.finish()?;

    Ok(())
}

pub fn check_output_dir<P>(name: &str, dir: Option<P>) -> Result<Option<String>>
    where
        P: AsRef<Path>,
{
    if name.contains('-') {
        anyhow::bail!("Names cannot contain hyphens");
    }

    let out_dir = dir
        .map_or(env::current_dir()?, |p| p.as_ref().to_path_buf())
        .join(name);
    if out_dir.join("Cargo.toml").exists() {
        anyhow::bail!("A Cargo package already exists in {}", name);
    }
    if !out_dir.exists() {
        fs::create_dir(&out_dir)?;
    }

    Ok(Some(format!("Created dir {}", name)))
}

pub fn with_tmp_dir<F>(f: F)
    where
        F: FnOnce(&Path) -> anyhow::Result<()>,
{
    let tmp_dir = tempfile::Builder::new()
        .prefix("bluecc.test.")
        .tempdir()
        .expect("temporary directory creation failed");

    // catch test panics in order to clean up temp dir which will be very large
    f(tmp_dir.path()).expect("Error executing test with tmp dir")
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn path_works() {
        let p = PathBuf::from("/test");
        assert_eq!(Path::new("/test"), p.as_path());
    }

    #[test]
    fn env_dir_works() {
        let manifest_dir: PathBuf = env::var("HOME")
            .expect("CARGO_MANIFEST_DIR should be set by cargo")
            .into();
        let template_dir = manifest_dir.join("templates").join("new");
        println!("{:?}", template_dir.display());
    }

    #[test]
    fn rejects_hyphenated_name() {
        with_tmp_dir(|path| {
            let result = check_output_dir("rejects-hyphenated-name", Some(path));
            assert!(result.is_err(), "Should fail");
            assert_eq!(
                result.err().unwrap().to_string(),
                "Names cannot contain hyphens"
            );
            Ok(())
        })
    }
}


