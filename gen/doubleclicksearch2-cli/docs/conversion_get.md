Retrieves a list of conversions from a DoubleClick Search engine account.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/doubleclicksearch* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/doubleclicksearch*.
You can set the scope for this method like this: `doubleclicksearch2 --scope <scope> conversion get ...`
# Required Scalar Arguments
* **&lt;agency-id&gt;** *(string)*
    - Numeric ID of the agency.
* **&lt;advertiser-id&gt;** *(string)*
    - Numeric ID of the advertiser.
* **&lt;engine-account-id&gt;** *(string)*
    - Numeric ID of the engine account.
* **&lt;end-date&gt;** *(integer)*
    - Last date (inclusive) on which to retrieve conversions. Format is yyyymmdd.
* **&lt;row-count&gt;** *(integer)*
    - The number of conversions to return per call.
* **&lt;start-date&gt;** *(integer)*
    - First date (inclusive) on which to retrieve conversions. Format is yyyymmdd.
* **&lt;start-row&gt;** *(integer)*
    - The 0-based starting index for retrieving conversions results.

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

* **-p campaign-id=string**
    - Numeric ID of the campaign.

* **-p ad-group-id=string**
    - Numeric ID of the ad group.

* **-p ad-id=string**
    - Numeric ID of the ad.

* **-p criterion-id=string**
    - Numeric ID of the criterion.

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
