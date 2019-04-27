Exports assets with time and resource types to a given Cloud Storage
location. The output format is newline-delimited JSON.
This API implements the google.longrunning.Operation API allowing you
to keep track of the export.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudasset1-beta1 --scope <scope> projects export-assets ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The relative name of the root asset. This can only be an organization
        number (such as &#34;organizations/123&#34;), a project ID (such as
        &#34;projects/my-project-id&#34;), or a project number (such as &#34;projects/12345&#34;).
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ExportAssetsRequest:
  asset-types: [string]
  content-type: string
  output-config:
    gcs-destination:
      uri: string
  read-time: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    asset-types=et`
    - A list of asset types of which to take a snapshot for. For example:
        &#34;google.compute.disk&#34;. If specified, only matching assets will be returned.
    - Each invocation of this argument appends the given value to the array.
* `content-type=dolores`
    - Asset content type. If not specified, no content but the asset name will be
        returned.
* `output-config.gcs-destination    uri=kasd`
    - The path of the Cloud Storage objects. It&#39;s the same path that is used by
         gsutil. For example: &#34;gs://bucket_name/object_path&#34;. See [Viewing and Editing Object Metadata](https://cloud.google.com/storage/docs/viewing-editing-metadata)
        for more information.


* `...    read-time=accusam`
    - Timestamp to take an asset snapshot. This can only be set to a timestamp
        between 2018-10-02 UTC (inclusive) and the current time. If not specified,
        the current time will be used. Due to delays in resource data collection
        and indexing, there is a volatile window during which running the same
        query may get different results.


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

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).
