Exports variant set data to an external destination.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/bigquery*
* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/genomics*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/bigquery*.
You can set the scope for this method like this: `genomics1 --scope <scope> variantsets export ...`
# Required Scalar Argument
* **&lt;variant-set-id&gt;** *(string)*
    - Required. The ID of the variant set that contains variant data which
        should be exported. The caller must have READ access to this variant set.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ExportVariantSetRequest:
  bigquery-dataset: string
  bigquery-table: string
  call-set-ids: [string]
  format: string
  project-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    bigquery-dataset=ut`
    - Required. The BigQuery dataset to export data to. This dataset must already
        exist. Note that this is distinct from the Genomics concept of &#34;dataset&#34;.
* `bigquery-table=amet.`
    - Required. The BigQuery table to export data to.
        If the table doesn&#39;t exist, it will be created. If it already exists, it
        will be overwritten.
* `call-set-ids=ipsum`
    - If provided, only variant call information from the specified call sets
        will be exported. By default all variant calls are exported.
    - Each invocation of this argument appends the given value to the array.
* `format=ut`
    - The format for the exported data.
* `project-id=dolor`
    - Required. The Google Cloud project ID that owns the destination
        BigQuery dataset. The caller must have WRITE access to this project.  This
        project will also own the resulting export job.


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
