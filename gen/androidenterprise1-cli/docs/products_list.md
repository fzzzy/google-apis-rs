Finds approved products that match a query, or all approved products if there is no query.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidenterprise* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidenterprise*.
You can set the scope for this method like this: `androidenterprise1 --scope <scope> products list ...`
# Required Scalar Argument
* **&lt;enterprise-id&gt;** *(string)*
    - The ID of the enterprise.

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

* **-p query=string**
    - The search query as typed in the Google Play store search box. If omitted, all approved apps will be returned (using the pagination parameters), including apps that are not available in the store (e.g. unpublished apps).

* **-p approved=boolean**
    - Specifies whether to search among all products (false) or among only products that have been approved (true). Only &#34;true&#34; is supported, and should be specified.

* **-p token=string**
    - A pagination token is contained in a request&#39;&#39;s response when there are more products. The token can be used in a subsequent request to obtain more products, and so forth. This parameter cannot be used in the initial request.

* **-p language=string**
    - The BCP47 tag for the user&#39;s preferred language (e.g. &#34;en-US&#34;, &#34;de&#34;). Results are returned in the language best matching the preferred language.

* **-p max-results=integer**
    - Specifies the maximum number of products that can be returned per request. If not specified, uses a default value of 100, which is also the maximum retrievable within a single response.

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
