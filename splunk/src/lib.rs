// Copyright (c) 2023 LogCraft, SAS.
// SPDX-License-Identifier: MPL-2.0

mod bindings;
use crate::bindings::exports::logcraft::host::plugin::{Guest, Metadata};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, time::Duration};
use url::Url;
use wasi_http_client::{Client, Method, RequestBuilder};

const SETTINGS: &str = include_str!("../package/settings.k");
const DETECTION_SCHEMA: &str = include_str!("../package/rule.k");

#[derive(Serialize, Deserialize)]
struct Splunk {
    pub endpoint: String,
    pub authorization: String,
    pub timeout: Option<u64>,
}

impl Splunk {
    fn client(&self, method: Method, path: &str) -> Result<RequestBuilder, String> {
        let url = Url::parse(&format!("{}/services/saved/searches/", &self.endpoint))
            .map_err(|e| e.to_string())?
            .join(path)
            .map_err(|e| e.to_string())?;

        let client = Client::new()
            .request(method, url.as_str())
            .connect_timeout(Duration::from_secs(self.timeout.unwrap_or(60)))
            .header("Authorization", self.authorization.as_str());

        Ok(client)
    }

    fn parse_configuration(config: &str) -> Result<Self, String> {
        serde_json::from_str::<Splunk>(config)
            .map_err(|err| format!("unable to parse configuration: {}", err))
    }
}

#[derive(Deserialize)]
struct SavedSearch {
    // pub app: String,
    pub savedsearch: HashMap<String, Value>,
}

#[derive(Deserialize)]
struct SplunkReponse {
    pub messages: Vec<Message>,
}

#[derive(Deserialize)]
struct Message {
    pub r#type: String,
    pub text: String,
}

#[derive(Deserialize)]
struct SearchResponse {
    pub entry: Vec<Entry>,
}

#[derive(Deserialize)]
struct Entry {
    pub content: HashMap<String, Value>,
}

impl Guest for Splunk {
    /// Retrieve plugin metadata
    fn load() -> Metadata {
        Metadata {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            author: env!("CARGO_PKG_AUTHORS").to_string(),
            description: env!("CARGO_PKG_DESCRIPTION").to_string(),
        }
    }

    /// Retrieve plugin settings
    fn settings() -> String {
        SETTINGS.to_string()
    }

    /// Retrieve plugin detection schema
    fn schema() -> String {
        DETECTION_SCHEMA.to_string()
    }

    /// Create SavedSearch
    fn create(config: String, name: String, params: String) -> Result<Option<String>, String> {
        // Parse saved search
        let mut search: SavedSearch = serde_json::from_str(&params)
            .map_err(|err| format!("unable to parse detection url: {}", err))?;

        // Prepare query
        search
            .savedsearch
            .insert("name".to_string(), Value::String(name));

        // Prepare request
        let client = Splunk::parse_configuration(&config)?
            .client(Method::Post, "")?
            .form(&search.savedsearch)
            .query(&[("output_mode", "json")]);

        match client.send() {
            Ok(resp) => match resp.status() {
                201 => match String::from_utf8(resp.body().unwrap()) {
                    Ok(_) => Ok(Some(String::from("prespent"))),
                    Err(e) => Err(e.to_string()),
                },
                400 => {
                    let resp = resp.body().map_err(|e| e.to_string())?;

                    match serde_json::from_slice::<SplunkReponse>(&resp) {
                        Ok(resp) => Err(format!(
                            "{}: {}",
                            resp.messages[0].r#type, resp.messages[0].text
                        )),
                        Err(_) => Err(format!("RAW_ERROR: {}", String::from_utf8(resp).unwrap())),
                    }
                }
                code => Err(format!("HTTP/{}", code)),
            },
            Err(e) => Err(format!("unable to send request: {}", e)),
        }
    }

