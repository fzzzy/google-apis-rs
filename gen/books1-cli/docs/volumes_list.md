Performs a book search.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/books* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/books*.
You can set the scope for this method like this: `books1 --scope <scope> volumes list ...`
# Required Scalar Argument
* **&lt;q&gt;** *(string)*
    - Full-text search query string.

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

* **-p projection=string**
    - Restrict information returned to a set of selected fields.

* **-p max-allowed-maturity-rating=string**
    - The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out.

* **-p start-index=integer**
    - Index of the first result to return (starts at 0)

* **-p download=string**
    - Restrict to volumes by download availability.

* **-p source=string**
    - String to identify the originator of this request.

* **-p max-results=integer**
    - Maximum number of results to return.

* **-p partner=string**
    - Restrict and brand results for partner ID.

* **-p show-preorders=boolean**
    - Set to true to show books available for preorder. Defaults to false.

* **-p print-type=string**
    - Restrict to books or magazines.

* **-p library-restrict=string**
    - Restrict search to this user&#39;s library.

* **-p lang-restrict=string**
    - Restrict results to books with this language code.

* **-p filter=string**
    - Filter search results.

* **-p order-by=string**
    - Sort search results.

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
