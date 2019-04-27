Analyzes the syntax of the text and provides sentence boundaries and
tokenization along with part of speech tags, dependency trees, and other
properties.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-language*
* *https://www.googleapis.com/auth/cloud-platform*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-language*.
You can set the scope for this method like this: `language1 --scope <scope> documents analyze-syntax ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
AnalyzeSyntaxRequest:
  document:
    content: string
    gcs-content-uri: string
    language: string
    type: string
  encoding-type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .document    content=dolores`
    - The content of the input in string format.
        Cloud audit logging exempt since it is based on user data.
* `gcs-content-uri=gubergren`
    - The Google Cloud Storage URI where the file content is located.
        This URI must be of the form: gs://bucket_name/object_name. For more
        details, see https://cloud.google.com/storage/docs/reference-uris.
        NOTE: Cloud Storage object versioning is not supported.
* `language=sadipscing`
    - The language of the document (if not specified, the language is
        automatically detected). Both ISO and BCP-47 language codes are
        accepted.&lt;br&gt;
        [Language Support](/natural-language/docs/languages)
        lists currently supported languages for each API method.
        If the language (either specified by the caller or automatically detected)
        is not supported by the called API method, an `INVALID_ARGUMENT` error
        is returned.
* `type=aliquyam`
    - Required. If the type is not set or is `TYPE_UNSPECIFIED`,
        returns an `INVALID_ARGUMENT` error.

* `..    encoding-type=ea`
    - The encoding type used by the API to calculate offsets.


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
