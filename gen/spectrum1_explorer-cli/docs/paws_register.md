The Google Spectrum Database implements registration in the getSpectrum method. As such this always returns an UNIMPLEMENTED error.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
PawsRegisterRequest:
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
  device-owner:
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
  location:
    confidence: integer
    point:
      center:
        latitude: number
        longitude: number
      orientation: number
      semi-major-axis: number
      semi-minor-axis: number
  type: string
  version: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .antenna    height=0.236711616706`
    - The antenna height in meters. Whether the antenna height is required depends on the device type and the regulatory domain. Note that the height may be negative.
* `height-type=duo`
    - If the height is required, then the height type (AGL for above ground level or AMSL for above mean sea level) is also required. The default is AGL.
* `height-uncertainty=0.0858312730549`
    - The height uncertainty in meters. Whether this is required depends on the regulatory domain.

* `..device-desc    etsi-en-device-category=aliquyam`
    - Specifies the ETSI white space device category. Valid values are the strings master and slave. This field is case-insensitive. Consult the ETSI documentation for details about the device types.
* `etsi-en-device-emissions-class=ea`
    - Specifies the ETSI white space device emissions class. The values are represented by numeric strings, such as 1, 2, etc. Consult the ETSI documentation for details about the device types.
* `etsi-en-device-type=ea`
    - Specifies the ETSI white space device type. Valid values are single-letter strings, such as A, B, etc. Consult the ETSI documentation for details about the device types.
* `etsi-en-technology-id=et`
    - Specifies the ETSI white space device technology identifier. The string value must not exceed 64 characters in length. Consult the ETSI documentation for details about the device types.
* `fcc-id=dolor`
    - Specifies the device&#39;s FCC certification identifier. The value is an identifier string whose length should not exceed 32 characters. Note that, in practice, a valid FCC ID may be limited to 19 characters.
* `fcc-tvbd-device-type=diam`
    - Specifies the TV Band White Space device type, as defined by the FCC. Valid values are FIXED, MODE_1, MODE_2.
* `manufacturer-id=kasd`
    - The manufacturer&#39;s ID may be required by the regulatory domain. This should represent the name of the device manufacturer, should be consistent across all devices from the same manufacturer, and should be distinct from that of other manufacturers. The string value must not exceed 64 characters in length.
* `model-id=invidunt`
    - The device&#39;s model ID may be required by the regulatory domain. The string value must not exceed 64 characters in length.
* `ruleset-ids=rebum.`
    - The list of identifiers for rulesets supported by the device. A database may require that the device provide this list before servicing the device requests. If the database does not support any of the rulesets specified in the list, the database may refuse to service the device requests. If present, the list must contain at least one entry.
        
        For information about the valid requests, see section 9.2 of the PAWS specification. Currently, FccTvBandWhiteSpace-2010 is the only supported ruleset.
    - Each invocation of this argument appends the given value to the array.
* `serial-number=lorem`
    - The manufacturer&#39;s device serial number; required by the applicable regulatory domain. The length of the value must not exceed 64 characters.

* `..device-owner.operator.adr    code=clita`
    - The postal code associated with the address. For example: 94423.
* `country=invidunt`
    - The country name. For example: US.
* `locality=eirmod`
    - The city or local equivalent portion of the address. For example: San Jose.
* `pobox=at`
    - An optional post office box number.
* `region=consetetur`
    - The state or local equivalent portion of the address. For example: CA.
* `street=et`
    - The street number and name. For example: 123 Any St.

* `..email    text=sed`
    - The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.

* `..    fn=sit`
    - The full name of the contact person. For example: John A. Smith.
* `org    text=takimata`
    - The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.

* `..tel    uri=elitr`
    - A nested telephone URI of the form: tel:+1-123-456-7890.


* `...owner.adr    code=nonumy`
    - The postal code associated with the address. For example: 94423.
* `country=rebum.`
    - The country name. For example: US.
* `locality=lorem`
    - The city or local equivalent portion of the address. For example: San Jose.
* `pobox=lorem`
    - An optional post office box number.
* `region=diam`
    - The state or local equivalent portion of the address. For example: CA.
* `street=ut`
    - The street number and name. For example: 123 Any St.

* `..email    text=ut`
    - The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.

* `..    fn=amet.`
    - The full name of the contact person. For example: John A. Smith.
* `org    text=ipsum`
    - The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.

* `..tel    uri=ut`
    - A nested telephone URI of the form: tel:+1-123-456-7890.



* `....location    confidence=98`
    - The location confidence level, as an integer percentage, may be required, depending on the regulatory domain. When the parameter is optional and not provided, its value is assumed to be 95. Valid values range from 0 to 99, since, in practice, 100-percent confidence is not achievable. The confidence value is meaningful only when geolocation refers to a point with uncertainty.
* `point.center    latitude=0.915204993436`
    - A required floating-point number that expresses the latitude in degrees using the WGS84 datum. For details on this encoding, see the National Imagery and Mapping Agency&#39;s Technical Report TR8350.2.
* `longitude=0.649303628005`
    - A required floating-point number that expresses the longitude in degrees using the WGS84 datum. For details on this encoding, see the National Imagery and Mapping Agency&#39;s Technical Report TR8350.2.

* `..    orientation=0.615268420345`
    - A floating-point number that expresses the orientation of the ellipse, representing the rotation, in degrees, of the semi-major axis from North towards the East. For example, when the uncertainty is greatest along the North-South direction, orientation is 0 degrees; conversely, if the uncertainty is greatest along the East-West direction, orientation is 90 degrees. When orientation is not present, the orientation is assumed to be 0.
* `semi-major-axis=0.930485818219`
    - A floating-point number that expresses the location uncertainty along the major axis of the ellipse. May be required by the regulatory domain. When the uncertainty is optional, the default value is 0.
* `semi-minor-axis=0.735189875377`
    - A floating-point number that expresses the location uncertainty along the minor axis of the ellipse. May be required by the regulatory domain. When the uncertainty is optional, the default value is 0.


* `...    type=dolor`
    - The message type (e.g., INIT_REQ, AVAIL_SPECTRUM_REQ, ...).
        
        Required field.
* `version=et`
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
