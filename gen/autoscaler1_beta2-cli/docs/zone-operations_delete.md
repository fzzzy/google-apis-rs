Deletes the specified zone-specific operation resource.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/compute* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/compute*.
You can set the scope for this method like this: `autoscaler1-beta2 --scope <scope> zone-operations delete ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - No description provided.
* **&lt;zone&gt;** *(string)*
    - No description provided.
* **&lt;operation&gt;** *(string)*
    - No description provided.
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
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.

* **-p user-ip=string**
    - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
