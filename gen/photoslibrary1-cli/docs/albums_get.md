Returns the album based on the specified `albumId` or `shareToken`.
Exactly one of `albumId` and `shareToken` must be set.
The `albumId` should be the ID of an album owned by the user or a shared
album that the user has joined.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/photoslibrary*
* *https://www.googleapis.com/auth/photoslibrary.readonly*
* *https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata*
* *https://www.googleapis.com/auth/photoslibrary.sharing*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/photoslibrary.readonly*.
You can set the scope for this method like this: `photoslibrary1 --scope <scope> albums get ...`
# Required Scalar Argument
* **&lt;album-id&gt;** *(string)*
    - Identifier of the album to be requested. Must not be set if `shareToken` is
        set.

# Optional Output Flags

The method's return value a JSON encoded structure, which will be written to standard output by default.

* **-o out**
    - *out* specifies the *destination* to which to write the server's result to.
      It will be a JSON-encoded structure.
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p share-token=string**
    - Share token of the album to be request. Must not be set if `albumId` is
        set.

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).