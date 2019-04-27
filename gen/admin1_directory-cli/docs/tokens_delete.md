Delete all access tokens issued by a user for an application.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/admin.directory.user.security* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.directory.user.security*.
You can set the scope for this method like this: `admin1-directory --scope <scope> tokens delete ...`
# Required Scalar Arguments
* **&lt;user-key&gt;** *(string)*
    - Identifies the user in the API request. The value can be the user&#39;s primary email address, alias email address, or unique user ID.
* **&lt;client-id&gt;** *(string)*
    - The Client ID of the application the token is issued to.
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
