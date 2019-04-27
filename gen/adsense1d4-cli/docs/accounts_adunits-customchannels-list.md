List all custom channels which the specified ad unit belongs to.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/adsense*
* *https://www.googleapis.com/auth/adsense.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adsense.readonly*.
You can set the scope for this method like this: `adsense1d4 --scope <scope> accounts adunits-customchannels-list ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - Account to which the ad client belongs.
* **&lt;ad-client-id&gt;** *(string)*
    - Ad client which contains the ad unit.
* **&lt;ad-unit-id&gt;** *(string)*
    - Ad unit for which to list custom channels.

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
    - A continuation token, used to page through custom channels. To retrieve the next page, set this parameter to the value of &#34;nextPageToken&#34; from the previous response.

* **-p max-results=integer**
    - The maximum number of custom channels to include in the response, used for paging.

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p alt=string**
    - Data format for the response.

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.

* **-p user-ip=string**
    - Deprecated. Please use quotaUser instead.
