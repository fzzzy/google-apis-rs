Retrieves list of reports.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfareporting* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfareporting*.
You can set the scope for this method like this: `dfareporting3 --scope <scope> reports list ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - The DFA user profile ID.

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

* **-p scope=string**
    - The scope that defines which results are returned.

* **-p page-token=string**
    - The value of the nextToken from the previous result page.

* **-p sort-order=string**
    - Order of sorted results.

* **-p max-results=integer**
    - Maximum number of results to return.

* **-p sort-field=string**
    - The field by which to sort the list.

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
