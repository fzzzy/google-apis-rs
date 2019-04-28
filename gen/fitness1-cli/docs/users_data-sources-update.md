Updates the specified data source. The dataStreamId, dataType, type, dataStreamName, and device properties with the exception of version, cannot be modified.

Data sources are identified by their dataStreamId.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/fitness.activity.write*
* *https://www.googleapis.com/auth/fitness.blood_glucose.write*
* *https://www.googleapis.com/auth/fitness.blood_pressure.write*
* *https://www.googleapis.com/auth/fitness.body.write*
* *https://www.googleapis.com/auth/fitness.body_temperature.write*
* *https://www.googleapis.com/auth/fitness.location.write*
* *https://www.googleapis.com/auth/fitness.nutrition.write*
* *https://www.googleapis.com/auth/fitness.oxygen_saturation.write*
* *https://www.googleapis.com/auth/fitness.reproductive_health.write*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/fitness.activity.write*.
You can set the scope for this method like this: `fitness1 --scope <scope> users data-sources-update ...`
# Required Scalar Arguments
* **&lt;user-id&gt;** *(string)*
    - Update the data source for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
* **&lt;data-source-id&gt;** *(string)*
    - The data stream ID of the data source to update.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
DataSource:
  application:
    details-url: string
    name: string
    package-name: string
    version: string
  data-quality-standard: [string]
  data-stream-id: string
  data-stream-name: string
  data-type:
    name: string
  device:
    manufacturer: string
    model: string
    type: string
    uid: string
    version: string
  name: string
  type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .application    details-url=erat`
    - An optional URI that can be used to link back to the application.
* `name=sadipscing`
    - The name of this application. This is required for REST clients, but we do not enforce uniqueness of this name. It is provided as a matter of convenience for other developers who would like to identify which REST created an Application or Data Source.
* `package-name=dolor`
    - Package name for this application. This is used as a unique identifier when created by Android applications, but cannot be specified by REST clients. REST clients will have their developer project number reflected into the Data Source data stream IDs, instead of the packageName.
* `version=eirmod`
    - Version of the application. You should update this field whenever the application changes in a way that affects the computation of the data.

* `..    data-quality-standard=elitr`
    - 
    - Each invocation of this argument appends the given value to the array.
* `data-stream-id=amet`
    - A unique identifier for the data stream produced by this data source. The identifier includes:
        
         
        - The physical device&#39;s manufacturer, model, and serial number (UID). 
        - The application&#39;s package name or name. Package name is used when the data source was created by an Android application. The developer project number is used when the data source was created by a REST client. 
        - The data source&#39;s type. 
        - The data source&#39;s stream name.  Note that not all attributes of the data source are used as part of the stream identifier. In particular, the version of the hardware/the application isn&#39;t used. This allows us to preserve the same stream through version updates. This also means that two DataSource objects may represent the same data stream even if they&#39;re not equal.
        
        The exact format of the data stream ID created by an Android application is: type:dataType.name:application.packageName:device.manufacturer:device.model:device.uid:dataStreamName 
        
        The exact format of the data stream ID created by a REST client is: type:dataType.name:developer project number:device.manufacturer:device.model:device.uid:dataStreamName 
        
        When any of the optional fields that comprise of the data stream ID are blank, they will be omitted from the data stream ID. The minimum viable data stream ID would be: type:dataType.name:developer project number
        
        Finally, the developer project number is obfuscated when read by any REST or Android client that did not create the data source. Only the data source creator will see the developer project number in clear and normal form.
* `data-stream-name=no`
    - The stream name uniquely identifies this particular data source among other data sources of the same type from the same underlying producer. Setting the stream name is optional, but should be done whenever an application exposes two streams for the same data type, or when a device has two equivalent sensors.
* `data-type    name=labore`
    - Each data type has a unique, namespaced, name. All data types in the com.google namespace are shared as part of the platform.

* `..device    manufacturer=eirmod`
    - Manufacturer of the product/hardware.
* `model=dolore`
    - End-user visible model name for the device.
* `type=invidunt`
    - A constant representing the type of the device.
* `uid=aliquyam`
    - The serial number or other unique ID for the hardware. This field is obfuscated when read by any REST or Android client that did not create the data source. Only the data source creator will see the uid field in clear and normal form.
* `version=accusam`
    - Version string for the device hardware/software.

* `..    name=lorem`
    - An end-user visible name for this data source.
* `type=sea`
    - A constant describing the type of this data source. Indicates whether this data source produces raw or derived data.


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