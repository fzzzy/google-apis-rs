Update a RemoteConfig. We treat this as an always-existing
resource (when it is not found in our data store, we treat it as version
0, a template with zero conditions and zero parameters). Hence there are
no Create or Delete operations. Returns the updated template when
successful (and the updated eTag as a response header), or an error if
things go wrong.
Possible error messages:
* VALIDATION_ERROR (HTTP status 400) with additional details if the
template being passed in can not be validated.
* AUTHENTICATION_ERROR (HTTP status 401) if the request can not be
authenticate (e.g. no access token, or invalid access token).
* AUTHORIZATION_ERROR (HTTP status 403) if the request can not be
authorized (e.g. the user has no access to the specified project id).
* VERSION_MISMATCH (HTTP status 412) when trying to update when the
expected eTag (passed in via the &#34;If-match&#34; header) is not specified, or
is specified but does does not match the current eTag.
* Internal error (HTTP status 500) for Database problems or other internal
errors.
# Required Scalar Argument
* **&lt;project&gt;** *(string)*
    - The GMP project identifier. Required.
        See note at the beginning of this file regarding project ids.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
RemoteConfig:

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.



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
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p validate-only=boolean**
    - Optional. Defaults to &lt;code&gt;false&lt;/code&gt; (UpdateRemoteConfig call should
        update the backend if there are no validation/interal errors). May be set
        to &lt;code&gt;true&lt;/code&gt; to indicate that, should no validation errors occur,
        the call should return a &#34;200 OK&#34; instead of performing the update. Note
        that other error messages (500 Internal Error, 412 Version Mismatch, etc)
        may still result after flipping to &lt;code&gt;false&lt;/code&gt;, even if getting a
        &#34;200 OK&#34; when calling with &lt;code&gt;true&lt;/code&gt;.

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
