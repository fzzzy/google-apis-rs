Retrieves a list of user roles, possibly filtered. This method supports paging.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3d2 --scope <scope> user-roles list ...`
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

* **-p ids=string**
    - Select only user roles with the specified IDs.

* **-p page-token=string**
    - Value of the nextPageToken from the previous result page.

* **-p sort-field=string**
    - Field by which to sort the list.

* **-p subaccount-id=string**
    - Select only user roles that belong to this subaccount.

* **-p account-user-role-only=boolean**
    - Select only account level user roles not associated with any specific subaccount.

* **-p search-string=string**
    - Allows searching for objects by name or ID. Wildcards (*) are allowed. For example, &#34;userrole*2015&#34; will return objects with names like &#34;userrole June 2015&#34;, &#34;userrole April 2015&#34;, or simply &#34;userrole 2015&#34;. Most of the searches also add wildcards implicitly at the start and the end of the search string. For example, a search string of &#34;userrole&#34; will match objects with name &#34;my userrole&#34;, &#34;userrole 2015&#34;, or simply &#34;userrole&#34;.

* **-p max-results=integer**
    - Maximum number of results to return.

* **-p sort-order=string**
    - Order of sorted results.

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
