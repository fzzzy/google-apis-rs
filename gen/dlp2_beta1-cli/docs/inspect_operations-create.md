Schedules a job scanning content in a Google Cloud Platform data
repository.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dlp2-beta1 --scope <scope> inspect operations-create ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GooglePrivacyDlpV2beta1CreateInspectOperationRequest:
  inspect-config:
    exclude-types: boolean
    include-quote: boolean
    max-findings: integer
    min-likelihood: string
  operation-config:
    max-item-findings: string
  output-config:
    storage-path:
      path: string
    table:
      dataset-id: string
      project-id: string
      table-id: string
  storage-config:
    big-query-options:
      table-reference:
        dataset-id: string
        project-id: string
        table-id: string
    cloud-storage-options:
      file-set:
        url: string
    datastore-options:
      kind:
        name: string
      partition-id:
        namespace-id: string
        project-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .inspect-config    exclude-types=false`
    - When true, excludes type information of the findings.
* `include-quote=true`
    - When true, a contextual quote from the data that triggered a finding is
        included in the response; see Finding.quote.
* `max-findings=80`
    - Limits the number of findings per content item or long running operation.
* `min-likelihood=et`
    - Only returns findings equal or above this threshold.

* `..operation-config    max-item-findings=et`
    - Max number of findings per file, Datastore entity, or database row.

* `..output-config.storage-path    path=diam`
    - The url, in the format of `gs://bucket/&lt;path&gt;`.

* `..table    dataset-id=ipsum`
    - Dataset ID of the table.
* `project-id=lorem`
    - The Google Cloud Platform project ID of the project containing the table.
        If omitted, project ID is inferred from the API call.
* `table-id=et`
    - Name of the table.


* `...storage-config.big-query-options.table-reference    dataset-id=duo`
    - Dataset ID of the table.
* `project-id=aliquyam`
    - The Google Cloud Platform project ID of the project containing the table.
        If omitted, project ID is inferred from the API call.
* `table-id=sea`
    - Name of the table.


* `...cloud-storage-options.file-set    url=lorem`
    - The url, in the format `gs://&lt;bucket&gt;/&lt;path&gt;`. Trailing wildcard in the
        path is allowed.


* `...datastore-options.kind    name=eos`
    - The name of the kind.

* `..partition-id    namespace-id=erat`
    - If not empty, the ID of the namespace to which the entities belong.
* `project-id=sadipscing`
    - The ID of the project to which the entities belong.





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

* **-p bearer-token=string**
    - OAuth bearer token.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pp=boolean**
    - Pretty-print response.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).
