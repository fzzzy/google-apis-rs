Create a new CryptoKeyVersion in a CryptoKey.

The server will assign the next sequential id. If unset,
state will be set to
ENABLED.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudkms1 --scope <scope> projects locations-key-rings-crypto-keys-crypto-key-versions-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The name of the CryptoKey associated with
        the CryptoKeyVersions.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CryptoKeyVersion:
  algorithm: string
  attestation:
    content: string
    format: string
  create-time: string
  destroy-event-time: string
  destroy-time: string
  generate-time: string
  name: string
  protection-level: string
  state: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    algorithm=et`
    - Output only. The CryptoKeyVersionAlgorithm that this
        CryptoKeyVersion supports.
* `attestation    content=diam`
    - Output only. The attestation data provided by the HSM when the key
        operation was performed.
* `format=ipsum`
    - Output only. The format of the attestation data.

* `..    create-time=lorem`
    - Output only. The time at which this CryptoKeyVersion was created.
* `destroy-event-time=et`
    - Output only. The time this CryptoKeyVersion&#39;s key material was
        destroyed. Only present if state is
        DESTROYED.
* `destroy-time=duo`
    - Output only. The time this CryptoKeyVersion&#39;s key material is scheduled
        for destruction. Only present if state is
        DESTROY_SCHEDULED.
* `generate-time=aliquyam`
    - Output only. The time this CryptoKeyVersion&#39;s key material was
        generated.
* `name=sea`
    - Output only. The resource name for this CryptoKeyVersion in the format
        `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`.
* `protection-level=lorem`
    - Output only. The ProtectionLevel describing how crypto operations are
        performed with this CryptoKeyVersion.
* `state=eos`
    - The current state of the CryptoKeyVersion.


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