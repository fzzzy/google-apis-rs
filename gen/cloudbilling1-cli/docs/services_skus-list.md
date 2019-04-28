Lists all publicly available SKUs for a given cloud service.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudbilling1 --scope <scope> services skus-list ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - The name of the service.
        Example: &#34;services/DA34-426B-A397&#34;

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

* **-p start-time=string**
    - Optional inclusive start time of the time range for which the pricing
        versions will be returned. Timestamps in the future are not allowed.
        The time range has to be within a single calendar month in
        America/Los_Angeles timezone. Time range as a whole is optional. If not
        specified, the latest pricing will be returned (up to 12 hours old at
        most).

* **-p end-time=string**
    - Optional exclusive end time of the time range for which the pricing
        versions will be returned. Timestamps in the future are not allowed.
        The time range has to be within a single calendar month in
        America/Los_Angeles timezone. Time range as a whole is optional. If not
        specified, the latest pricing will be returned (up to 12 hours old at
        most).

* **-p page-size=integer**
    - Requested page size. Defaults to 5000.

* **-p page-token=string**
    - A token identifying a page of results to return. This should be a
        `next_page_token` value returned from a previous `ListSkus`
        call. If unspecified, the first page of results is returned.

* **-p currency-code=string**
    - The ISO 4217 currency code for the pricing info in the response proto.
        Will use the conversion rate as of start_time.
        Optional. If not specified USD will be used.

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