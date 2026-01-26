use serde_json::Value;

pub fn to_string_opt(v: Option<&Value>) -> Option<String> {
    v.and_then(|val| {
        if val.is_null() {
            None
        } else if let Some(s) = val.as_str() {
            Some(s.to_string())
        } else {
            Some(val.to_string().trim_matches('"').to_string())
        }
    })
}

pub fn to_i64(v: Option<&Value>) -> Option<i64> {
    v.and_then(|val| {
        if val.is_null() {
            None
        } else if let Some(i) = val.as_i64() {
            Some(i)
        } else if let Some(u) = val.as_u64() {
            Some(u as i64)
        } else if let Some(s) = val.as_str() {
            s.parse::<i64>().ok()
        } else {
            None
        }
    })
}

pub fn to_bool(v: Option<&Value>) -> Option<bool> {
    v.and_then(|val| {
        if val.is_null() {
            None
        } else if let Some(b) = val.as_bool() {
            Some(b)
        } else if let Some(s) = val.as_str() {
            match s {
                "true" | "1" => Some(true),
                "false" | "0" => Some(false),
                _ => None,
            }
        } else {
            to_i64(Some(val)).map(|n| n != 0)
        }
    })
}

pub fn to_f64(v: Option<&Value>) -> Option<f64> {
    v.and_then(|val| {
        if val.is_null() {
            None
        } else if let Some(f) = val.as_f64() {
            Some(f)
        } else if let Some(i) = val.as_i64() {
            Some(i as f64)
        } else if let Some(s) = val.as_str() {
            s.parse::<f64>().ok()
        } else {
            None
        }
    })
}
