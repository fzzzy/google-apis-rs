Overwrites the Realtime API data model associated with this file with the provided JSON data model.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.file*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `drive2 --scope <scope> realtime update ...`
# Required Scalar Argument
* **&lt;file-id&gt;** *(string)*
    - The ID of the file that the Realtime API data model is associated with.
# Required Upload Flags

This method supports the upload of data, which *requires* all of the following flags to be set:

* **-u (simple|resumable)**
    - **simple** - Upload media all at once.
    - **resumable** - Upload media in a resumable fashion.
* **-f file**
    - Path to file to upload. It must be seekable.

The following flag *may* be set: 

* **-m mime**
    - the mime type, like 'application/octet-stream', which is the default

# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p base-revision=string**
    - The revision of the model to diff the uploaded model against. If set, the uploaded model is diffed against the provided revision and those differences are merged with any changes made to the model after the provided revision. If not set, the uploaded model replaces the current model on the server.

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
