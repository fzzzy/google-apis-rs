Returns information about running and completed jobs.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/bigquery*
* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloud-platform.read-only*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/bigquery*.
You can set the scope for this method like this: `bigquerydatatransfer1 --scope <scope> projects transfer-configs-runs-list ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Name of transfer configuration for which transfer runs should be retrieved.
        Format of transfer configuration resource name is:
        `projects/{project_id}/transferConfigs/{config_id}`.

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

* **-p run-attempt=string**
    - Indicates how run attempts are to be pulled.

* **-p page-token=string**
    - Pagination token, which can be used to request a specific page
        of `ListTransferRunsRequest` list results. For multiple-page
        results, `ListTransferRunsResponse` outputs
        a `next_page` token, which can be used as the
        `page_token` value to request the next page of list results.

* **-p page-size=integer**
    - Page size. The default page size is the maximum value of 1000 results.

* **-p states=string**
    - When specified, only transfer runs with requested states are returned.

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
