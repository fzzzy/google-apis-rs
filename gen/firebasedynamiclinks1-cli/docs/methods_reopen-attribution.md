Get iOS reopen attribution for app universal link open deeplinking.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/firebase* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/firebase*.
You can set the scope for this method like this: `firebasedynamiclinks1 --scope <scope> methods reopen-attribution ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GetIosReopenAttributionRequest:
  bundle-id: string
  requested-link: string
  sdk-version: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    bundle-id=lorem`
    - APP bundle ID.
* `requested-link=sea`
    - FDL link to be verified from an app universal link open.
        The FDL link can be one of:
        1) short FDL.
        e.g. &lt;app_code&gt;.page.link/&lt;ddl_id&gt;, or
        2) long FDL.
        e.g. &lt;app_code&gt;.page.link/?{query params}, or
        3) Invite FDL.
        e.g. &lt;app_code&gt;.page.link/i/&lt;invite_id_or_alias&gt;
* `sdk-version=et`
    - Google SDK version. Version takes the form &#34;$major.$minor.$patch&#34;


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
