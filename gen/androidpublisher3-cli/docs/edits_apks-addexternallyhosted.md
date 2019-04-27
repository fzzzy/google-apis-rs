Creates a new APK without uploading the APK itself to Google Play, instead hosting the APK at a specified URL. This function is only available to enterprises using Google Play for Work whose application is configured to restrict distribution to the enterprise domain.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidpublisher* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidpublisher*.
You can set the scope for this method like this: `androidpublisher3 --scope <scope> edits apks-addexternallyhosted ...`
# Required Scalar Arguments
* **&lt;package-name&gt;** *(string)*
    - Unique identifier for the Android app that is being updated; for example, &#34;com.spiffygame&#34;.
* **&lt;edit-id&gt;** *(string)*
    - Unique identifier for this edit.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ApksAddExternallyHostedRequest:
  externally-hosted-apk:
    application-label: string
    certificate-base64s: [string]
    externally-hosted-url: string
    file-sha1-base64: string
    file-sha256-base64: string
    file-size: string
    icon-base64: string
    maximum-sdk: integer
    minimum-sdk: integer
    native-codes: [string]
    package-name: string
    uses-features: [string]
    version-code: integer
    version-name: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .externally-hosted-apk    application-label=eirmod`
    - The application label.
* `certificate-base64s=sit`
    - A certificate (or array of certificates if a certificate-chain is used) used to signed this APK, represented as a base64 encoded byte array.
    - Each invocation of this argument appends the given value to the array.
* `externally-hosted-url=stet`
    - The URL at which the APK is hosted. This must be an https URL.
* `file-sha1-base64=sed`
    - The SHA1 checksum of this APK, represented as a base64 encoded byte array.
* `file-sha256-base64=et`
    - The SHA256 checksum of this APK, represented as a base64 encoded byte array.
* `file-size=dolores`
    - The file size in bytes of this APK.
* `icon-base64=kasd`
    - The icon image from the APK, as a base64 encoded byte array.
* `maximum-sdk=79`
    - The maximum SDK supported by this APK (optional).
* `minimum-sdk=93`
    - The minimum SDK targeted by this APK.
* `native-codes=justo`
    - The native code environments supported by this APK (optional).
    - Each invocation of this argument appends the given value to the array.
* `package-name=amet.`
    - The package name.
* `uses-features=erat`
    - The features required by this APK (optional).
    - Each invocation of this argument appends the given value to the array.
* `version-code=66`
    - The version code of this APK.
* `version-name=sea`
    - The version name of this APK.



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
