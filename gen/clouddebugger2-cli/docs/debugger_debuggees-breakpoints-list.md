Lists all breakpoints for the debuggee.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloud_debugger*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `clouddebugger2 --scope <scope> debugger debuggees-breakpoints-list ...`
# Required Scalar Argument
* **&lt;debuggee-id&gt;** *(string)*
    - ID of the debuggee whose breakpoints to list.

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

* **-p strip-results=boolean**
    - This field is deprecated. The following fields are always stripped out of
        the result: `stack_frames`, `evaluated_expressions` and `variable_table`.

* **-p wait-token=string**
    - A wait token that, if specified, blocks the call until the breakpoints
        list has changed, or a server selected timeout has expired.  The value
        should be set from the last response. The error code
        `google.rpc.Code.ABORTED` (RPC) is returned on wait timeout, which
        should be called again with the same `wait_token`.

* **-p client-version=string**
    - The client version making the call.
        Schema: `domain/type/version` (e.g., `google.com/intellij/v1`).

* **-p include-all-users=boolean**
    - When set to `true`, the response includes the list of breakpoints set by
        any user. Otherwise, it includes only breakpoints set by the caller.

* **-p include-inactive=boolean**
    - When set to `true`, the response includes active and inactive
        breakpoints. Otherwise, it includes only active breakpoints.

* **-p action-value=string**
    - Only breakpoints with the specified action will pass the filter.

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
