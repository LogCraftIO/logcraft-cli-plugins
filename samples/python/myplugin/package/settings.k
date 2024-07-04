# Copyright (c) 2023 LogCraft, SAS.
# SPDX-License-Identifier: MPL-2.0

import regex

schema Configuration:
  """Splunk Configuration
  
  Attributes
  ----------
  endpoint: str, required
    Splunk URL
  authorization_scheme: str, required
    Authorization scheme
  authorization: str, required
    Authorization
  timeout: int, optional
    Timeout in seconds
  """
  # Mandatory Parameters

  # Splunk URL
  endpoint: str = 'https://splunk_endpoint:8089'
  # Authorization scheme
  authorization_scheme: AuthorizationScheme = "Bearer"
  # Authorization
  @info(sensitive="true")
  authorization: str = "base64_encoded_token"
  # Timeout
  timeout?: int = 60

  check:
    regex.match(endpoint, "^https?://"), "Incorrect endpoint, must start with http:// or https://."

type AuthorizationScheme = "Bearer" | "Basic"