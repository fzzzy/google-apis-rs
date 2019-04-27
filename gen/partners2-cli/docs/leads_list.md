Lists advertiser leads for a user&#39;s associated company.
Should only be called within the context of an authorized logged in user.

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

* **-p request-metadata-locale=string**
    - Locale to use for the current request.

* **-p page-token=string**
    - A token identifying a page of results that the server returns.
        Typically, this is the value of `ListLeadsResponse.next_page_token`
        returned from the previous call to
        ListLeads.

* **-p request-metadata-user-overrides-user-id=string**
    - Logged-in user ID to impersonate instead of the user&#39;s ID.

* **-p page-size=integer**
    - Requested page size. Server may return fewer leads than requested.
        If unspecified, server picks an appropriate default.

* **-p request-metadata-user-overrides-ip-address=string**
    - IP address to use instead of the user&#39;s geo-located IP address.

* **-p request-metadata-partners-session-id=string**
    - Google Partners session ID.

* **-p request-metadata-traffic-source-traffic-source-id=string**
    - Identifier to indicate where the traffic comes from.
        An identifier has multiple letters created by a team which redirected the
        traffic to us.

* **-p order-by=string**
    - How to order Leads. Currently, only `create_time`
        and `create_time desc` are supported

* **-p request-metadata-experiment-ids=string**
    - Experiment IDs the current request belongs to.

* **-p request-metadata-traffic-source-traffic-sub-id=string**
    - Second level identifier to indicate where the traffic comes from.
        An identifier has multiple letters created by a team which redirected the
        traffic to us.

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
