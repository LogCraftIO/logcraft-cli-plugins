// Copyright (c) 2023 LogCraft, SAS.
// SPDX-License-Identifier: MPL-2.0

mod bindings {
    wit_bindgen::generate!({
        path: "../wit",
        world: "logcraft:lgc/plugins"
    });
}
use bindings::{
    export,
    exports::logcraft::lgc::plugin::{Guest, Metadata},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, time::Duration};
use url::Url;
use waki::{Client, Method, RequestBuilder};

const SETTINGS: &str = include_str!("../package/settings.k");
const DETECTION_SCHEMA: &str = include_str!("../package/rule.k");

#[derive(Serialize, Deserialize)]
struct Sentinel {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    api_version: Option<String>,
    resource_group_name: String,
    subscription_id: String,
    workspace_name: String,
    timeout: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct SentinelRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
    kind: RuleType,
    properties: HashMap<String, Value>,
}

impl SentinelRule {
    fn default_properties(&mut self) -> &Self {
        // Default ISO 8601 duration format
        let default_duration = Value::String("PT5H".to_string());
        let defaults = [
            ("queryFrequency", default_duration.clone()),
            ("queryPeriod", default_duration.clone()),
            ("suppressionDuration", default_duration),
            ("suppressionEnabled", Value::Bool(false)),
            ("enabled", Value::Bool(true)),
            ("triggerOperator", Value::String("GreaterThan".to_string())),
            ("triggerThreshold", Value::Number(0.into())),
        ];

        for (fname, fvalue) in defaults {
            self.properties.entry(fname.to_string()).or_insert(fvalue);
        }

        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
enum RuleType {
    Scheduled,
}

#[derive(Deserialize)]
struct CloudError {
    error: ErrorBody,
}

#[derive(Deserialize)]
struct ErrorBody {
    code: String,
    message: String,
}

impl CloudError {
    fn from_slices(body: Vec<u8>) -> String {
        match serde_json::from_slice::<Self>(&body) {
            Ok(resp) => format!("{}: {}", resp.error.code, resp.error.message),
            Err(_) => String::from_utf8(body).unwrap(),
        }
    }
}

#[derive(Deserialize)]
struct AzureAuthz {
    access_token: String,
}

const AZURE_AUTH_DEFAULT_ENDPOINT: &str = "https://login.microsoftonline.com";
const AZURE_MGT_ENDPOINT: &str = "https://management.azure.com";

impl Sentinel {
    fn get_credentials(&self) -> Result<String, String> {
        let req = Client::new()
            .post(&format!(
                "{AZURE_AUTH_DEFAULT_ENDPOINT}/{}/oauth2/token",
                self.tenant_id
            ))
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("resource", AZURE_MGT_ENDPOINT),
            ]);

        match req.send() {
            Ok(resp) => match resp.status_code() {
                200 => {
                    let resp: AzureAuthz = serde_json::from_slice(
                        &resp.body().expect("azure authz invalid UTF-8 response"),
                    )
                    .map_err(|e| format!("unable to parse azure authz response: {e}"))?;

                    Ok(resp.access_token)
                }
                _ => Err(CloudError::from_slices(
                    resp.body()
                        .map_err(|e| format!("invalid UTF-8 response: {e}"))?,
                )),
            },
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn client(&self, method: Method, rule_id: &str) -> Result<RequestBuilder, String> {
        let url = Url::parse(&format!(
            "{AZURE_MGT_ENDPOINT}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/providers/Microsoft.SecurityInsights/alertRules/{}",
            &self.subscription_id,
            &self.resource_group_name,
            &self.workspace_name,
            rule_id
        ))
        .map_err(|e| e.to_string())?;

        let bearer_token = self.get_credentials()?;
        let client = Client::new()
            .request(method, url.as_str())
            .connect_timeout(Duration::from_secs(self.timeout.unwrap_or(60)))
            .header("Authorization", format!("Bearer {}", &bearer_token,))
            .query(&[(
                "api-version",
                self.api_version.clone().unwrap_or("2023-11-01".to_string()),
            )]);

        Ok(client)
    }

    fn parse_configuration(config: &str) -> Result<Self, String> {
        serde_json::from_str::<Sentinel>(config)
            .map_err(|err| format!("unable to parse configuration: {}", err))
    }
}

impl Guest for Sentinel {
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

    /// Create Sentinel detection rule
    fn create(config: String, name: String, params: String) -> Result<Option<String>, String> {
        let mut rule: SentinelRule =
            serde_json::from_str(&params).map_err(|e| format!("unable to parse rule: {e}"))?;

        rule.properties
            .insert("displayName".to_string(), Value::String(name.clone()));

        let rule_id = if let Some(rule_id) = &rule.rule_id {
            rule_id
        } else {
            &name
        };

        let config = Sentinel::parse_configuration(&config)?;
        let client = config
            .client(Method::Put, rule_id)?
            .header("Content-Type", "application/json")
            .json(rule.default_properties());

        match client.send() {
            Ok(resp) => match resp.status_code() {
                200 | 201 => Ok(Some(String::default())),
                _ => Err(CloudError::from_slices(
                    resp.body()
                        .map_err(|e| format!("invalid UTF-8 response: {e}"))?,
                )),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    /// Create Sentinel detection rule
    fn read(config: String, name: String, params: String) -> Result<Option<String>, String> {
        let mut rule: SentinelRule =
            serde_json::from_str(&params).map_err(|e| format!("unable to parse rule: {e}"))?;

        let rule_id = if let Some(rule_id) = &rule.rule_id {
            rule_id
        } else {
            &name
        };

        let config = Sentinel::parse_configuration(&config)?;
        let client = config.client(Method::Get, rule_id)?;

        match client.send() {
            Ok(resp) => match resp.status_code() {
                200 => match resp.body() {
                    Ok(body) => {
                        let resp: SentinelRule = serde_json::from_slice(&body)
                            .map_err(|e| format!("unable to parse response: {e}"))?;

                        let filtered: HashMap<String, Value> = rule
                            .properties
                            .iter()
                            .filter_map(|(k, _)| {
                                resp.properties
                                    .get_key_value(k)
                                    .map(|(k, v)| (k.clone(), v.clone()))
                            })
                            .collect();

                        rule.properties = filtered;
                        let json = serde_json::to_string(&rule)
                            .map_err(|e| format!("unable to serialize response: {e}"))?;
                        Ok(Some(json))
                    }
                    Err(e) => Err(format!("response: invalid UTF-8 sequence, {e}")),
                },
                404 => Ok(None),
                _ => Err(CloudError::from_slices(
                    resp.body()
                        .map_err(|e| format!("invalid UTF-8 response: {e}"))?,
                )),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    /// Create Sentinel detection rule
    fn update(config: String, name: String, params: String) -> Result<Option<String>, String> {
        // Sentinal uses the same method for create and udpdate
        Sentinel::create(config, name, params)
    }

    /// Delete Sentinel detection rule
    fn delete(config: String, name: String, params: String) -> Result<Option<String>, String> {
        let context: SentinelRule =
            serde_json::from_str(&params).map_err(|e| format!("unable to parse rule: {e}"))?;

        let rule_id = if let Some(rule_id) = context.rule_id {
            rule_id
        } else {
            name
        };

        let config = Sentinel::parse_configuration(&config)?;
        let client = config.client(Method::Delete, &rule_id)?;

        match client.send() {
            Ok(resp) => match resp.status_code() {
                200 => Ok(Some(String::default())),
                204 => Ok(None),
                _ => Err(CloudError::from_slices(
                    resp.body()
                        .map_err(|e| format!("invalid UTF-8 response: {e}"))?,
                )),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    /// Ping service
    fn ping(config: String) -> Result<bool, String> {
        let config = Sentinel::parse_configuration(&config)?;

        // Check access to workspace
        let workspace_endpoint = format!(
            "{AZURE_MGT_ENDPOINT}/subscriptions/{}/resourcegroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/providers/Microsoft.SecurityInsights/alertRules",
            config.subscription_id,
            config.resource_group_name,
            config.workspace_name,
        );

        match Client::new()
            .get(&workspace_endpoint)
            .header(
                "Authorization",
                format!("Bearer {}", config.get_credentials()?),
            )
            .query(&[(
                "api-version",
                config
                    .api_version
                    .clone()
                    .unwrap_or("2023-11-01".to_string()),
            )])
            .send()
        {
            Ok(resp) => match resp.status_code() {
                200 => Ok(true),
                _ => Err(CloudError::from_slices(
                    resp.body()
                        .map_err(|e| format!("invalid UTF-8 response: {e}"))?,
                )),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}

export!(Sentinel with_types_in bindings);
