use chrono::{NaiveDateTime, Datelike};

#[test]
fn dt_works() -> anyhow::Result<()> {
    let created_date =
        NaiveDateTime::parse_from_str("2004-03-04 18:48:34.612",
                                      "%Y-%m-%d %H:%M:%S%.f")?;

    println!("{} - {:?}",  created_date.year(), created_date);
    Ok(())
}

