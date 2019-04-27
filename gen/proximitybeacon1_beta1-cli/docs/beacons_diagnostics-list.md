List the diagnostics for a single beacon. You can also list diagnostics for
all the beacons owned by your Google Developers Console project by using
the beacon name `beacons/-`.

Authenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)
from a signed-in user with **viewer**, **Is owner** or **Can edit**
permissions in the Google Developers Console project.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/userlocation.beacon.registry* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/userlocation.beacon.registry*.
You can set the scope for this method like this: `proximitybeacon1-beta1 --scope <scope> beacons diagnostics-list ...`
# Required Scalar Argument
* **&lt;beacon-name&gt;** *(string)*
    - Beacon that the diagnostics are for.

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
    - Requests results that occur after the `page_token`, obtained from the
        response to a previous request. Optional.

* **-p page-size=integer**
    - Specifies the maximum number of results to return. Defaults to
        10. Maximum 1000. Optional.

* **-p project-id=string**
    - Requests only diagnostic records for the given project id. If not set,
        then the project making the request will be used for looking up
        diagnostic records. Optional.

* **-p alert-filter=string**
    - Requests only beacons that have the given alert. For example, to find
        beacons that have low batteries use `alert_filter=LOW_BATTERY`.

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
