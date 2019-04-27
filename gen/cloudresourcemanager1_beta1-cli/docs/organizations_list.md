Lists Organization resources that are visible to the user and satisfy
the specified filter. This method returns Organizations in an unspecified
order. New Organizations do not necessarily appear at the end of the list.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloud-platform.read-only*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudresourcemanager1-beta1 --scope <scope> organizations list ...`

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
    - A pagination token returned from a previous call to `ListOrganizations`
        that indicates from where listing should continue.
        This field is optional.

* **-p page-size=integer**
    - The maximum number of Organizations to return in the response.
        This field is optional.

* **-p filter=string**
    - An optional query string used to filter the Organizations to return in
        the response. Filter rules are case-insensitive.
        
        
        Organizations may be filtered by `owner.directoryCustomerId` or by
        `domain`, where the domain is a G Suite domain, for example:
        
        |Filter|Description|
        |------|-----------|
        |owner.directorycustomerid:123456789|Organizations with `owner.directory_customer_id` equal to `123456789`.|
        |domain:google.com|Organizations corresponding to the domain `google.com`.|
        
        This field is optional.

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
