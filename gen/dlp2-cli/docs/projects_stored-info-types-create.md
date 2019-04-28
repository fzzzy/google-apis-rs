Creates a pre-built stored infoType to be used for inspection.
See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
learn more.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dlp2 --scope <scope> projects stored-info-types-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - The parent resource name, for example projects/my-project-id or
        organizations/my-org-id.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GooglePrivacyDlpV2CreateStoredInfoTypeRequest:
  config:
    description: string
    display-name: string
    large-custom-dictionary:
      big-query-field:
        field:
          name: string
        table:
          dataset-id: string
          project-id: string
          table-id: string
      cloud-storage-file-set:
        url: string
      output-path:
        path: string
  stored-info-type-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .config    description=elitr`
    - Description of the StoredInfoType (max 256 characters).
* `display-name=magna`
    - Display name of the StoredInfoType (max 256 characters).
* `large-custom-dictionary.big-query-field.field    name=ipsum`
    - Name describing the field.

* `..table    dataset-id=invidunt`
    - Dataset ID of the table.
* `project-id=accusam`
    - The Google Cloud Platform project ID of the project containing the table.
        If omitted, project ID is inferred from the API call.
* `table-id=labore`
    - Name of the table.


* `...cloud-storage-file-set    url=diam`
    - The url, in the format `gs://&lt;bucket&gt;/&lt;path&gt;`. Trailing wildcard in the
        path is allowed.

* `..output-path    path=nonumy`
    - A url representing a file or path (no wildcards) in Cloud Storage.
        Example: gs://[BUCKET_NAME]/dictionary.txt



* `....    stored-info-type-id=sed`
    - The storedInfoType ID can contain uppercase and lowercase letters,
        numbers, and hyphens; that is, it must match the regular
        expression: `[a-zA-Z\\d-]+`. The maximum length is 100
        characters. Can be empty to allow the system to generate one.


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