Returns the fields that are compatible to be selected in the respective sections of a report criteria, given the fields already selected in the input report and user permissions.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfareporting* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfareporting*.
You can set the scope for this method like this: `dfareporting2d8 --scope <scope> reports compatible-fields-query ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - The DFA user profile ID.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Report:
  account-id: string
  criteria:
    activities:
      kind: string
      metric-names: [string]
    custom-rich-media-events:
      kind: string
    date-range:
      end-date: string
      kind: string
      relative-date-range: string
      start-date: string
    metric-names: [string]
  cross-dimension-reach-criteria:
    date-range:
      end-date: string
      kind: string
      relative-date-range: string
      start-date: string
    dimension: string
    metric-names: [string]
    overlap-metric-names: [string]
    pivoted: boolean
  delivery:
    email-owner: boolean
    email-owner-delivery-type: string
    message: string
  etag: string
  file-name: string
  floodlight-criteria:
    date-range:
      end-date: string
      kind: string
      relative-date-range: string
      start-date: string
    floodlight-config-id:
      dimension-name: string
      etag: string
      id: string
      kind: string
      match-type: string
      value: string
    metric-names: [string]
    report-properties:
      include-attributed-ip-conversions: boolean
      include-unattributed-cookie-conversions: boolean
      include-unattributed-ip-conversions: boolean
  format: string
  id: string
  kind: string
  last-modified-time: string
  name: string
  owner-profile-id: string
  path-to-conversion-criteria:
    date-range:
      end-date: string
      kind: string
      relative-date-range: string
      start-date: string
    floodlight-config-id:
      dimension-name: string
      etag: string
      id: string
      kind: string
      match-type: string
      value: string
    metric-names: [string]
    report-properties:
      clicks-lookback-window: integer
      impressions-lookback-window: integer
      include-attributed-ip-conversions: boolean
      include-unattributed-cookie-conversions: boolean
      include-unattributed-ip-conversions: boolean
      maximum-click-interactions: integer
      maximum-impression-interactions: integer
      maximum-interaction-gap: integer
      pivot-on-interaction-path: boolean
  reach-criteria:
    activities:
      kind: string
      metric-names: [string]
    custom-rich-media-events:
      kind: string
    date-range:
      end-date: string
      kind: string
      relative-date-range: string
      start-date: string
    enable-all-dimension-combinations: boolean
    metric-names: [string]
    reach-by-frequency-metric-names: [string]
  schedule:
    active: boolean
    every: integer
    expiration-date: string
    repeats: string
    repeats-on-week-days: [string]
    runs-on-day-of-month: string
    start-date: string
  sub-account-id: string
  type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=lorem`
    - The account ID to which this report belongs.
* `criteria.activities    kind=clita`
    - The kind of resource this is, in this case dfareporting#activities.
* `metric-names=et`
    - List of names of floodlight activity metrics.
    - Each invocation of this argument appends the given value to the array.

* `..custom-rich-media-events    kind=tempor`
    - The kind of resource this is, in this case dfareporting#customRichMediaEvents.

* `..date-range    end-date=ea`
    - The end date of the date range, inclusive. A string of the format: &#34;yyyy-MM-dd&#34;.
* `kind=aliquyam`
    - The kind of resource this is, in this case dfareporting#dateRange.
* `relative-date-range=gubergren`
    - The date range relative to the date of when the report is run.
* `start-date=amet`
    - The start date of the date range, inclusive. A string of the format: &#34;yyyy-MM-dd&#34;.

* `..    metric-names=voluptua.`
    - The list of names of metrics the report should include.
    - Each invocation of this argument appends the given value to the array.

* `..cross-dimension-reach-criteria.date-range    end-date=consetetur`
    - The end date of the date range, inclusive. A string of the format: &#34;yyyy-MM-dd&#34;.
* `kind=sanctus`
    - The kind of resource this is, in this case dfareporting#dateRange.
* `relative-date-range=at`
    - The date range relative to the date of when the report is run.
* `start-date=labore`
    - The start date of the date range, inclusive. A string of the format: &#34;yyyy-MM-dd&#34;.

* `..    dimension=et`
    - The dimension option.
* `metric-names=rebum.`
    - The list of names of metrics the report should include.
    - Each invocation of this argument appends the given value to the array.
