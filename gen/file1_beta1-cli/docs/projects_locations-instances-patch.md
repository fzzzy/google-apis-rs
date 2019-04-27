Updates the settings of a specific instance.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `file1-beta1 --scope <scope> projects locations-instances-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Output only.
        The resource name of the instance, in the format
        projects/{project_id}/locations/{location_id}/instances/{instance_id}.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Instance:
  create-time: string
  description: string
  etag: string
  labels: { string: string }
  name: string
  state: string
  status-message: string
  tier: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    create-time=takimata`
    - Output only.
        The time when the instance was created.
* `description=justo`
    - Optional. A description of the instance (2048 characters or less).
* `etag=amet.`
    - Server-specified ETag for the instance resource to prevent simultaneous
        updates from overwriting each other.
* `labels=key=erat`
    - Resource labels to represent user provided metadata.
    - the value will be associated with the given `key`
* `name=labore`
    - Output only.
        The resource name of the instance, in the format
        projects/{project_id}/locations/{location_id}/instances/{instance_id}.
* `state=sea`
    - Output only.
        The instance state.
* `status-message=nonumy`
    - Output only.
        Additional information about the instance state, if available.
* `tier=dolores`
    - The service tier of the instance.


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

* **-p update-mask=string**
    - Mask of fields to update.  At least one path must be supplied in this
        field.  The elements of the repeated paths field may only include these
        fields:
        &#34;description&#34;

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
