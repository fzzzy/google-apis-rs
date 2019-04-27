Generates and returns a report immediately.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/doubleclicksearch* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/doubleclicksearch*.
You can set the scope for this method like this: `doubleclicksearch2 --scope <scope> reports generate ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ReportRequest:
  download-format: string
  include-deleted-entities: boolean
  include-removed-entities: boolean
  max-rows-per-file: integer
  report-scope:
    ad-group-id: string
    ad-id: string
    advertiser-id: string
    agency-id: string
    campaign-id: string
    engine-account-id: string
    keyword-id: string
  report-type: string
  row-count: integer
  start-row: integer
  statistics-currency: string
  time-range:
    changed-attributes-since-timestamp: string
    changed-metrics-since-timestamp: string
    end-date: string
    start-date: string
  verify-single-time-zone: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    download-format=sed`
    - Format that the report should be returned in. Currently csv or tsv is supported.
* `include-deleted-entities=false`
    - Determines if removed entities should be included in the report. Defaults to false. Deprecated, please use includeRemovedEntities instead.
* `include-removed-entities=true`
    - Determines if removed entities should be included in the report. Defaults to false.
* `max-rows-per-file=38`
    - Asynchronous report only. The maximum number of rows per report file. A large report is split into many files based on this field. Acceptable values are 1000000 to 100000000, inclusive.
* `report-scope    ad-group-id=accusam`
    - DS ad group ID.
* `ad-id=takimata`
    - DS ad ID.
* `advertiser-id=justo`
    - DS advertiser ID.
* `agency-id=amet.`
    - DS agency ID.
* `campaign-id=erat`
    - DS campaign ID.
* `engine-account-id=labore`
    - DS engine account ID.
* `keyword-id=sea`
    - DS keyword ID.

* `..    report-type=nonumy`
    - Determines the type of rows that are returned in the report. For example, if you specify reportType: keyword, each row in the report will contain data about a keyword. See the Types of Reports reference for the columns that are available for each type.
* `row-count=82`
    - Synchronous report only. The maxinum number of rows to return; additional rows are dropped. Acceptable values are 0 to 10000, inclusive. Defaults to 10000.
* `start-row=40`
    - Synchronous report only. Zero-based index of the first row to return. Acceptable values are 0 to 50000, inclusive. Defaults to 0.
* `statistics-currency=sadipscing`
    - Specifies the currency in which monetary will be returned. Possible values are: usd, agency (valid if the report is scoped to agency or lower), advertiser (valid if the report is scoped to * advertiser or lower), or account (valid if the report is scoped to engine account or lower).
* `time-range    changed-attributes-since-timestamp=aliquyam`
    - Inclusive UTC timestamp in RFC format, e.g., 2013-07-16T10:16:23.555Z. See additional references on how changed attribute reports work.
* `changed-metrics-since-timestamp=ea`
    - Inclusive UTC timestamp in RFC format, e.g., 2013-07-16T10:16:23.555Z. See additional references on how changed metrics reports work.
* `end-date=no`
    - Inclusive date in YYYY-MM-DD format.
* `start-date=justo`
    - Inclusive date in YYYY-MM-DD format.

* `..    verify-single-time-zone=true`
    - If true, the report would only be created if all the requested stat data are sourced from a single timezone. Defaults to false.


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
