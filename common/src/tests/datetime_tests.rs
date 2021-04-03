use chrono::prelude::*;
use chrono::offset::LocalResult;

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.f";

#[test]
fn utc_format_works() -> anyhow::Result<()> {
    let s="2008-04-23 16:49:27.392";
    let dt:DateTime<Utc>=Utc.datetime_from_str(s, FORMAT)?;
    println!("{}", dt);

    let s = format!("{}", dt.format(FORMAT));
    println!("{}, {}", s, dt.to_rfc3339_opts(SecondsFormat::Millis, false));
    Ok(())
}

#[test]
fn utc_works() -> anyhow::Result<()> {
    use chrono::{DateTime, FixedOffset, SecondsFormat, TimeZone, Utc};

    let dt = Utc.ymd(2018, 1, 26).and_hms_micro(18, 30, 9, 453_829);
    assert_eq!(dt.to_rfc3339_opts(SecondsFormat::Millis, false),
               "2018-01-26T18:30:09.453+00:00");
    assert_eq!(dt.to_rfc3339_opts(SecondsFormat::Millis, true),
               "2018-01-26T18:30:09.453Z");
    assert_eq!(dt.to_rfc3339_opts(SecondsFormat::Secs, true),
               "2018-01-26T18:30:09Z");

    let pst = FixedOffset::east(8 * 60 * 60);
    let dt = pst.ymd(2018, 1, 26).and_hms_micro(10, 30, 9, 453_829);
    assert_eq!(dt.to_rfc3339_opts(SecondsFormat::Secs, true),
               "2018-01-26T10:30:09+08:00");

    Ok(())
}

#[test]
fn datetime_works() {
    let d = NaiveDate::from_ymd(2015, 6, 3);
    let t = NaiveTime::from_hms_milli(12, 34, 56, 789);

    let dt = NaiveDateTime::new(d, t);
    assert_eq!(dt.date(), d);
    assert_eq!(dt.time(), t);
}

#[test]
fn datetime_ops_works() {
    let dt = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11); // `2014-07-08T09:10:11Z`
    // July 8 is 188th day of the year 2014 (`o` for "ordinal")
    assert_eq!(dt, Utc.yo(2014, 189).and_hms(9, 10, 11));
    // July 8 is Tuesday in ISO week 28 of the year 2014.
    assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms(9, 10, 11));

    let dt = Utc.ymd(2014, 7, 8).and_hms_milli(9, 10, 11, 12); // `2014-07-08T09:10:11.012Z`
    assert_eq!(dt, Utc.ymd(2014, 7, 8).and_hms_micro(9, 10, 11, 12_000));
    assert_eq!(dt, Utc.ymd(2014, 7, 8).and_hms_nano(9, 10, 11, 12_000_000));

    // dynamic verification
    assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
               LocalResult::Single(Utc.ymd(2014, 7, 8).and_hms(21, 15, 33)));
    assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
    assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

    // other time zone objects can be used to construct a local datetime.
    // obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
    let _ = Local.ymd(2014, 7, 8).and_hms_milli(9, 10, 11, 12);
    let fixed_dt = FixedOffset::east(9 * 3600).ymd(2014, 7, 8).and_hms_milli(18, 10, 11, 12);
    assert_eq!(dt, fixed_dt);
}