Requests information about the available spectrum for a device at a location. Requests from a fixed-mode device must include owner information so the device can be registered with the database.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
PawsGetSpectrumRequest:
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
  location:
    confidence: integer
    point:
      center:
        latitude: number
        longitude: number
      orientation: number
      semi-major-axis: number
      semi-minor-axis: number
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

* `-r .antenna    height=0.617752856951`
    - The antenna height in meters. Whether the antenna height is required depends on the device type and the regulatory domain. Note that the height may be negative.
* `height-type=sit`
    - If the height is required, then the height type (AGL for above ground level or AMSL for above mean sea level) is also required. The default is AGL.
* `height-uncertainty=0.365848359249`
    - The height uncertainty in meters. Whether this is required depends on the regulatory domain.

* `..device-desc    etsi-en-device-category=sed`
    - Specifies the ETSI white space device category. Valid values are the strings master and slave. This field is case-insensitive. Consult the ETSI documentation for details about the device types.
* `etsi-en-device-emissions-class=et`
    - Specifies the ETSI white space device emissions class. The values are represented by numeric strings, such as 1, 2, etc. Consult the ETSI documentation for details about the device types.
* `etsi-en-device-type=dolores`
    - Specifies the ETSI white space device type. Valid values are single-letter strings, such as A, B, etc. Consult the ETSI documentation for details about the device types.
* `etsi-en-technology-id=kasd`
    - Specifies the ETSI white space device technology identifier. The string value must not exceed 64 characters in length. Consult the ETSI documentation for details about the device types.
* `fcc-id=accusam`
    - Specifies the device&#39;s FCC certification identifier. The value is an identifier string whose length should not exceed 32 characters. Note that, in practice, a valid FCC ID may be limited to 19 characters.
* `fcc-tvbd-device-type=takimata`
    - Specifies the TV Band White Space device type, as defined by the FCC. Valid values are FIXED, MODE_1, MODE_2.
* `manufacturer-id=justo`
    - The manufacturer&#39;s ID may be required by the regulatory domain. This should represent the name of the device manufacturer, should be consistent across all devices from the same manufacturer, and should be distinct from that of other manufacturers. The string value must not exceed 64 characters in length.
* `model-id=amet.`
    - The device&#39;s model ID may be required by the regulatory domain. The string value must not exceed 64 characters in length.
* `ruleset-ids=erat`
    - The list of identifiers for rulesets supported by the device. A database may require that the device provide this list before servicing the device requests. If the database does not support any of the rulesets specified in the list, the database may refuse to service the device requests. If present, the list must contain at least one entry.
        
        For information about the valid requests, see section 9.2 of the PAWS specification. Currently, FccTvBandWhiteSpace-2010 is the only supported ruleset.
    - Each invocation of this argument appends the given value to the array.
* `serial-number=labore`
    - The manufacturer&#39;s device serial number; required by the applicable regulatory domain. The length of the value must not exceed 64 characters.

* `..location    confidence=92`
    - The location confidence level, as an integer percentage, may be required, depending on the regulatory domain. When the parameter is optional and not provided, its value is assumed to be 95. Valid values range from 0 to 99, since, in practice, 100-percent confidence is not achievable. The confidence value is meaningful only when geolocation refers to a point with uncertainty.
* `point.center    latitude=0.109288397621`
    - A required floating-point number that expresses the latitude in degrees using the WGS84 datum. For details on this encoding, see the National Imagery and Mapping Agency&#39;s Technical Report TR8350.2.
* `longitude=0.820437629783`
    - A required floating-point number that expresses the longitude in degrees using the WGS84 datum. For details on this encoding, see the National Imagery and Mapping Agency&#39;s Technical Report TR8350.2.

* `..    orientation=0.398739054055`
    - A floating-point number that expresses the orientation of the ellipse, representing the rotation, in degrees, of the semi-major axis from North towards the East. For example, when the uncertainty is greatest along the North-South direction, orientation is 0 degrees; conversely, if the uncertainty is greatest along the East-West direction, orientation is 90 degrees. When orientation is not present, the orientation is assumed to be 0.
* `semi-major-axis=0.0653431304201`
    - A floating-point number that expresses the location uncertainty along the major axis of the ellipse. May be required by the regulatory domain. When the uncertainty is optional, the default value is 0.
