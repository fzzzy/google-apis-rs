Deletes all images for the specified language and image type.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidpublisher* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidpublisher*.
You can set the scope for this method like this: `androidpublisher3 --scope <scope> edits images-deleteall ...`
# Required Scalar Arguments
* **&lt;package-name&gt;** *(string)*
    - Unique identifier for the Android app that is being updated; for example, &#34;com.spiffygame&#34;.
* **&lt;edit-id&gt;** *(string)*
    - Unique identifier for this edit.
* **&lt;language&gt;** *(string)*
    - The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass &#34;de-AT&#34;.
* **&lt;image-type&gt;** *(string)*
    - No description provided.

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