Updates an existing variable with a new value.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloudruntimeconfig*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `runtimeconfig1-beta1 --scope <scope> projects configs-variables-update ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The name of the variable to update, in the format:
        
        `projects/[PROJECT_ID]/configs/[CONFIG_NAME]/variables/[VARIABLE_NAME]`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Variable:
  name: string
  state: string
  text: string
  update-time: string
  value: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    name=nonumy`
    - The name of the variable resource, in the format:
        
            projects/[PROJECT_ID]/configs/[CONFIG_NAME]/variables/[VARIABLE_NAME]
        
        The `[PROJECT_ID]` must be a valid project ID, `[CONFIG_NAME]` must be a
        valid RuntimeConfig reource and `[VARIABLE_NAME]` follows Unix file system
        file path naming.
        
        The `[VARIABLE_NAME]` can contain ASCII letters, numbers, slashes and
        dashes. Slashes are used as path element separators and are not part of the
        `[VARIABLE_NAME]` itself, so `[VARIABLE_NAME]` must contain at least one
        non-slash character. Multiple slashes are coalesced into single slash
        character. Each path segment should match
        [0-9A-Za-z](?:[_.A-Za-z0-9-]{0,62}[_.A-Za-z0-9])? regular expression.
        The length of a `[VARIABLE_NAME]` must be less than 256 characters.
        
        Once you create a variable, you cannot change the variable name.
* `state=dolores`
    - [Ouput only] The current state of the variable. The variable state indicates
        the outcome of the `variables().watch` call and is visible through the
        `get` and `list` calls.
* `text=gubergren`
    - The string value of the variable. The length of the value must be less
        than 4096 bytes. Empty values are also accepted. For example,
        `text: &#34;my text value&#34;`. The string must be valid UTF-8.
* `update-time=sadipscing`
    - Output only. The time of the last variable update.
* `value=aliquyam`
    - The binary value of the variable. The length of the value must be less
        than 4096 bytes. Empty values are also accepted. The value must be
        base64 encoded. Only one of `value` or `text` can be set.


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
