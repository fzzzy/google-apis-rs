Updates a ServiceAccount.

Currently, only the following fields are updatable:
`display_name` .
The `etag` is mandatory.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `iam1 --scope <scope> projects service-accounts-update ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The resource name of the service account in the following format:
        `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
        
        Requests using `-` as a wildcard for the `PROJECT_ID` will infer the
        project from the `account` and the `ACCOUNT` value can be the `email`
        address or the `unique_id` of the service account.
        
        In responses the resource name will always be in the format
        `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ServiceAccount:
  display-name: string
  email: string
  etag: string
  name: string
  oauth2-client-id: string
  project-id: string
  unique-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    display-name=consetetur`
    - Optional. A user-specified name for the service account.
        Must be less than or equal to 100 UTF-8 bytes.
* `email=amet.`
    - @OutputOnly The email address of the service account.
* `etag=voluptua.`
    - Optional. Note: `etag` is an inoperable legacy field that is only returned
        for backwards compatibility.
* `name=lorem`
    - The resource name of the service account in the following format:
        `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
        
        Requests using `-` as a wildcard for the `PROJECT_ID` will infer the
        project from the `account` and the `ACCOUNT` value can be the `email`
        address or the `unique_id` of the service account.
        
        In responses the resource name will always be in the format
        `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
* `oauth2-client-id=gubergren`
    - @OutputOnly The OAuth2 client id for the service account.
        This is used in conjunction with the OAuth2 clientconfig API to make
        three legged OAuth2 (3LO) flows to access the data of Google users.
* `project-id=justo`
    - @OutputOnly The id of the project that owns the service account.
* `unique-id=sit`
    - @OutputOnly The unique and stable id of the service account.


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
