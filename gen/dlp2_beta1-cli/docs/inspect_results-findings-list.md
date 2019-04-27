Returns list of results for given inspect operation result set id.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dlp2-beta1 --scope <scope> inspect results-findings-list ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Identifier of the results set returned as metadata of
        the longrunning operation created by a call to InspectDataSource.
        Should be in the format of `inspect/results/{id}`.

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

* **-p filter=string**
    - Restricts findings to items that match. Supports info_type and likelihood.
        
        Examples:
        
        - info_type=EMAIL_ADDRESS
        - info_type=PHONE_NUMBER,EMAIL_ADDRESS
        - likelihood=VERY_LIKELY
        - likelihood=VERY_LIKELY,LIKELY
        - info_type=EMAIL_ADDRESS,likelihood=VERY_LIKELY,LIKELY

* **-p page-token=string**
    - The value returned by the last `ListInspectFindingsResponse`; indicates
        that this is a continuation of a prior `ListInspectFindings` call, and that
        the system should return the next page of data.

* **-p page-size=integer**
    - Maximum number of results to return.
        If 0, the implementation selects a reasonable value.

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
