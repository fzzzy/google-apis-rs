Lists all Redis instances owned by a project in either the specified
location (region) or all locations.

The location should have the following format:
* `projects/{project_id}/locations/{location_id}`

If `location_id` is specified as `-` (wildcard), then all regions
available to the project are queried, and the results are aggregated.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `redis1 --scope <scope> projects locations-instances-list ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The resource name of the instance location using the form:
            `projects/{project_id}/locations/{location_id}`
        where `location_id` refers to a GCP region

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
    - The maximum number of items to return.
        
        If not specified, a default value of 1000 will be used by the service.
        Regardless of the page_size value, the response may include a partial list
        and a caller should only rely on response&#39;s
        next_page_token
        to determine if there are more instances left to be queried.

* **-p page-token=string**
    - The next_page_token value returned from a previous List request,
        if any.

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
