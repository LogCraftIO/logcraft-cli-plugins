# Copyright (c) 2023 LogCraft, SAS.
# SPDX-License-Identifier: MPL-2.0

schema Rule:
  """Splunk Detection Rule
  
  Attributes
  ----------
  app: str, required
    Application name
  """
  # Mandatory Parameters

  # Name of the Detection Rule
  app: str
  savedsearch: {str:any}