Retrieves a building.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/admin.directory.resource.calendar*
* *https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly*.
You can set the scope for this method like this: `admin1-directory --scope <scope> resources buildings-get ...`
# Required Scalar Arguments
* **&lt;customer&gt;** *(string)*
    - The unique ID for the customer&#39;s G Suite account. As an account administrator, you can also use the my_customer alias to represent your account&#39;s customer ID.
* **&lt;building-id&gt;** *(string)*
    - The unique ID of the building to retrieve.

# Optional Output Flags

The method's return value a JSON encoded structure, which will be written to standard output by default.

* **-o out**
    - *out* specifies the *destination* to which to write the server's result to.
      It will be a JSON-encoded structure.
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