* `overlap-metric-names=amet`
    - The list of names of overlap metrics the report should include.
    - Each invocation of this argument appends the given value to the array.
* `pivoted=true`
    - Whether the report is pivoted or not. Defaults to true.

* `..delivery    email-owner=false`
    - Whether the report should be emailed to the report owner.
* `email-owner-delivery-type=takimata`
    - The type of delivery for the owner to receive, if enabled.
* `message=labore`
    - The message to be sent with each email.

* `..    etag=dolore`
    - The eTag of this response for caching purposes.
* `file-name=accusam`
    - The filename used when generating report files for this report.
* `floodlight-criteria.date-range    end-date=voluptua.`
    - The end date of the date range, inclusive. A string of the format: &#34;yyyy-MM-dd&#34;.
* `kind=sadipscing`
    - The kind of resource this is, in this case dfareporting#dateRange.
* `relative-date-range=vero`
    - The date range relative to the date of when the report is run.
* `start-date=ut`
    - The start date of the date range, inclusive. A string of the format: &#34;yyyy-MM-dd&#34;.

* `..floodlight-config-id    dimension-name=sadipscing`
    - The name of the dimension.
* `etag=ipsum`
    - The eTag of this response for caching purposes.
* `id=ea`
    - The ID associated with the value if available.
* `kind=rebum.`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=takimata`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=et`
    - The value of the dimension.

* `..    metric-names=et`
    - The list of names of metrics the report should include.
    - Each invocation of this argument appends the given value to the array.
* `report-properties    include-attributed-ip-conversions=false`
    - Include conversions that have no cookie, but do have an exposure path.
* `include-unattributed-cookie-conversions=true`
    - Include conversions of users with a DoubleClick cookie but without an exposure. That means the user did not click or see an ad from the advertiser within the Floodlight group, or that the interaction happened outside the lookback window.
* `include-unattributed-ip-conversions=false`
    - Include conversions that have no associated cookies and no exposures. It’s therefore impossible to know how the user was exposed to your ads during the lookback window prior to a conversion.


* `...    format=dolor`
    - The output format of the report. If not specified, default format is &#34;CSV&#34;. Note that the actual format in the completed report file might differ if for instance the report&#39;s size exceeds the format&#39;s capabilities. &#34;CSV&#34; will then be the fallback format.
* `id=invidunt`
    - The unique ID identifying this report resource.
* `kind=erat`
    - The kind of resource this is, in this case dfareporting#report.
* `last-modified-time=dolor`
    - The timestamp (in milliseconds since epoch) of when this report was last modified.
* `name=ut`
    - The name of the report.
* `owner-profile-id=vero`
    - The user profile id of the owner of this report.
* `path-to-conversion-criteria.date-range    end-date=sit`
    - The end date of the date range, inclusive. A string of the format: &#34;yyyy-MM-dd&#34;.
* `kind=elitr`
    - The kind of resource this is, in this case dfareporting#dateRange.
* `relative-date-range=vero`
    - The date range relative to the date of when the report is run.
* `start-date=dolores`
    - The start date of the date range, inclusive. A string of the format: &#34;yyyy-MM-dd&#34;.

* `..floodlight-config-id    dimension-name=diam`
    - The name of the dimension.
* `etag=sed`
    - The eTag of this response for caching purposes.
* `id=eirmod`
    - The ID associated with the value if available.
* `kind=dolor`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=sed`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=justo`
    - The value of the dimension.

* `..    metric-names=amet`
    - The list of names of metrics the report should include.
    - Each invocation of this argument appends the given value to the array.
* `report-properties    clicks-lookback-window=33`
    - DFA checks to see if a click interaction occurred within the specified period of time before a conversion. By default the value is pulled from Floodlight or you can manually enter a custom value. Valid values: 1-90.
* `impressions-lookback-window=6`
    - DFA checks to see if an impression interaction occurred within the specified period of time before a conversion. By default the value is pulled from Floodlight or you can manually enter a custom value. Valid values: 1-90.
* `include-attributed-ip-conversions=false`
    - Deprecated: has no effect.
* `include-unattributed-cookie-conversions=false`
    - Include conversions of users with a DoubleClick cookie but without an exposure. That means the user did not click or see an ad from the advertiser within the Floodlight group, or that the interaction happened outside the lookback window.
