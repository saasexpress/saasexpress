use std::collections::HashMap;

use hyper::Method;
use saasexpress_core::settings::settings::Setting;
use tracing::{debug, info};

pub struct HTTPBuilder {
    builder: reqwest::RequestBuilder,
}

impl HTTPBuilder {
    /// Example:
    /// ```
    /// let url = Builder::derive_url("https://example.com", "/api", "/default", "query=1");
    /// assert_eq!(url, "https://example.com/api?query=1");
    /// ```
    /// ```
    /// let url = Builder::derive_url("https://example.com", "/api", "", "query=1");
    /// assert_eq!(url, "https://example.com/api?query=1");
    /// ```
    /// ```
    /// let url = Builder::derive_url("https://example.com", "", "/default", "query=1");
    /// assert_eq!(url, "https://example.com/default?query=1");
    /// ```
    ///
    pub fn new(method: &str, url: &str) -> Self {
        debug!("--> [{}] {}", method, url);

        HTTPBuilder {
            builder: reqwest::Client::new().request(Method::try_from(method).unwrap(), url),
        }
    }

    #[fastrace::trace(short_name = true)]
    pub async fn send(self) -> Result<reqwest::Response, reqwest::Error> {
        debug!("--> Sending request");
        debug!("Builder : {:?}", self.builder);
        let response = self.builder.send().await;
        debug!("<-- Response received");
        response
    }

    #[fastrace::trace(short_name = true)]
    pub fn set_body(mut self, body: Vec<u8>) -> Self {
        self.builder = self.builder.body(body);
        self
    }

    #[fastrace::trace(short_name = true)]
    pub fn set_headers(mut self, headers: &Vec<Setting>) -> Self {
        for setting in headers.iter() {
            self.builder = self.builder.header(
                setting.key.clone().replace("_", "-").to_lowercase(),
                setting.value.clone(),
            );
        }
        self
    }

    #[fastrace::trace(short_name = true)]
    pub fn set_headers_with_map(mut self, headers: &HashMap<String, String>) -> Self {
        for setting in headers.iter() {
            self.builder = self.builder.header(
                setting.0.clone().replace("_", "-").to_lowercase(),
                setting.1.clone(),
            );
        }
        self
    }

    pub fn set_header(mut self, key: &str, value: &str) -> Self {
        self.builder = self
            .builder
            .header(key.replace("_", "-").to_lowercase(), value);
        self
    }

    pub fn payloads_json(self) -> Self {
        let mut json_headers = Vec::new();
        json_headers.push(Setting {
            key: "Content-Type".to_string(),
            value: "application/json".to_string(),
        });

        json_headers.push(Setting {
            key: "Accept".to_string(),
            value: "application/json".to_string(),
        });
        self.set_headers(&json_headers)
    }

    pub fn derive_url(base_url: &str, path: &str, default_path: &str, query: String) -> String {
        let mut url_path = path;
        if !default_path.is_empty() {
            url_path = default_path;
        }
        let query = if query.len() == 0 {
            "".to_string()
        } else {
            format!("?{}", query)
        };
        format!("{}{}{}", base_url, url_path, query)
    }
}
