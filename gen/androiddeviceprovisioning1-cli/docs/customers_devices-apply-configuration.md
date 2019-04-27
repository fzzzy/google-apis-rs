Applies a Configuration to the device to register the device for zero-touch
enrollment. After applying a configuration to a device, the device
automatically provisions itself on first boot, or next factory reset.
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The customer managing the device. An API resource name in the
        format `customers/[CUSTOMER_ID]`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CustomerApplyConfigurationRequest:
  configuration: string
  device:
    device-id: string
    device-identifier:
      imei: string
      manufacturer: string
      meid: string
      model: string
      serial-number: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    configuration=no`
    - Required. The configuration applied to the device in the format
        `customers/[CUSTOMER_ID]/configurations/[CONFIGURATION_ID]`.
* `device    device-id=justo`
    - The ID of the device.
* `device-identifier    imei=justo`
    - The device’s IMEI number. Validated on input.
* `manufacturer=et`
    - The device manufacturer’s name. Matches the device&#39;s built-in
        value returned from `android.os.Build.MANUFACTURER`. Allowed values are
        listed in
        [manufacturers](/zero-touch/resources/manufacturer-names#manufacturers-names).
* `meid=et`
    - The device’s MEID number.
* `model=diam`
    - The device model&#39;s name. Matches the device&#39;s built-in value returned from
        `android.os.Build.MODEL`. Allowed values are listed in
        [models](/zero-touch/resources/manufacturer-names#model-names).
* `serial-number=ipsum`
    - The manufacturer&#39;s serial number for the device. This value might not be
        unique across different device models.




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
