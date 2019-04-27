Retrieves a list of directory sites, possibly filtered. This method supports paging.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3d2 --scope <scope> directory-sites list ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.

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

* **-p accepts-publisher-paid-placements=boolean**
    - Select only directory sites that accept publisher paid placements. This field can be left blank.

* **-p active=boolean**
    - Select only active directory sites. Leave blank to retrieve both active and inactive directory sites.

* **-p max-results=integer**
    - Maximum number of results to return.

* **-p sort-order=string**
    - Order of sorted results.

* **-p page-token=string**
    - Value of the nextPageToken from the previous result page.

* **-p accepts-in-stream-video-placements=boolean**
    - This search filter is no longer supported and will have no effect on the results returned.

* **-p ids=string**
    - Select only directory sites with these IDs.

* **-p sort-field=string**
    - Field by which to sort the list.

* **-p dfp-network-code=string**
    - Select only directory sites with this Ad Manager network code.

* **-p accepts-interstitial-placements=boolean**
    - This search filter is no longer supported and will have no effect on the results returned.

* **-p country-id=string**
    - Select only directory sites with this country ID.

* **-p search-string=string**
    - Allows searching for objects by name, ID or URL. Wildcards (*) are allowed. For example, &#34;directory site*2015&#34; will return objects with names like &#34;directory site June 2015&#34;, &#34;directory site April 2015&#34;, or simply &#34;directory site 2015&#34;. Most of the searches also add wildcards implicitly at the start and the end of the search string. For example, a search string of &#34;directory site&#34; will match objects with name &#34;my directory site&#34;, &#34;directory site 2015&#34; or simply, &#34;directory site&#34;.

* **-p parent-id=string**
    - Select only directory sites with this parent ID.

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
