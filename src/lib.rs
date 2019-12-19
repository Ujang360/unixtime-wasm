use chrono::prelude::*;
use chrono::ParseResult;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn unixtime_now() -> i64 {
    Utc::now().timestamp()
}

#[wasm_bindgen]
pub fn unixtime_ms_now() -> i64 {
    Utc::now().timestamp_millis()
}

#[wasm_bindgen]
pub fn unixtime_ns_now() -> i64 {
    Utc::now().timestamp_nanos()
}

#[wasm_bindgen]
pub fn rfc3339_utc_now() -> String {
    Utc::now().to_rfc3339()
}

#[wasm_bindgen]
pub fn rfc3339_local_now() -> String {
    Local::now().to_rfc3339()
}

#[wasm_bindgen]
pub fn get_rfc3339_utc_from_unixtime(unixtime: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(unixtime, 0);
    DateTime::<Utc>::from_utc(naive, Utc).to_rfc3339()
}

#[wasm_bindgen]
pub fn get_rfc3339_utc_from_unixtime_ms(unixtime_ms: i64) -> String {
    let secs = unixtime_ms / 1_000;
    let nsecs = ((unixtime_ms % 1_000) * 1_000_000) as u32;
    let naive = NaiveDateTime::from_timestamp(secs, nsecs);
    DateTime::<Utc>::from_utc(naive, Utc).to_rfc3339()
}

#[wasm_bindgen]
pub fn get_rfc3339_utc_from_unixtime_ns(unixtime_ns: i64) -> String {
    let secs = unixtime_ns / 1_000_000_000;
    let nsecs = (unixtime_ns % 1_000_000_000) as u32;
    let naive = NaiveDateTime::from_timestamp(secs, nsecs);
    DateTime::<Utc>::from_utc(naive, Utc).to_rfc3339()
}

#[wasm_bindgen]
pub fn get_rfc3339_local_from_unixtime(unixtime: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(unixtime, 0);
    Local.from_utc_datetime(&naive).to_rfc3339()
}

#[wasm_bindgen]
pub fn get_rfc3339_local_from_unixtime_ms(unixtime_ms: i64) -> String {
    let secs = unixtime_ms / 1_000;
    let nsecs = ((unixtime_ms % 1_000) * 1_000_000) as u32;
    let naive = NaiveDateTime::from_timestamp(secs, nsecs);
    Local.from_utc_datetime(&naive).to_rfc3339()
}

#[wasm_bindgen]
pub fn get_rfc3339_local_from_unixtime_ns(unixtime_ns: i64) -> String {
    let secs = unixtime_ns / 1_000_000_000;
    let nsecs = (unixtime_ns % 1_000_000_000) as u32;
    let naive = NaiveDateTime::from_timestamp(secs, nsecs);
    Local.from_utc_datetime(&naive).to_rfc3339()
}

#[wasm_bindgen]
pub fn get_unixtime_from_rfc3339(rfc3339: &str) -> i64 {
    match get_dt_from_rfc3339(rfc3339) {
        Ok(dt) => dt.timestamp(),
        Err(_) => 0,
    }
}

#[wasm_bindgen]
pub fn get_unixtime_ms_from_rfc3339(rfc3339: &str) -> i64 {
    match get_dt_from_rfc3339(rfc3339) {
        Ok(dt) => dt.timestamp_millis(),
        Err(_) => 0,
    }
}

#[wasm_bindgen]
pub fn get_unixtime_ns_from_rfc3339(rfc3339: &str) -> i64 {
    match get_dt_from_rfc3339(rfc3339) {
        Ok(dt) => dt.timestamp_nanos(),
        Err(_) => 0,
    }
}

fn get_dt_from_rfc3339(rfc3339: &str) -> ParseResult<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(rfc3339)
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn test_rfc3339_parsing_yield_correct_unixtime_ns() {
        let expected_unixtime_ns = 1_576_727_417_163_850_963;
        let rfc3339_dt = "2019-12-19T10:50:17.163850963+07:00";
        let parse_result = get_unixtime_ns_from_rfc3339(rfc3339_dt);

        assert_eq!(parse_result, expected_unixtime_ns);
    }

    #[test]
    fn test_rfc3339_localtime_generation_from_unixtime_ns_is_correct() {
        let expected_rfc3339_dt = "2019-12-19T10:50:17.163850963+07:00";
        let unixtime_ns = 1_576_727_417_163_850_963;
        let generated_rfc3339_dt = get_rfc3339_local_from_unixtime_ns(unixtime_ns);

        assert_eq!(generated_rfc3339_dt, expected_rfc3339_dt);
    }
}
