The Google Spectrum Database does not support batch requests, so this method always yields an UNIMPLEMENTED error.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
PawsGetSpectrumBatchRequest:
  antenna:
    height: number
    height-type: string
    height-uncertainty: number
  device-desc:
    etsi-en-device-category: string
    etsi-en-device-emissions-class: string
    etsi-en-device-type: string
    etsi-en-technology-id: string
    fcc-id: string
    fcc-tvbd-device-type: string
    manufacturer-id: string
    model-id: string
    ruleset-ids: [string]
    serial-number: string
  master-device-desc:
    etsi-en-device-category: string
    etsi-en-device-emissions-class: string
    etsi-en-device-type: string
    etsi-en-technology-id: string
    fcc-id: string
    fcc-tvbd-device-type: string
    manufacturer-id: string
    model-id: string
    ruleset-ids: [string]
    serial-number: string
  owner:
    operator:
      adr:
        code: string
        country: string
        locality: string
        pobox: string
        region: string
        street: string
      email:
        text: string
      fn: string
      org:
        text: string
      tel:
        uri: string
    owner:
      adr:
        code: string
        country: string
        locality: string
        pobox: string
        region: string
        street: string
      email:
        text: string
      fn: string
      org:
        text: string
      tel:
        uri: string
  request-type: string
  type: string
  version: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .antenna    height=0.610793242854`
    - The antenna height in meters. Whether the antenna height is required depends on the device type and the regulatory domain. Note that the height may be negative.
* `height-type=sanctus`
    - If the height is required, then the height type (AGL for above ground level or AMSL for above mean sea level) is also required. The default is AGL.
* `height-uncertainty=0.790427604532`
    - The height uncertainty in meters. Whether this is required depends on the regulatory domain.

* `..device-desc    etsi-en-device-category=amet`
    - Specifies the ETSI white space device category. Valid values are the strings master and slave. This field is case-insensitive. Consult the ETSI documentation for details about the device types.
* `etsi-en-device-emissions-class=et`
    - Specifies the ETSI white space device emissions class. The values are represented by numeric strings, such as 1, 2, etc. Consult the ETSI documentation for details about the device types.
* `etsi-en-device-type=consetetur`
    - Specifies the ETSI white space device type. Valid values are single-letter strings, such as A, B, etc. Consult the ETSI documentation for details about the device types.
* `etsi-en-technology-id=ut`
    - Specifies the ETSI white space device technology identifier. The string value must not exceed 64 characters in length. Consult the ETSI documentation for details about the device types.
* `fcc-id=ea`
    - Specifies the device&#39;s FCC certification identifier. The value is an identifier string whose length should not exceed 32 characters. Note that, in practice, a valid FCC ID may be limited to 19 characters.
* `fcc-tvbd-device-type=sed`
    - Specifies the TV Band White Space device type, as defined by the FCC. Valid values are FIXED, MODE_1, MODE_2.
* `manufacturer-id=dolor`
    - The manufacturer&#39;s ID may be required by the regulatory domain. This should represent the name of the device manufacturer, should be consistent across all devices from the same manufacturer, and should be distinct from that of other manufacturers. The string value must not exceed 64 characters in length.
* `model-id=dolor`
    - The device&#39;s model ID may be required by the regulatory domain. The string value must not exceed 64 characters in length.
* `ruleset-ids=dolor`
    - The list of identifiers for rulesets supported by the device. A database may require that the device provide this list before servicing the device requests. If the database does not support any of the rulesets specified in the list, the database may refuse to service the device requests. If present, the list must contain at least one entry.
        
        For information about the valid requests, see section 9.2 of the PAWS specification. Currently, FccTvBandWhiteSpace-2010 is the only supported ruleset.
    - Each invocation of this argument appends the given value to the array.
* `serial-number=et`
    - The manufacturer&#39;s device serial number; required by the applicable regulatory domain. The length of the value must not exceed 64 characters.

* `..master-device-desc    etsi-en-device-category=consetetur`
    - Specifies the ETSI white space device category. Valid values are the strings master and slave. This field is case-insensitive. Consult the ETSI documentation for details about the device types.
* `etsi-en-device-emissions-class=amet.`
    - Specifies the ETSI white space device emissions class. The values are represented by numeric strings, such as 1, 2, etc. Consult the ETSI documentation for details about the device types.
* `etsi-en-device-type=voluptua.`
    - Specifies the ETSI white space device type. Valid values are single-letter strings, such as A, B, etc. Consult the ETSI documentation for details about the device types.
* `etsi-en-technology-id=lorem`
    - Specifies the ETSI white space device technology identifier. The string value must not exceed 64 characters in length. Consult the ETSI documentation for details about the device types.
* `fcc-id=gubergren`
    - Specifies the device&#39;s FCC certification identifier. The value is an identifier string whose length should not exceed 32 characters. Note that, in practice, a valid FCC ID may be limited to 19 characters.
* `fcc-tvbd-device-type=justo`
    - Specifies the TV Band White Space device type, as defined by the FCC. Valid values are FIXED, MODE_1, MODE_2.
* `manufacturer-id=sit`
    - The manufacturer&#39;s ID may be required by the regulatory domain. This should represent the name of the device manufacturer, should be consistent across all devices from the same manufacturer, and should be distinct from that of other manufacturers. The string value must not exceed 64 characters in length.
* `model-id=vero`
    - The device&#39;s model ID may be required by the regulatory domain. The string value must not exceed 64 characters in length.
* `ruleset-ids=diam`
    - The list of identifiers for rulesets supported by the device. A database may require that the device provide this list before servicing the device requests. If the database does not support any of the rulesets specified in the list, the database may refuse to service the device requests. If present, the list must contain at least one entry.
        
        For information about the valid requests, see section 9.2 of the PAWS specification. Currently, FccTvBandWhiteSpace-2010 is the only supported ruleset.
    - Each invocation of this argument appends the given value to the array.
* `serial-number=rebum.`
    - The manufacturer&#39;s device serial number; required by the applicable regulatory domain. The length of the value must not exceed 64 characters.

* `..owner.operator.adr    code=consetetur`
    - The postal code associated with the address. For example: 94423.
* `country=sadipscing`
    - The country name. For example: US.
* `locality=vero`
    - The city or local equivalent portion of the address. For example: San Jose.
* `pobox=sadipscing`
    - An optional post office box number.
* `region=invidunt`
    - The state or local equivalent portion of the address. For example: CA.
* `street=consetetur`
    - The street number and name. For example: 123 Any St.

* `..email    text=dolore`
    - The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.

* `..    fn=duo`
    - The full name of the contact person. For example: John A. Smith.
* `org    text=aliquyam`
    - The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.

* `..tel    uri=lorem`
    - A nested telephone URI of the form: tel:+1-123-456-7890.


* `...owner.adr    code=et`
    - The postal code associated with the address. For example: 94423.
* `country=clita`
    - The country name. For example: US.
* `locality=consetetur`
    - The city or local equivalent portion of the address. For example: San Jose.
* `pobox=takimata`
    - An optional post office box number.
* `region=nonumy`
    - The state or local equivalent portion of the address. For example: CA.
* `street=kasd`
    - The street number and name. For example: 123 Any St.

* `..email    text=sanctus`
    - The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.

* `..    fn=takimata`
    - The full name of the contact person. For example: John A. Smith.
* `org    text=at`
    - The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.

* `..tel    uri=labore`
    - A nested telephone URI of the form: tel:+1-123-456-7890.



* `....    request-type=invidunt`
    - The request type parameter is an optional parameter that can be used to modify an available spectrum batch request, but its use depends on applicable regulatory rules. For example, It may be used to request generic slave device parameters without having to specify the device descriptor for a specific device. When the requestType parameter is missing, the request is for a specific device (master or slave), and the device descriptor parameter for the device on whose behalf the batch request is made is required.
* `type=ea`
    - The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
        
        Required field.
* `version=sadipscing`
    - The PAWS version. Must be exactly 1.0.
        
        Required field.


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
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.

* **-p user-ip=string**
    - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
