Downloads a report file encoded in UTF-8.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/doubleclicksearch* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/doubleclicksearch*.
You can set the scope for this method like this: `doubleclicksearch2 --scope <scope> reports get-file ...`
# Required Scalar Arguments
* **&lt;report-id&gt;** *(string)*
    - ID of the report.
* **&lt;report-fragment&gt;** *(integer)*
    - The index of the report fragment to download.

# Optional Output Flags


The method's return value is a byte stream of the downloadable resource.

* **-o out**
    - *out* specifies the *destination* to which to write the server's result to.
      It will be a byte stream of the downloadable resource.
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
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
