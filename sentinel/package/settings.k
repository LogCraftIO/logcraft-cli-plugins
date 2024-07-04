# Copyright (c) 2023 LogCraft, SAS.
# SPDX-License-Identifier: MPL-2.0

import regex

schema Configuration:
  """Sentinel Configuration
  
  Attributes
  ----------
  client_id: str, required
    Azure client id
  client_secret: str, required
    Azure client secret
  tenant_id: str, required
    Azure tenant id
  api_version: str, optional
    Sentinel API version
  resource_group_name: str, required
    The name of the resource group. The name is case insensitive
  workspace_name: str, required
    The name of the workspace
  subscription_id: str, required
    The ID of the target subscription
  timeout: int, optional
    Timeout in seconds
  """
  # Mandatory Parameters
  # Client ID
  @info(sensitive="true")
  client_id: str = "MY_AZURE_CLIENT_ID"
  # Client Secret
  @info(sensitive="true")
  client_secret: str = "MY_AZURE_CLIENT_SECRET"
  # Tenant ID
  tenant_id: str = "my-tenant-id"
  # Sentinel API version
  api_version?: str = "2023-11-01"
  # ResourceGroup name 
  resource_group_name: str
  # Workspace name
  workspace_name: str
  # Target subscription ID
  subscription_id: str
  # Timeout
  timeout?: int = 60

  check:
    regex.match(client_id, "^[A-Za-z0-9][A-Za-z0-9-]+[A-Za-z0-9]"), "Incorrect client_id, must be kebab-case formatted"
    regex.match(tenant_id, "^[A-Za-z0-9][A-Za-z0-9-]+[A-Za-z0-9]"), "Incorrect tenant_id, must be kebab-case formatted"
    regex.match(subscription_id, "^[A-Za-z0-9][A-Za-z0-9-]+[A-Za-z0-9]"), "Incorrect subscription_id, must be kebab-case formatted"