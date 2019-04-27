Retrieves a list of activities for a specific customer and application.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/admin.reports.audit.readonly* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.reports.audit.readonly*.
You can set the scope for this method like this: `admin1-reports --scope <scope> activities list ...`
# Required Scalar Arguments
* **&lt;user-key&gt;** *(string)*
    - Represents the profile id or the user email for which the data should be filtered. When &#39;all&#39; is specified as the userKey, it returns usageReports for all users.
* **&lt;application-name&gt;** *(string)*
    - Application name for which the events are to be retrieved.

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

* **-p filters=string**
    - Event parameters in the form [parameter1 name][operator][parameter1 value],[parameter2 name][operator][parameter2 value],...

* **-p actor-ip-address=string**
    - IP Address of host where the event was performed. Supports both IPv4 and IPv6 addresses.

* **-p max-results=integer**
    - Number of activity records to be shown in each page.

* **-p event-name=string**
    - Name of the event being queried.

* **-p page-token=string**
    - Token to specify next page.

* **-p start-time=string**
    - Return events which occurred at or after this time.

* **-p customer-id=string**
    - Represents the customer for which the data is to be fetched.

* **-p end-time=string**
    - Return events which occurred at or before this time.

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
