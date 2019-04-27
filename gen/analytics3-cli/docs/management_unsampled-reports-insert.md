Create a new unsampled report.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/analytics*
* *https://www.googleapis.com/auth/analytics.edit*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/analytics*.
You can set the scope for this method like this: `analytics3 --scope <scope> management unsampled-reports-insert ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - Account ID to create the unsampled report for.
* **&lt;web-property-id&gt;** *(string)*
    - Web property ID to create the unsampled report for.
* **&lt;profile-id&gt;** *(string)*
    - View (Profile) ID to create the unsampled report for.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
UnsampledReport:
  account-id: string
  cloud-storage-download-details:
    bucket-id: string
    object-id: string
  created: string
  dimensions: string
  download-type: string
  drive-download-details:
    document-id: string
  end-date: string
  filters: string
  id: string
  kind: string
  metrics: string
  profile-id: string
  segment: string
  self-link: string
  start-date: string
  status: string
  title: string
  updated: string
  web-property-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=dolores`
    - Account ID to which this unsampled report belongs.
* `cloud-storage-download-details    bucket-id=dolores`
    - Id of the bucket the file object is stored in.
* `object-id=accusam`
    - Id of the file object containing the report data.

* `..    created=amet`
    - Time this unsampled report was created.
* `dimensions=duo`
    - The dimensions for the unsampled report.
* `download-type=diam`
    - The type of download you need to use for the report data file. Possible values include `GOOGLE_DRIVE` and `GOOGLE_CLOUD_STORAGE`. If the value is `GOOGLE_DRIVE`, see the `driveDownloadDetails` field. If the value is `GOOGLE_CLOUD_STORAGE`, see the `cloudStorageDownloadDetails` field.
* `drive-download-details    document-id=et`
    - Id of the document/file containing the report data.

* `..    end-date=sit`
    - The end date for the unsampled report.
* `filters=tempor`
    - The filters for the unsampled report.
* `id=rebum.`
    - Unsampled report ID.
* `kind=sed`
    - Resource type for an Analytics unsampled report.
* `metrics=et`
    - The metrics for the unsampled report.
* `profile-id=rebum.`
    - View (Profile) ID to which this unsampled report belongs.
* `segment=eos`
    - The segment for the unsampled report.
* `self-link=gubergren`
    - Link for this unsampled report.
* `start-date=dolores`
    - The start date for the unsampled report.
* `status=ut`
    - Status of this unsampled report. Possible values are PENDING, COMPLETED, or FAILED.
* `title=dolore`
    - Title of the unsampled report.
* `updated=eos`
    - Time this unsampled report was last modified.
* `web-property-id=voluptua.`
    - Web property ID to which this unsampled report belongs. The web property ID is of the form UA-XXXXX-YY.


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
