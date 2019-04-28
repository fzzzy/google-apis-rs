List devices in a device registry.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloudiot*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudiot1 --scope <scope> projects locations-registries-groups-devices-list ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - The device registry path. Required. For example,
        `projects/my-project/locations/us-central1/registries/my-registry`.

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

* **-p device-ids=string**
    - A list of device string IDs. For example, `[&#39;device0&#39;, &#39;device12&#39;]`.
        If empty, this field is ignored. Maximum IDs: 10,000

* **-p page-size=integer**
    - The maximum number of devices to return in the response. If this value
        is zero, the service will select a default size. A call may return fewer
        objects than requested. A non-empty `next_page_token` in the response
        indicates that more data is available.

* **-p device-num-ids=string**
    - A list of device numeric IDs. If empty, this field is ignored. Maximum
        IDs: 10,000.

* **-p field-mask=string**
    - The fields of the `Device` resource to be returned in the response. The
        fields `id` and `num_id` are always returned, along with any
        other fields specified.

* **-p page-token=string**
    - The value returned by the last `ListDevicesResponse`; indicates
        that this is a continuation of a prior `ListDevices` call and
        the system should return the next page of data.

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