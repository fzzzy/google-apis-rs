Deletes a timeline item.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/glass.location*
* *https://www.googleapis.com/auth/glass.timeline*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/glass.location*.
You can set the scope for this method like this: `mirror1 --scope <scope> timeline delete ...`
# Required Scalar Argument
* **&lt;id&gt;** *(string)*
    - The ID of the timeline item.
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