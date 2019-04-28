Batch gets the update history of assets that overlap a time window.
For RESOURCE content, this API outputs history with asset in both
non-delete or deleted status.
For IAM_POLICY content, this API outputs history when the asset and its
attached IAM POLICY both exist. This can create gaps in the output history.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudasset1-beta1 --scope <scope> projects batch-get-assets-history ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The relative name of the root asset. It can only be an
        organization number (such as &#34;organizations/123&#34;), a project ID (such as
        &#34;projects/my-project-id&#34;)&#34;, or a project number (such as &#34;projects/12345&#34;).

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

* **-p asset-names=string**
    - A list of the full names of the assets. For example:
        `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
        See [Resource Names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
        for more info.
        
        The request becomes a no-op if the asset name list is empty, and the max
        size of the asset name list is 100 in one request.

* **-p read-time-window-end-time=string**
    - End time of the time window (exclusive).
        Current timestamp if not specified.

* **-p content-type=string**
    - Required. The content type.

* **-p read-time-window-start-time=string**
    - Start time of the time window (inclusive).

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