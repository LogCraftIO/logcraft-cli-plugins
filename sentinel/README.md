# Microsoft Azure Sentinel plugin

LogCraft CLI plugin for Microsoft Azure Sentinel.

## Installation

For installation instructions, please refer to the [main README](../README.md) (i.e. download the plugin from the [releases](https://github.com/LogCraftIO/logcraft-cli-plugins/releases) page).

## Configuration

The plugin has the following parameters:

- `client_id`: Azure Client ID
- `client_secret`: Azure Client Secret
- `tenant_id`: Azure Tenant ID
- `api_version`: optional Sentinel API version, defaults to `2023-11-01` (latest)
- `resource_group_name`: The name of the Azure resource group (case insensitive)
- `workspace_name`: The name of the Azure workspace
- `subscription_id`: The target Azure Subscription ID
- `timeout`: optional plugin timeout (default: 60 seconds).

Example

```
....
services:
- id: sentinel-prod
  plugin: sentinel
  settings:
    client_id: 10214573-e66f-4efc-9c58-b61a179b89d2
    client_secret: some-secure-secret
    tenant_id: 33be0e7b-421e-4095-a44b-a5187a8470a3
    resource_group_name: myResourceGroup
    workspace_name: myWorkspace
    subscription_id: 74202d25-6ae4-49e8-aca6-5ac74cd1a4ab
```

## Rule

Here is an example of a detection rule

```
....

rules:
    ...
    sentinel:
        kind: Scheduled
        # ruleId: 04df2776-e230-4df0-9624-56364de3f902
        properties:
            enabled: true
            severity: Medium
            query: |-
                AzureDiagnostics
                | where Category == 'JobLogs'
                | extend RunbookName = RunbookName_s
                | project TimeGenerated,RunbookName,ResultType,CorrelationId,JobId_g
                | summarize StartTime = minif(TimeGenerated,ResultType == 'Started'),EndTime = minif(TimeGenerated,ResultType in ('Completed','Failed','Failed')),
                Status = tostring(parse_json(make_list_if(ResultType,ResultType in ('Completed','Failed','Stopped')))[0]) by JobId_g,RunbookName
                | extend DurationSec = datetime_diff('second', EndTime,StartTime)
                | join kind=leftouter (AzureDiagnostics
                | where Category == "JobStreams"
                | where StreamType_s == "Error"
                | summarize TotalErrors = dcount(StreamType_s) by JobId_g, StreamType_s)
                on $left. JobId_g == $right. JobId_g
                | extend HasErrors = iff(StreamType_s == 'Error',true,false)
                | project StartTime, EndTime, DurationSec,RunbookName,Status,HasErrors,TotalErrors,JobId_g

```

**NOTE** 
- The `ruleId` field is optional per Microsoft Azure Sentinel specifications. If this field is absent, then the rule ID will be set to the detection display name. 
- Refer to the [rule schema](./package/rule.k) for a complete list of fields
- This Azure query was borrowed from the "Query of the month" from [Kusto Insights](https://kustoinsights.substack.com/p/kusto-insights-june-update)

## Feedback!

This plugin needs some more attention, we are looking for volunteers to test it and give us some feedback 🙏
