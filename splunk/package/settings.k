# Copyright (c) 2023 LogCraft, SAS.
# SPDX-License-Identifier: MPL-2.0

import regex

schema Configuration:
  """Splunk Configuration
  
  Attributes
  ----------
  endpoint: str, required
    Splunk endpoint
  authorization: str, required
    sensitive: Authorization header
  """
  # Mandatory Parameters

  # Splunk URL
  endpoint: str = 'https://splunk_endpoint:8089'
  # Authorization header
  authorization: str = 'Bearer c29tZQo=.dG9rZW4K'

  check:
    regex.match(authorization, "^(Bearer|Basic)\s+.+"), "Incorrect authorization, must be: 'Bearer|Basic  <authz_base64>'"
    regex.match(endpoint, "^https?://"), "Incorrect endpoint, must start with http:// or https://."