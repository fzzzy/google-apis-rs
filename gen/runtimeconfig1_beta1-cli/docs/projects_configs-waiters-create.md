Creates a Waiter resource. This operation returns a long-running Operation
resource which can be polled for completion. However, a waiter with the
given name will exist (and can be retrieved) prior to the operation
completing. If the operation fails, the failed Waiter resource will
still exist and must be deleted prior to subsequent creation attempts.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloudruntimeconfig*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `runtimeconfig1-beta1 --scope <scope> projects configs-waiters-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - The path to the configuration that will own the waiter.
        The configuration must exist beforehand; the path must be in the format:
        
        `projects/[PROJECT_ID]/configs/[CONFIG_NAME]`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Waiter:
  create-time: string
  done: boolean
  error:
    code: integer
    message: string
  failure:
    cardinality:
      number: integer
      path: string
  name: string
  success:
    cardinality:
      number: integer
      path: string
  timeout: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    create-time=no`
    - Output only. The instant at which this Waiter resource was created. Adding
        the value of `timeout` to this instant yields the timeout deadline for the
        waiter.
* `done=true`
    - Output only. If the value is `false`, it means the waiter is still waiting
        for one of its conditions to be met.
        
        If true, the waiter has finished. If the waiter finished due to a timeout
        or failure, `error` will be set.
* `error    code=80`
    - The status code, which should be an enum value of google.rpc.Code.
* `message=et`
    - A developer-facing error message, which should be in English. Any
        user-facing error message should be localized and sent in the
        google.rpc.Status.details field, or localized by the client.

* `..failure.cardinality    number=84`
    - The number variables under the `path` that must exist to meet this
        condition. Defaults to 1 if not specified.
* `path=diam`
    - The root of the variable subtree to monitor. For example, `/foo`.


* `...    name=ipsum`
    - The name of the Waiter resource, in the format:
        
            projects/[PROJECT_ID]/configs/[CONFIG_NAME]/waiters/[WAITER_NAME]
        
        The `[PROJECT_ID]` must be a valid Google Cloud project ID,
        the `[CONFIG_NAME]` must be a valid RuntimeConfig resource, the
        `[WAITER_NAME]` must match RFC 1035 segment specification, and the length
        of `[WAITER_NAME]` must be less than 64 bytes.
        
        After you create a Waiter resource, you cannot change the resource name.
* `success.cardinality    number=96`
    - The number variables under the `path` that must exist to meet this
        condition. Defaults to 1 if not specified.
* `path=et`
    - The root of the variable subtree to monitor. For example, `/foo`.


* `...    timeout=duo`
    - [Required] Specifies the timeout of the waiter in seconds, beginning from
        the instant that `waiters().create` method is called. If this time elapses
        before the success or failure conditions are met, the waiter fails and sets
        the `error` code to `DEADLINE_EXCEEDED`.


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

* **-p request-id=string**
    - An optional but recommended unique `request_id`. If the server
        receives two `create()` requests  with the same
        `request_id`, then the second request will be ignored and the
        first resource created and stored in the backend is returned.
        Empty `request_id` fields are ignored.
        
        It is responsibility of the client to ensure uniqueness of the
        `request_id` strings.
        
        `request_id` strings are limited to 64 characters.

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
