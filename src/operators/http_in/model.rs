use std::{collections::HashMap, path::Display};

pub(crate) struct TempRedirectUrl {
    pub(self) url: String,
    pub(self) query_string: String,
}

impl Into<TempRedirectUrl> for &serde_json::Value {
    fn into(self) -> TempRedirectUrl {
        let query_data = self.get("query").unwrap();

        let query_string = serde_html_form::to_string(query_data).unwrap_or("".to_string());
        let url = self
            .get("url")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        TempRedirectUrl { url, query_string }
    }
}

impl ToString for TempRedirectUrl {
    fn to_string(&self) -> String {
        format!("{}?{}", self.url, self.query_string)
    }
}
