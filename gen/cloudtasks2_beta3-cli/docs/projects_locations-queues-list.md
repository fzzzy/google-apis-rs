Lists queues.

Queues are returned in lexicographical order.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudtasks2-beta3 --scope <scope> projects locations-queues-list ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required.
        
        The location name.
        For example: `projects/PROJECT_ID/locations/LOCATION_ID`

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

* **-p page-size=integer**
    - Requested page size.
        
        The maximum page size is 9800. If unspecified, the page size will
        be the maximum. Fewer queues than requested might be returned,
        even if more queues exist; use the
        next_page_token in the
        response to determine if more queues exist.

* **-p page-token=string**
    - A token identifying the page of results to return.
        
        To request the first page results, page_token must be empty. To
        request the next page of results, page_token must be the value of
        next_page_token returned
        from the previous call to ListQueues
        method. It is an error to switch the value of the
        filter while iterating through pages.

* **-p filter=string**
    - `filter` can be used to specify a subset of queues. Any Queue
        field can be used as a filter and several operators as supported.
        For example: `&lt;=, &lt;, &gt;=, &gt;, !=, =, :`. The filter syntax is the same as
        described in
        [Stackdriver&#39;s Advanced Logs Filters](https://cloud.google.com/logging/docs/view/advanced_filters).
        
        Sample filter &#34;state: PAUSED&#34;.
        
        Note that using filters might cause fewer queues than the
        requested page_size to be returned.

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
