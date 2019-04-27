Sets my reading position information for a volume.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/books* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/books*.
You can set the scope for this method like this: `books1 --scope <scope> mylibrary readingpositions-set-position ...`
# Required Scalar Arguments
* **&lt;volume-id&gt;** *(string)*
    - ID of volume for which to update the reading position.
* **&lt;timestamp&gt;** *(string)*
    - RFC 3339 UTC format timestamp associated with this reading position.
* **&lt;position&gt;** *(string)*
    - Position string for the new volume reading position.
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p content-version=string**
    - Volume content version for which this reading position applies.

* **-p device-cookie=string**
    - Random persistent device cookie optional on set position.

* **-p action=string**
    - Action that caused this reading position to be set.

* **-p source=string**
    - String to identify the originator of this request.

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
