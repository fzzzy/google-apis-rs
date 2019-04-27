Exports a copy of all or a subset of entities from Google Cloud Datastore
to another storage system, such as Google Cloud Storage. Recent updates to
entities may not be reflected in the export. The export occurs in the
background and its progress can be monitored and managed via the
Operation resource that is created. The output of an export may only be
used once the associated operation is done. If an export operation is
cancelled before completion it may leave partial data behind in Google
Cloud Storage.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/datastore*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `datastore1 --scope <scope> projects export ...`
# Required Scalar Argument
* **&lt;project-id&gt;** *(string)*
    - Project ID against which to make the request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GoogleDatastoreAdminV1ExportEntitiesRequest:
  entity-filter:
    kinds: [string]
    namespace-ids: [string]
  labels: { string: string }
  output-url-prefix: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .entity-filter    kinds=sed`
    - If empty, then this represents all kinds.
    - Each invocation of this argument appends the given value to the array.
* `namespace-ids=et`
    - An empty list represents all namespaces. This is the preferred
        usage for projects that don&#39;t use namespaces.
        
        An empty string element represents the default namespace. This should be
        used if the project has data in non-default namespaces, but doesn&#39;t want to
        include them.
        Each namespace in this list must be unique.
    - Each invocation of this argument appends the given value to the array.

* `..    labels=key=dolores`
    - Client-assigned labels.
    - the value will be associated with the given `key`
* `output-url-prefix=kasd`
    - Location for the export metadata and data files.
        
        The full resource URL of the external storage location. Currently, only
        Google Cloud Storage is supported. So output_url_prefix should be of the
        form: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the
        name of the Cloud Storage bucket and `NAMESPACE_PATH` is an optional Cloud
        Storage namespace path (this is not a Cloud Datastore namespace). For more
        information about Cloud Storage namespace paths, see
        [Object name
        considerations](https://cloud.google.com/storage/docs/naming#object-considerations).
        
        The resulting files will be nested deeper than the specified URL prefix.
        The final output URL will be provided in the
        google.datastore.admin.v1.ExportEntitiesResponse.output_url field. That
        value should be used for subsequent ImportEntities operations.
        
        By nesting the data files deeper, the same Cloud Storage bucket can be used
        in multiple ExportEntities operations without conflict.


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
