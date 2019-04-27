Creates a query.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/doubleclickbidmanager* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/doubleclickbidmanager*.
You can set the scope for this method like this: `doubleclickbidmanager1 --scope <scope> queries createquery ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Query:
  kind: string
  metadata:
    data-range: string
    format: string
    google-cloud-storage-path-for-latest-report: string
    google-drive-path-for-latest-report: string
    latest-report-run-time-ms: string
    locale: string
    report-count: integer
    running: boolean
    send-notification: boolean
    share-email-address: [string]
    title: string
  params:
    group-bys: [string]
    include-invite-data: boolean
    metrics: [string]
    type: string
  query-id: string
  report-data-end-time-ms: string
  report-data-start-time-ms: string
  schedule:
    end-time-ms: string
    frequency: string
    next-run-minute-of-day: integer
    next-run-timezone-code: string
  timezone-code: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    kind=accusam`
    - Identifies what kind of resource this is. Value: the fixed string &#34;doubleclickbidmanager#query&#34;.
* `metadata    data-range=takimata`
    - Range of report data.
* `format=justo`
    - Format of the generated report.
* `google-cloud-storage-path-for-latest-report=amet.`
    - The path to the location in Google Cloud Storage where the latest report is stored.
* `google-drive-path-for-latest-report=erat`
    - The path in Google Drive for the latest report.
* `latest-report-run-time-ms=labore`
    - The time when the latest report started to run.
* `locale=sea`
    - Locale of the generated reports. Valid values are cs CZECH de GERMAN en ENGLISH es SPANISH fr FRENCH it ITALIAN ja JAPANESE ko KOREAN pl POLISH pt-BR BRAZILIAN_PORTUGUESE ru RUSSIAN tr TURKISH uk UKRAINIAN zh-CN CHINA_CHINESE zh-TW TAIWAN_CHINESE
        
        An locale string not in the list above will generate reports in English.
* `report-count=11`
    - Number of reports that have been generated for the query.
* `running=true`
    - Whether the latest report is currently running.
* `send-notification=false`
    - Whether to send an email notification when a report is ready. Default to false.
* `share-email-address=sadipscing`
    - List of email addresses which are sent email notifications when the report is finished. Separate from sendNotification.
    - Each invocation of this argument appends the given value to the array.
* `title=aliquyam`
    - Query title. It is used to name the reports generated from this query.

* `..params    group-bys=ea`
    - Data is grouped by the filters listed in this field.
    - Each invocation of this argument appends the given value to the array.
* `include-invite-data=false`
    - Whether to include data from Invite Media.
* `metrics=justo`
    - Metrics to include as columns in your report.
    - Each invocation of this argument appends the given value to the array.
* `type=justo`
    - Report type.

* `..    query-id=et`
    - Query ID.
* `report-data-end-time-ms=et`
    - The ending time for the data that is shown in the report. Note, reportDataEndTimeMs is required if metadata.dataRange is CUSTOM_DATES and ignored otherwise.
* `report-data-start-time-ms=diam`
    - The starting time for the data that is shown in the report. Note, reportDataStartTimeMs is required if metadata.dataRange is CUSTOM_DATES and ignored otherwise.
* `schedule    end-time-ms=ipsum`
    - Datetime to periodically run the query until.
* `frequency=lorem`
    - How often the query is run.
* `next-run-minute-of-day=80`
    - Time of day at which a new report will be generated, represented as minutes past midnight. Range is 0 to 1439. Only applies to scheduled reports.
* `next-run-timezone-code=duo`
    - Canonical timezone code for report generation time. Defaults to America/New_York.

* `..    timezone-code=aliquyam`
    - Canonical timezone code for report data time. Defaults to America/New_York.


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
