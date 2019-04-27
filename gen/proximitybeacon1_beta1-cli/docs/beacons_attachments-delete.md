Deletes the specified attachment for the given beacon. Each attachment has
a unique attachment name (`attachmentName`) which is returned when you
fetch the attachment data via this API. You specify this with the delete
request to control which attachment is removed. This operation cannot be
undone.

Authenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)
from a signed-in user with **Is owner** or **Can edit** permissions in the
Google Developers Console project.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/userlocation.beacon.registry* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/userlocation.beacon.registry*.
You can set the scope for this method like this: `proximitybeacon1-beta1 --scope <scope> beacons attachments-delete ...`
# Required Scalar Argument
* **&lt;attachment-name&gt;** *(string)*
    - The attachment name (`attachmentName`) of
        the attachment to remove. For example:
        `beacons/3!893737abc9/attachments/c5e937-af0-494-959-ec49d12738`. For
        Eddystone-EID beacons, the beacon ID portion (`3!893737abc9`) may be the
        beacon&#39;s current EID, or its &#34;stable&#34; Eddystone-UID.
        Required.

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

* **-p project-id=string**
    - The project id of the attachment to delete. If not provided, the project
        that is making the request is used.
        Optional.

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
