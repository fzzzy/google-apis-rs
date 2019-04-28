Lists the statuses of the products in your Merchant Center account.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/content* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/content*.
You can set the scope for this method like this: `content2 --scope <scope> productstatuses list ...`
# Required Scalar Argument
* **&lt;merchant-id&gt;** *(string)*
    - The ID of the account that contains the products. This account cannot be a multi-client account.

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
    - The token returned by the previous request.

* **-p include-invalid-inserted-items=boolean**
    - Flag to include the invalid inserted items in the result of the list request. By default the invalid items are not shown (the default value is false).

* **-p max-results=integer**
    - The maximum number of product statuses to return in the response, used for paging.

* **-p destinations=string**
    - If set, only issues for the specified destinations are returned, otherwise only issues for the Shopping destination.

* **-p include-attributes=boolean**
    - Flag to include full product data in the results of the list request. The default value is false.

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