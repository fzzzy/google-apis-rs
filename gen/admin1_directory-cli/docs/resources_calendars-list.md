Retrieves a list of calendar resources for an account.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/admin.directory.resource.calendar*
* *https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly*.
You can set the scope for this method like this: `admin1-directory --scope <scope> resources calendars-list ...`
# Required Scalar Argument
* **&lt;customer&gt;** *(string)*
    - The unique ID for the customer&#39;s G Suite account. As an account administrator, you can also use the my_customer alias to represent your account&#39;s customer ID.

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

* **-p order-by=string**
    - Field(s) to sort results by in either ascending or descending order. Supported fields include resourceId, resourceName, capacity, buildingId, and floorName. If no order is specified, defaults to ascending. Should be of the form &#34;field [asc|desc], field [asc|desc], ...&#34;. For example buildingId, capacity desc would return results sorted first by buildingId in ascending order then by capacity in descending order.

* **-p page-token=string**
    - Token to specify the next page in the list.

* **-p max-results=integer**
    - Maximum number of results to return.

* **-p query=string**
    - String query used to filter results. Should be of the form &#34;field operator value&#34; where field can be any of supported fields and operators can be any of supported operations. Operators include &#39;=&#39; for exact match and &#39;:&#39; for prefix match or HAS match where applicable. For prefix match, the value should always be followed by a *. Supported fields include generatedResourceName, name, buildingId, featureInstances.feature.name. For example buildingId=US-NYC-9TH AND featureInstances.feature.name:Phone.

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
