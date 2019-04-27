Deletes a building.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/admin.directory.resource.calendar* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.directory.resource.calendar*.
You can set the scope for this method like this: `admin1-directory --scope <scope> resources buildings-delete ...`
# Required Scalar Arguments
* **&lt;customer&gt;** *(string)*
    - The unique ID for the customer&#39;s G Suite account. As an account administrator, you can also use the my_customer alias to represent your account&#39;s customer ID.
* **&lt;building-id&gt;** *(string)*
    - The ID of the building to delete.
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
