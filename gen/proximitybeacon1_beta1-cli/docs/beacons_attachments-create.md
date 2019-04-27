Associates the given data with the specified beacon. Attachment data must
contain two parts:
&lt;ul&gt;
&lt;li&gt;A namespaced type.&lt;/li&gt;
&lt;li&gt;The actual attachment data itself.&lt;/li&gt;
&lt;/ul&gt;
The namespaced type consists of two parts, the namespace and the type.
The namespace must be one of the values returned by the `namespaces`
endpoint, while the type can be a string of any characters except for the
forward slash (`/`) up to 100 characters in length.

Attachment data can be up to 1024 bytes long.

Authenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)
from a signed-in user with **Is owner** or **Can edit** permissions in the
Google Developers Console project.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/userlocation.beacon.registry* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/userlocation.beacon.registry*.
You can set the scope for this method like this: `proximitybeacon1-beta1 --scope <scope> beacons attachments-create ...`
# Required Scalar Argument
* **&lt;beacon-name&gt;** *(string)*
    - Beacon on which the attachment should be created. A beacon name has the
        format &#34;beacons/N!beaconId&#34; where the beaconId is the base16 ID broadcast
        by the beacon and N is a code for the beacon&#39;s type. Possible values are
        `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
        for AltBeacon. For Eddystone-EID beacons, you may use either the
        current EID or the beacon&#39;s &#34;stable&#34; UID.
        Required.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
BeaconAttachment:
  attachment-name: string
  creation-time-ms: string
  data: string
  max-distance-meters: number
  namespaced-type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    attachment-name=sit`
    - Resource name of this attachment. Attachment names have the format:
        &lt;code&gt;beacons/&lt;var&gt;beacon_id&lt;/var&gt;/attachments/&lt;var&gt;attachment_id&lt;/var&gt;&lt;/code&gt;.
        Leave this empty on creation.
* `creation-time-ms=stet`
    - The UTC time when this attachment was created, in milliseconds since the
        UNIX epoch.
* `data=sed`
    - An opaque data container for client-provided data. Must be
        [base64](http://tools.ietf.org/html/rfc4648#section-4) encoded in HTTP
        requests, and will be so encoded (with padding) in responses.
        Required.
* `max-distance-meters=0.165687283689`
    - The distance away from the beacon at which this attachment should be
        delivered to a mobile app.
        
        Setting this to a value greater than zero indicates that the app should
        behave as if the beacon is &#34;seen&#34; when the mobile device is less than this
        distance away from the beacon.
        
        Different attachments on the same beacon can have different max distances.
        
        Note that even though this value is expressed with fractional meter
        precision, real-world behavior is likley to be much less precise than one
        meter, due to the nature of current Bluetooth radio technology.
        
        Optional. When not set or zero, the attachment should be delivered at the
        beacon&#39;s outer limit of detection.
        
        Negative values are invalid and return an error.
* `namespaced-type=dolores`
    - Specifies what kind of attachment this is. Tells a client how to
        interpret the `data` field. Format is &lt;var&gt;namespace/type&lt;/var&gt;. Namespace
        provides type separation between clients. Type describes the type of
        `data`, for use by the client when parsing the `data` field.
        Required.


### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.


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
    - The project id of the project the attachment will belong to. If
        the project id is not specified then the project making the request
        is used.
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
