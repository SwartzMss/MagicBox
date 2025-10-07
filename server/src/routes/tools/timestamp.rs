use axum::Json;
use serde::{Deserialize, Serialize};
use time::{
    format_description::well_known::Rfc3339,
    macros::format_description,
    Duration,
    OffsetDateTime,
    PrimitiveDateTime,
    UtcOffset,
};

use crate::error::{ApiError, ApiResult};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TimestampReq {
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default = "default_mode")]
    pub mode: TimestampMode,
    #[serde(default)]
    pub unit: TimestampUnit,
    #[serde(default)]
    pub timezone: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TimestampResp {
    pub unix_seconds: i64,
    pub unix_millis: i128,
    pub iso_8601: String,
    pub zone_offset: String,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum TimestampMode {
    Now,
    FromUnix,
    FromIso,
}

impl Default for TimestampMode {
    fn default() -> Self {
        TimestampMode::Now
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum TimestampUnit {
    Seconds,
    Milliseconds,
}

impl Default for TimestampUnit {
    fn default() -> Self {
        TimestampUnit::Seconds
    }
}

fn default_mode() -> TimestampMode {
    TimestampMode::Now
}

pub async fn handle(Json(req): Json<TimestampReq>) -> ApiResult<TimestampResp> {
    let ts = match req.mode {
        TimestampMode::Now => OffsetDateTime::now_utc(),
        TimestampMode::FromUnix => parse_unix(req.value.as_deref(), req.unit)?,
        TimestampMode::FromIso => parse_iso(req.value.as_deref(), req.timezone.as_deref())?,
    };

    let offset = local_offset().unwrap_or_else(|| ts.offset().to_owned());

    let resp = TimestampResp {
        unix_seconds: ts.unix_timestamp(),
        unix_millis: ts.unix_timestamp_nanos() / 1_000_000,
        iso_8601: ts.format(&Rfc3339).unwrap_or_else(|_| "".to_string()),
        zone_offset: format_offset(offset),
    };

    Ok(Json(resp))
}

fn parse_unix(input: Option<&str>, unit: TimestampUnit) -> Result<OffsetDateTime, ApiError> {
    let value = input
        .and_then(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .ok_or_else(|| ApiError::BadRequest("missing input".into()))?;

    let raw: i128 = value
        .parse()
        .map_err(|_| ApiError::BadRequest("invalid unix timestamp".into()))?;

    let (seconds, nanos) = match unit {
        TimestampUnit::Seconds => (raw as i64, 0),
        TimestampUnit::Milliseconds => {
            let seconds = raw / 1_000;
            let millis = raw % 1_000;
            (seconds as i64, (millis as i32) * 1_000_000)
        }
    };

    OffsetDateTime::from_unix_timestamp(seconds)
        .map_err(|_| ApiError::BadRequest("timestamp out of range".into()))
        .map(|dt| dt + Duration::nanoseconds(nanos as i64))
}

fn parse_iso(input: Option<&str>, tz: Option<&str>) -> Result<OffsetDateTime, ApiError> {
    let value = input
        .and_then(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .ok_or_else(|| ApiError::BadRequest("missing input".into()))?;

    if value.contains('T') && (value.contains('Z') || value.contains('+') || value.rfind('-').map(|idx| idx > value.find('T').unwrap_or(0)).unwrap_or(false)) {
        return OffsetDateTime::parse(value, &Rfc3339)
            .map_err(|_| ApiError::BadRequest("invalid ISO-8601 string".into()));
    }

    let normalized = normalize_local_datetime(value);
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let naive = PrimitiveDateTime::parse(&normalized, format)
        .map_err(|_| ApiError::BadRequest("invalid datetime format".into()))?;

    let offset = tz
        .and_then(|label| parse_timezone(label))
        .or_else(local_offset)
        .unwrap_or(UtcOffset::UTC);

    Ok(naive.assume_offset(offset))
}

fn local_offset() -> Option<UtcOffset> {
    UtcOffset::current_local_offset().ok()
}

fn format_offset(offset: UtcOffset) -> String {
    let total_minutes = offset.whole_minutes();
    let hours = total_minutes / 60;
    let minutes = (total_minutes % 60).abs();
    format!("{:+03}:{:02}", hours, minutes)
}

fn normalize_local_datetime(value: &str) -> String {
    let mut s = value.replace('T', " ");
    if s.matches(':').count() == 1 {
        s.push_str(":00");
    }
    s
}

fn parse_timezone(label: &str) -> Option<UtcOffset> {
    let trimmed = label.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("local") {
        return None;
    }
    if trimmed.eq_ignore_ascii_case("utc") {
        return Some(UtcOffset::UTC);
    }

    let sign = if trimmed.starts_with('-') {
        -1
    } else {
        1
    };
    let digits = trimmed.trim_start_matches(['+', '-']);
    let mut parts = digits.split(':');
    let hours = parts
        .next()
        .and_then(|v| v.parse::<i8>().ok())?;
    let minutes = parts
        .next()
        .map(|v| v.parse::<i8>().ok())
        .unwrap_or(Some(0))?;

    UtcOffset::from_hms(sign * hours, sign * minutes, 0).ok()
}
