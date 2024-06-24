// Copyright (c) 2023 LogCraft, SAS.
// SPDX-License-Identifier: MPL-2.0

mod bindings;
use crate::bindings::exports::logcraft::host::plugin::{Guest, Metadata};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, time::Duration};
use url::Url;
use waki::{Client, Method, RequestBuilder};

const SETTINGS: &str = include_str!("../package/settings.k");
const DETECTION_SCHEMA: &str = include_str!("../package/rule.k");

#[derive(Serialize, Deserialize)]
struct Splunk {
    pub endpoint: String,
    pub authorization: String,
    pub authorization_scheme: String,
    pub timeout: Option<u64>,
}

impl Splunk {
    fn client(
        &self,
        method: Method,
        path: &str,
        app: Option<&str>,
        user: Option<String>,
    ) -> Result<RequestBuilder, String> {
        let url = Url::parse(&format!(
            "{}/servicesNS/{}/{}/saved/searches/",
            &self.endpoint,
            user.unwrap_or("nobody".to_string()),
            app.unwrap_or("-")
        ))
        .map_err(|e| e.to_string())?
        .join(path)
        .map_err(|e| e.to_string())?;

        let client = Client::new()
            .request(method, url.as_str())
            .connect_timeout(Duration::from_secs(self.timeout.unwrap_or(60)))
            .header(
                "Authorization",
                format!(
                    "{} {}",
                    self.authorization_scheme.as_str(),
                    self.authorization.as_str()
                ),
            );

        Ok(client)
    }

    fn check_target_app(&self, app: &str) -> Result<(), String> {
        let url = Url::parse(&format!("{}/services/apps/local/{}", &self.endpoint, app))
            .map_err(|e| e.to_string())?;

        // return Err(url.to_string());
        let client = Client::new()
            .request(Method::Get, url.as_str())
            .connect_timeout(Duration::from_secs(self.timeout.unwrap_or(60)))
            .header(
                "Authorization",
                format!(
                    "{} {}",
                    self.authorization_scheme.as_str(),
                    self.authorization.as_str()
                ),
            );

        match client.send() {
            Ok(response) => match response.status_code() {
                200 => Ok(()),
                404 => Err(format!("target app '{}' not found", app)),
                status => Err(format!("unable to check target app: HTTP/{}", status)),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    fn parse_configuration(config: &str) -> Result<Self, String> {
        serde_json::from_str::<Splunk>(config)
            .map_err(|err| format!("unable to parse configuration: {}", err))
    }
}

#[derive(Serialize, Deserialize)]
struct SavedSearch {
    pub app: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
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
        let mut search_params: SavedSearch = serde_json::from_str(&params)
            .map_err(|err| format!("unable to parse detection url: {}", err))?;

        // Prepare query
        search_params
            .savedsearch
            .insert("name".to_string(), Value::String(name));

        // Prepare request
        let client = Splunk::parse_configuration(&config)?
            .client(
                Method::Post,
                "",
                Some(&search_params.app),
                search_params.user,
            )?
            .query(&[("output_mode", "json")])
            .form(&search_params.savedsearch);

        match client.send() {
            Ok(resp) => match resp.status_code() {
                201 => match String::from_utf8(resp.body().unwrap()) {
                    Ok(_) => Ok(Some(String::from("OK"))),
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
            Err(e) => Err(e.to_string()),
        }
    }

    /// Get SavedSearch
    fn read(config: String, name: String, params: String) -> Result<Option<String>, String> {
        let search_params = serde_json::from_str::<SavedSearch>(&params)
            .map_err(|err| format!("unable to parse response: {}", err))?;

        let mut filter = search_params
            .savedsearch
            .keys()
            .map(|s| ("f", s.as_str()))
            .collect::<Vec<_>>();
        filter.push(("output_mode", "json"));

        // Prepare request
        let config = Splunk::parse_configuration(&config)?;
        config.check_target_app(&search_params.app)?;
        let client = config
            .client(
                Method::Get,
                &name,
                Some(&search_params.app),
                search_params.user.clone(),
            )?
            .query(&filter);

        match client.send() {
            Ok(resp) => match resp.status_code() {
                200 => match String::from_utf8(resp.body().unwrap()) {
                    Ok(resp) => {
                        let resp = &serde_json::from_str::<SearchResponse>(&resp.to_string())
                            .map_err(|err| format!("unable to parse response: {}", err))?
                            .entry[0]
                            .content;

                        let filtered: HashMap<String, Value> = search_params
                            .savedsearch
                            .iter()
                            .filter_map(|(k, _)| {
                                resp.get_key_value(k).map(|(k, v)| (k.clone(), v.clone()))
                            })
                            .collect();

                        let resp = serde_json::to_string(&SavedSearch {
                            app: search_params.app.clone(),
                            user: search_params.user.clone(),
                            savedsearch: filtered,
                        })
                        .map_err(|err| format!("unable to parse response: {}", err))?;

                        Ok(Some(resp))
                    }
                    Err(e) => Err(e.to_string()),
                },
                404 => Ok(None),
                code => Err(format!("HTTP/{}", code)),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    /// Update SavedSearch
    fn update(config: String, name: String, params: String) -> Result<Option<String>, String> {
        // Parse saved search
        let search_params: SavedSearch = serde_json::from_str(&params)
            .map_err(|err| format!("unable to parse detection url: {}", err))?;

        // Prepare request
        let client = Splunk::parse_configuration(&config)?
            .client(
                Method::Post,
                &name,
                Some(&search_params.app),
                search_params.user,
            )?
            .query(&[("output_mode", "json")])
            .form(&search_params.savedsearch);

        match client.send() {
            Ok(resp) => match resp.status_code() {
                200 => match String::from_utf8(resp.body().unwrap()) {
                    Ok(_) => Ok(Some(String::from("OK"))),
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
            Err(e) => Err(e.to_string()),
        }
    }

    /// Delete SavedSearch
    fn delete(config: String, name: String, params: String) -> Result<Option<String>, String> {
        // Parse saved search
        let search_params: SavedSearch = serde_json::from_str(&params)
            .map_err(|err| format!("unable to parse detection url: {}", err))?;

        // Prepare request
        let config = Splunk::parse_configuration(&config)?;
        config.check_target_app(&search_params.app)?;
        let client = config
            .client(
                Method::Delete,
                &name,
                Some(&search_params.app),
                search_params.user,
            )?
            .query(&[("output_mode", "json")]);

        match client.send() {
            Ok(resp) => match resp.status_code() {
                200 => match String::from_utf8(resp.body().unwrap()) {
                    Ok(_) => Ok(Some(String::from("OK"))),
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
            Err(e) => Err(e.to_string()),
        }
    }

    /// Ping service
    fn ping(config: String) -> Result<bool, String> {
        // Prepare request
        let client =
            Splunk::parse_configuration(&config)?.client(Method::Get, "?count=1", None, None)?;

        match client.send() {
            Ok(resp) => match resp.status_code() {
                200 => Ok(true),
                401 => Err("Unauthorized".to_string()),
                code => Err(format!("HTTP/{}", code)),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}

bindings::export!(Splunk with_types_in bindings);
