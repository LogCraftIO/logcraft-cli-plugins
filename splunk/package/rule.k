# Copyright (c) 2023 LogCraft, SAS.
# SPDX-License-Identifier: MPL-2.0

schema Rule:
  """Splunk Detection Rule
  
  Attributes
  ----------
  app: str, required
    Application name
  """
  app: str
  savedsearch: SavedSearch

schema SavedSearch:
  """Splunk Saved Search
  cron_schedule: str, optional
    Valid values: cron string.
  disabled: str, optional
    Human-readable description of this saved search. Defaults to empty string.
  disabled: bool, optional
    Indicates if the saved search is enabled. Defaults to false.
  dispatch_buckets: int, optional
    The maximum number of timeline buckets. Defaults to 0.
  display_view: str, optional
    Defines the default UI view name (not label) in which to load the results.
  is_scheduled: bool, optional
    Whether this search is to be run on a schedule.
  is_visible: bool, optional
    Specifies whether this saved search should be listed in the visible saved search list. Defaults to true.
  max_concurrent: int, optional
    The maximum number of concurrent instances of this search the scheduler is allowed to run. Defaults to 1.
  realtime_schedule: bool, optional
    Controls the way the scheduler computes the next execution time of a scheduled search.
  request_ui_dispatch_app: str, optional
    Specifies a field used by Splunk Web to denote the app this search should be dispatched in.
  request_ui_dispatch_view: str, optional
    Specifies a field used by Splunk Web to denote the view this search should be displayed in.
  restart_on_searchpeer_add: bool, optional
    Specifies whether to restart a real-time search managed by the scheduler when a search peer becomes available for this saved search. Defaults to true.
  run_on_startup: bool, optional
    Specifies whether to restart a real-time search managed by the scheduler when a search peer becomes available for this saved search. Defaults to true.
  schedule_window: str, optional
    Time window (in minutes) during which the search has lower priority. Defaults to 0.
  schedule_priority: str, optional
    Raises the scheduling priority of the named search.
  search: str, optional
    Required when creating a new search.
  vsid: str, optional
    Defines the viewstate id associated with the UI view listed in 'displayview'.
  workload_pool: str, optional
    Specifies the new workload pool where the existing running search will be placed.
  """
  cron_schedule?: str
  description?: str
  disabled?: bool
  dispatch_buckets?: int
  display_view?: str
  is_scheduled?: bool
  is_visible?: bool
  max_concurrent?: int
  realtime_schedule?: bool
  request_ui_dispatch_app?: str
  request_ui_dispatch_view?: str
  restart_on_searchpeer_add?: bool
  run_on_startup?: bool
  schedule_window?: str
  schedule_priority?: str
  search?: str
  vsid?: str
  workload_pool?: str
  test?: bool | int | 't'
  actions: any
  acl: any
  dispatch: any