* `semi-minor-axis=0.699208331616`
    - A floating-point number that expresses the location uncertainty along the minor axis of the ellipse. May be required by the regulatory domain. When the uncertainty is optional, the default value is 0.


* `...master-device-desc    etsi-en-device-category=ea`
    - Specifies the ETSI white space device category. Valid values are the strings master and slave. This field is case-insensitive. Consult the ETSI documentation for details about the device types.
* `etsi-en-device-emissions-class=no`
    - Specifies the ETSI white space device emissions class. The values are represented by numeric strings, such as 1, 2, etc. Consult the ETSI documentation for details about the device types.
* `etsi-en-device-type=justo`
    - Specifies the ETSI white space device type. Valid values are single-letter strings, such as A, B, etc. Consult the ETSI documentation for details about the device types.
* `etsi-en-technology-id=justo`
    - Specifies the ETSI white space device technology identifier. The string value must not exceed 64 characters in length. Consult the ETSI documentation for details about the device types.
* `fcc-id=et`
    - Specifies the device&#39;s FCC certification identifier. The value is an identifier string whose length should not exceed 32 characters. Note that, in practice, a valid FCC ID may be limited to 19 characters.
* `fcc-tvbd-device-type=et`
    - Specifies the TV Band White Space device type, as defined by the FCC. Valid values are FIXED, MODE_1, MODE_2.
* `manufacturer-id=diam`
    - The manufacturer&#39;s ID may be required by the regulatory domain. This should represent the name of the device manufacturer, should be consistent across all devices from the same manufacturer, and should be distinct from that of other manufacturers. The string value must not exceed 64 characters in length.
* `model-id=ipsum`
    - The device&#39;s model ID may be required by the regulatory domain. The string value must not exceed 64 characters in length.
* `ruleset-ids=lorem`
    - The list of identifiers for rulesets supported by the device. A database may require that the device provide this list before servicing the device requests. If the database does not support any of the rulesets specified in the list, the database may refuse to service the device requests. If present, the list must contain at least one entry.
        
        For information about the valid requests, see section 9.2 of the PAWS specification. Currently, FccTvBandWhiteSpace-2010 is the only supported ruleset.
    - Each invocation of this argument appends the given value to the array.
* `serial-number=et`
    - The manufacturer&#39;s device serial number; required by the applicable regulatory domain. The length of the value must not exceed 64 characters.

* `..owner.operator.adr    code=duo`
    - The postal code associated with the address. For example: 94423.
* `country=aliquyam`
    - The country name. For example: US.
* `locality=sea`
    - The city or local equivalent portion of the address. For example: San Jose.
* `pobox=lorem`
    - An optional post office box number.
* `region=eos`
    - The state or local equivalent portion of the address. For example: CA.
* `street=erat`
    - The street number and name. For example: 123 Any St.

* `..email    text=sadipscing`
    - The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.

* `..    fn=dolor`
    - The full name of the contact person. For example: John A. Smith.
* `org    text=eirmod`
    - The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.

* `..tel    uri=elitr`
    - A nested telephone URI of the form: tel:+1-123-456-7890.


* `...owner.adr    code=amet`
    - The postal code associated with the address. For example: 94423.
* `country=no`
    - The country name. For example: US.
* `locality=labore`
    - The city or local equivalent portion of the address. For example: San Jose.
* `pobox=eirmod`
    - An optional post office box number.
* `region=dolore`
    - The state or local equivalent portion of the address. For example: CA.
* `street=invidunt`
    - The street number and name. For example: 123 Any St.

* `..email    text=aliquyam`
    - The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.

* `..    fn=accusam`
    - The full name of the contact person. For example: John A. Smith.
* `org    text=lorem`
    - The text string associated with this item. For example, for an org field: ACME, inc. For an email field: smith@example.com.

* `..tel    uri=sea`
    - A nested telephone URI of the form: tel:+1-123-456-7890.



* `....    request-type=et`
    - The request type parameter is an optional parameter that can be used to modify an available spectrum request, but its use depends on applicable regulatory rules. It may be used, for example, to request generic slave device parameters without having to specify the device descriptor for a specific device. When the requestType parameter is missing, the request is for a specific device (master or slave), and the deviceDesc parameter for the device on whose behalf the request is made is required.
* `type=duo`
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
