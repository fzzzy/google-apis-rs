Cancels a user&#39;s subscription purchase. The subscription remains valid until its expiration time.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidpublisher* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidpublisher*.
You can set the scope for this method like this: `androidpublisher2 --scope <scope> purchases subscriptions-cancel ...`
# Required Scalar Arguments
* **&lt;package-name&gt;** *(string)*
    - The package name of the application for which this subscription was purchased (for example, &#39;com.some.thing&#39;).
* **&lt;subscription-id&gt;** *(string)*
    - The purchased subscription ID (for example, &#39;monthly001&#39;).
* **&lt;token&gt;** *(string)*
    - The token provided to the user&#39;s device when the subscription was purchased.
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
