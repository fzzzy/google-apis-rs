Gets a revision&#39;s metadata or content by ID.
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
You can set the scope for this method like this: `drive3 --scope <scope> revisions get ...`
# Required Scalar Arguments
* **&lt;file-id&gt;** *(string)*
    - The ID of the file.
* **&lt;revision-id&gt;** *(string)*
    - The ID of the revision.

# Optional Output Flags

The method's return value a JSON encoded structure, which will be written to standard output by default.

As this method supports **media download**, you may specify the `-p alt=media` flag to set the output to be an octet stream of the underlying media. In that case, you will not receive JSON output anymore.

* **-o out**
    - *out* specifies the *destination* to which to write the server's result to.
      It will either be a JSON-encoded structure, or the media file you are downloading.
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p acknowledge-abuse=boolean**
    - Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.

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