Marks the provided site&#39;s sample URL as fixed, and removes it from the samples list.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/webmasters* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/webmasters*.
You can set the scope for this method like this: `webmasters3 --scope <scope> urlcrawlerrorssamples mark-as-fixed ...`
# Required Scalar Arguments
* **&lt;site-url&gt;** *(string)*
    - The site&#39;s URL, including protocol. For example: http://www.example.com/
* **&lt;url&gt;** *(string)*
    - The relative path (without the site) of the sample URL. It must be one of the URLs returned by list(). For example, for the URL https://www.example.com/pagename on the site https://www.example.com/, the url value is pagename
* **&lt;category&gt;** *(string)*
    - The crawl error category. For example: authPermissions
* **&lt;platform&gt;** *(string)*
    - The user agent type (platform) that made the request. For example: web
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