    /// Get SavedSearch
    fn read(config: String, name: String, params: String) -> Result<Option<String>, String> {
        // Prepare request
        let client = Splunk::parse_configuration(&config)?
            .client(Method::Get, &name)?
            .query(&[("output_mode", "json")]);

        match client.send() {
            Ok(resp) => match resp.status() {
                200 => match String::from_utf8(resp.body().unwrap()) {
                    Ok(resp) => {
                        let search_params = &serde_json::from_str::<SavedSearch>(&params)
                            .map_err(|err| format!("unable to parse response: {}", err))?;

                        let resp = &serde_json::from_str::<SearchResponse>(&resp.to_string())
                            .map_err(|err| format!("unable to parse response: {}", err))?
                            .entry[0]
                            .content;

                        // Check for changes
                        for (key, value) in search_params.savedsearch.iter() {
                            if let Some(resp_value) = resp.get(key) {
                                if resp_value != value {
                                    return Ok(Some(String::from("1")));
                                }
                            } else {
                                return Ok(Some(String::from("1")));
                            }
                        }

                        Ok(Some(String::default()))
                    }
                    Err(e) => Err(e.to_string()),
                },
                404 => Ok(None),
                code => Err(format!("HTTP/{}", code)),
            },
            Err(e) => Err(format!("unable to send request: {}", e)),
        }
    }

    /// Update SavedSearch
    fn update(config: String, name: String, params: String) -> Result<Option<String>, String> {
        // Parse saved search
        let search: SavedSearch = serde_json::from_str(&params)
            .map_err(|err| format!("unable to parse detection url: {}", err))?;

        // Prepare request
        let client = Splunk::parse_configuration(&config)?
            .client(Method::Post, &name)?
            .header("Content-Type", "application/json")
            .query(&search.savedsearch)
            .query(&[("output_mode", "json")]);

        match client.send() {
            Ok(resp) => match resp.status() {
                200 => match String::from_utf8(resp.body().unwrap()) {
                    Ok(_) => Ok(Some(String::from("prespent"))),
                    Err(e) => Err(e.to_string()),
                },
                400 => {
                    let resp = resp.body().map_err(|e| e.to_string())?;

                    match serde_json::from_slice::<SplunkReponse>(&resp) {
                        Ok(resp) => Err(format!(
                            "{}: {}",
                            resp.messages[0].r#type, resp.messages[0].text
                        )),
                        Err(_) => Err(format!("RAW_ERROR: {}", String::from_utf8(resp).unwrap())),
                    }
                }
                code => Err(format!("HTTP/{}", code)),
            },
            Err(e) => Err(format!("unable to send request: {}", e)),
        }
    }

    /// Delete SavedSearch
    fn delete(config: String, name: String, _params: String) -> Result<Option<String>, String> {
        // Prepare request
        let client = Splunk::parse_configuration(&config)?
            .client(Method::Delete, &name)?
            .query(&[("output_mode", "json")]);

        match client.send() {
            Ok(resp) => match resp.status() {
                200 => match String::from_utf8(resp.body().unwrap()) {
                    Ok(_) => Ok(Some(String::from("prespent"))),
                    Err(e) => Err(e.to_string()),
                },
                404 => Ok(None),
                400 => {
                    let resp = resp.body().map_err(|e| e.to_string())?;

                    match serde_json::from_slice::<SplunkReponse>(&resp) {
                        Ok(resp) => Err(format!(
                            "{}: {}",
                            resp.messages[0].r#type, resp.messages[0].text
                        )),
                        Err(_) => Err(format!("RAW_ERROR: {}", String::from_utf8(resp).unwrap())),
                    }
                }
                code => Err(format!("HTTP/{}", code)),
            },
            Err(e) => Err(format!("unable to send request: {}", e)),
        }
    }

    /// Ping service
    fn ping(config: String) -> Result<bool, String> {
        // Prepare request
        let client = Splunk::parse_configuration(&config)?.client(Method::Get, "?count=1")?;

        match client.send() {
            Ok(resp) => match resp.status() {
                200 => Ok(true),
                401 => Err("Unauthorized".to_string()),
                code => Err(format!("HTTP/{}", code)),
            },
            Err(e) => Err(format!("unable to send request: {}", e)),
        }
    }
}

bindings::export!(Splunk with_types_in bindings);
