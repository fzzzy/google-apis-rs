Deletes the APK-specific localized listing for a specified APK and language code.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidpublisher* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidpublisher*.
You can set the scope for this method like this: `androidpublisher2 --scope <scope> edits apklistings-delete ...`
# Required Scalar Arguments
* **&lt;package-name&gt;** *(string)*
    - Unique identifier for the Android app that is being updated; for example, &#34;com.spiffygame&#34;.
* **&lt;edit-id&gt;** *(string)*
    - Unique identifier for this edit.
* **&lt;apk-version-code&gt;** *(integer)*
    - The APK version code whose APK-specific listings should be read or modified.
* **&lt;language&gt;** *(string)*
    - The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass &#34;de-AT&#34;.
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
