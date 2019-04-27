Returns Analytics data for a view (profile).
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/analytics*
* *https://www.googleapis.com/auth/analytics.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/analytics.readonly*.
You can set the scope for this method like this: `analytics3 --scope <scope> data ga-get ...`
# Required Scalar Arguments
* **&lt;ids&gt;** *(string)*
    - Unique table ID for retrieving Analytics data. Table ID is of the form ga:XXXX, where XXXX is the Analytics view (profile) ID.
* **&lt;start-date&gt;** *(string)*
    - Start date for fetching Analytics data. Requests can specify a start date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is 7daysAgo.
* **&lt;end-date&gt;** *(string)*
    - End date for fetching Analytics data. Request can should specify an end date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is yesterday.
* **&lt;metrics&gt;** *(string)*
    - A comma-separated list of Analytics metrics. E.g., &#39;ga:sessions,ga:pageviews&#39;. At least one metric must be specified.

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

* **-p max-results=integer**
    - The maximum number of entries to include in this feed.

* **-p sort=string**
    - A comma-separated list of dimensions or metrics that determine the sort order for Analytics data.

* **-p dimensions=string**
    - A comma-separated list of Analytics dimensions. E.g., &#39;ga:browser,ga:city&#39;.

* **-p start-index=integer**
    - An index of the first entity to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter.

* **-p include-empty-rows=boolean**
    - The response will include empty rows if this parameter is set to true, the default is true

* **-p sampling-level=string**
    - The desired sampling level.

* **-p filters=string**
    - A comma-separated list of dimension or metric filters to be applied to Analytics data.

* **-p segment=string**
    - An Analytics segment to be applied to data.

* **-p output=string**
    - The selected format for the response. Default format is JSON.

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
