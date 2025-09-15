use axum::http::HeaderValue;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::cookie::Expiration;
use axum_extra::extract::cookie::SameSite;
use chrono::Days;
use hyper::HeaderMap;
use saasexpress_core::timestamp::NaiveDateTimeExt;
use saasexpress_core::timestamp::now;
use serde_json::Value;
use std::fmt::Error;
use std::sync::{Arc, Mutex};
use time::{Duration, OffsetDateTime};

pub(super) fn set_cookies(temp: Arc<Mutex<Value>>) -> HeaderMap {
    let mut headers = HeaderMap::new();

    let new_cookies = get_new_cookies(temp);

    if new_cookies.is_err() {
        return headers;
    }
    new_cookies.unwrap().iter().for_each(|cookie| {
        let cookie_str = get_cookie(cookie);
        headers.append("Set-Cookie", HeaderValue::from_str(&cookie_str).unwrap());
    });

    return headers;
}

fn get_cookie(cookie: &Value) -> String {
    let name = cookie.get("name").unwrap().as_str().unwrap();
    let value = cookie.get("value").unwrap().as_str().unwrap();
    let path = cookie.get("path").unwrap().as_str().unwrap();
    let domain = cookie.get("domain").unwrap().as_str().unwrap();
    let same_site = cookie.get("same_site").unwrap().as_str().unwrap();
    let http_only = cookie.get("http_only").unwrap().as_bool().unwrap();
    let secure = cookie.get("secure").unwrap().as_bool().unwrap();
    let expire_hours = cookie.get("expire_hours").unwrap().as_i64().unwrap();

    let now = OffsetDateTime::now_utc();
    let expires = now + Duration::hours(expire_hours);

    let same_site = match same_site {
        "Lax" => SameSite::Lax,
        "Strict" => SameSite::Strict,
        "None" => SameSite::None,
        _ => SameSite::Lax,
    };

    Cookie::build((name, value))
        .path(path)
        .http_only(http_only)
        .secure(secure)
        .same_site(same_site)
        .domain(domain)
        .expires(expires)
        .to_string()
}

#[fastrace::trace]
fn get_new_cookies(temp: Arc<Mutex<Value>>) -> Result<Vec<serde_json::Value>, &'static str> {
    let temp = temp.lock().unwrap();

    if let Some(cookies) = temp.get("http_in") {
        if let Some(cookies) = cookies.get("response") {
            if let Some(cookies) = cookies.get("set-cookies") {
                return Ok(cookies.as_array().unwrap().to_vec());
            } else {
                return Err("No cookies found");
            }
        } else {
            return Err("No cookies found");
        }
    } else {
        return Err("No cookies found");
    }
}
