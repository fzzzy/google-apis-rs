Gets information about the user, the user&#39;s Drive, and system capabilities.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.appdata*
* *https://www.googleapis.com/auth/drive.file*
* *https://www.googleapis.com/auth/drive.metadata*
* *https://www.googleapis.com/auth/drive.metadata.readonly*
* *https://www.googleapis.com/auth/drive.photos.readonly*
* *https://www.googleapis.com/auth/drive.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive.metadata.readonly*.
You can set the scope for this method like this: `drive3 --scope <scope> about get ...`

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