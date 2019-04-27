Retrieves a report which is a collection of properties / statistics for a set of objects.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/admin.reports.usage.readonly* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.reports.usage.readonly*.
You can set the scope for this method like this: `admin1-reports --scope <scope> entity-usage-reports get ...`
# Required Scalar Arguments
* **&lt;entity-type&gt;** *(string)*
    - Type of object. Should be one of - gplus_communities.
* **&lt;entity-key&gt;** *(string)*
    - Represents the key of object for which the data should be filtered.
* **&lt;date&gt;** *(string)*
    - Represents the date in yyyy-mm-dd format for which the data is to be fetched.

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

* **-p parameters=string**
    - Represents the application name, parameter name pairs to fetch in csv as app_name1:param_name1, app_name2:param_name2.

* **-p filters=string**
    - Represents the set of filters including parameter operator value.

* **-p customer-id=string**
    - Represents the customer for which the data is to be fetched.

* **-p max-results=integer**
    - Maximum number of results to return. Maximum allowed is 1000

* **-p page-token=string**
    - Token to specify next page.

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
