Returns of a list of traces that match the specified filter conditions.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/trace.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/trace.readonly*.
You can set the scope for this method like this: `cloudtrace1 --scope <scope> projects traces-list ...`
# Required Scalar Argument
* **&lt;project-id&gt;** *(string)*
    - ID of the Cloud project where the trace data is stored.

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

* **-p page-token=string**
    - Token identifying the page of results to return. If provided, use the
        value of the `next_page_token` field from a previous request. Optional.

* **-p page-size=integer**
    - Maximum number of traces to return. If not specified or &lt;= 0, the
        implementation selects a reasonable value.  The implementation may
        return fewer traces than the requested page size. Optional.

* **-p order-by=string**
    - Field used to sort the returned traces. Optional.
        Can be one of the following:
        
        *   `trace_id`
        *   `name` (`name` field of root span in the trace)
        *   `duration` (difference between `end_time` and `start_time` fields of
             the root span)
        *   `start` (`start_time` field of the root span)
        
        Descending order can be specified by appending `desc` to the sort field
        (for example, `name desc`).
        
        Only one sort field is permitted.

* **-p filter=string**
    - An optional filter against labels for the request.
        
        By default, searches use prefix matching. To specify exact match, prepend
        a plus symbol (`+`) to the search term.
        Multiple terms are ANDed. Syntax:
        
        *   `root:NAME_PREFIX` or `NAME_PREFIX`: Return traces where any root
            span starts with `NAME_PREFIX`.
        *   `+root:NAME` or `+NAME`: Return traces where any root span&#39;s name is
            exactly `NAME`.
        *   `span:NAME_PREFIX`: Return traces where any span starts with
            `NAME_PREFIX`.
        *   `+span:NAME`: Return traces where any span&#39;s name is exactly
            `NAME`.
        *   `latency:DURATION`: Return traces whose overall latency is
            greater or equal to than `DURATION`. Accepted units are nanoseconds
            (`ns`), milliseconds (`ms`), and seconds (`s`). Default is `ms`. For
            example, `latency:24ms` returns traces whose overall latency
            is greater than or equal to 24 milliseconds.
        *   `label:LABEL_KEY`: Return all traces containing the specified
            label key (exact match, case-sensitive) regardless of the key:value
            pair&#39;s value (including empty values).
        *   `LABEL_KEY:VALUE_PREFIX`: Return all traces containing the specified
            label key (exact match, case-sensitive) whose value starts with
            `VALUE_PREFIX`. Both a key and a value must be specified.
        *   `+LABEL_KEY:VALUE`: Return all traces containing a key:value pair
            exactly matching the specified text. Both a key and a value must be
            specified.
        *   `method:VALUE`: Equivalent to `/http/method:VALUE`.
        *   `url:VALUE`: Equivalent to `/http/url:VALUE`.

* **-p start-time=string**
    - Start of the time interval (inclusive) during which the trace data was
        collected from the application.

* **-p view=string**
    - Type of data returned for traces in the list. Optional. Default is
        `MINIMAL`.

* **-p end-time=string**
    - End of the time interval (inclusive) during which the trace data was
        collected from the application.

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