* `include-unattributed-ip-conversions=true`
    - Include conversions that have no associated cookies and no exposures. It’s therefore impossible to know how the user was exposed to your ads during the lookback window prior to a conversion.
* `maximum-click-interactions=60`
    - The maximum number of click interactions to include in the report. Advertisers currently paying for E2C reports get up to 200 (100 clicks, 100 impressions). If another advertiser in your network is paying for E2C, you can have up to 5 total exposures per report.
* `maximum-impression-interactions=9`
    - The maximum number of click interactions to include in the report. Advertisers currently paying for E2C reports get up to 200 (100 clicks, 100 impressions). If another advertiser in your network is paying for E2C, you can have up to 5 total exposures per report.
* `maximum-interaction-gap=87`
    - The maximum amount of time that can take place between interactions (clicks or impressions) by the same user. Valid values: 1-90.
* `pivot-on-interaction-path=false`
    - Enable pivoting on interaction path.


* `...reach-criteria.activities    kind=invidunt`
    - The kind of resource this is, in this case dfareporting#activities.
* `metric-names=lorem`
    - List of names of floodlight activity metrics.
    - Each invocation of this argument appends the given value to the array.

* `..custom-rich-media-events    kind=vero`
    - The kind of resource this is, in this case dfareporting#customRichMediaEvents.

* `..date-range    end-date=invidunt`
    - The end date of the date range, inclusive. A string of the format: &#34;yyyy-MM-dd&#34;.
* `kind=sea`
    - The kind of resource this is, in this case dfareporting#dateRange.
* `relative-date-range=nonumy`
    - The date range relative to the date of when the report is run.
* `start-date=sea`
    - The start date of the date range, inclusive. A string of the format: &#34;yyyy-MM-dd&#34;.

* `..    enable-all-dimension-combinations=true`
    - Whether to enable all reach dimension combinations in the report. Defaults to false. If enabled, the date range of the report should be within the last 42 days.
* `metric-names=justo`
    - The list of names of metrics the report should include.
    - Each invocation of this argument appends the given value to the array.
* `reach-by-frequency-metric-names=lorem`
    - The list of names of  Reach By Frequency metrics the report should include.
    - Each invocation of this argument appends the given value to the array.

* `..schedule    active=true`
    - Whether the schedule is active or not. Must be set to either true or false.
* `every=25`
    - Defines every how many days, weeks or months the report should be run. Needs to be set when &#34;repeats&#34; is either &#34;DAILY&#34;, &#34;WEEKLY&#34; or &#34;MONTHLY&#34;.
* `expiration-date=et`
    - The expiration date when the scheduled report stops running.
* `repeats=duo`
    - The interval for which the report is repeated. Note:  
        - &#34;DAILY&#34; also requires field &#34;every&#34; to be set. 
        - &#34;WEEKLY&#34; also requires fields &#34;every&#34; and &#34;repeatsOnWeekDays&#34; to be set. 
        - &#34;MONTHLY&#34; also requires fields &#34;every&#34; and &#34;runsOnDayOfMonth&#34; to be set.
* `repeats-on-week-days=tempor`
    - List of week days &#34;WEEKLY&#34; on which scheduled reports should run.
    - Each invocation of this argument appends the given value to the array.
* `runs-on-day-of-month=duo`
    - Enum to define for &#34;MONTHLY&#34; scheduled reports whether reports should be repeated on the same day of the month as &#34;startDate&#34; or the same day of the week of the month.
        Example: If &#39;startDate&#39; is Monday, April 2nd 2012 (2012-04-02), &#34;DAY_OF_MONTH&#34; would run subsequent reports on the 2nd of every Month, and &#34;WEEK_OF_MONTH&#34; would run subsequent reports on the first Monday of the month.
* `start-date=ipsum`
    - Start date of date range for which scheduled reports should be run.

* `..    sub-account-id=sadipscing`
    - The subaccount ID to which this report belongs if applicable.
* `type=justo`
    - The type of the report.


### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.


# Optional Output Flags

The method's return value a JSON encoded structure, which will be written to standard output by default.

* **-o out**
    - *out* specifies the *destination* to which to write the server's result to.
      It will be a JSON-encoded structure.
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p alt=string**
    - Data format for the response.

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.

* **-p user-ip=string**
    - Deprecated. Please use quotaUser instead.