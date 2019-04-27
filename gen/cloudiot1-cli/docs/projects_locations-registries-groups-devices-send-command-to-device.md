Sends a command to the specified device. In order for a device to be able
to receive commands, it must:
1) be connected to Cloud IoT Core using the MQTT protocol, and
2) be subscribed to the group of MQTT topics specified by
   /devices/{device-id}/commands/#. This subscription will receive commands
   at the top-level topic /devices/{device-id}/commands as well as commands
   for subfolders, like /devices/{device-id}/commands/subfolder.
   Note that subscribing to specific subfolders is not supported.
If the command could not be delivered to the device, this method will
return an error; in particular, if the device is not subscribed, this
method will return FAILED_PRECONDITION. Otherwise, this method will
return OK. If the subscription is QoS 1, at least once delivery will be
guaranteed; for QoS 0, no acknowledgment will be expected from the device.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloudiot*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudiot1 --scope <scope> projects locations-registries-groups-devices-send-command-to-device ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The name of the device. For example,
        `projects/p0/locations/us-central1/registries/registry0/devices/device0` or
        `projects/p0/locations/us-central1/registries/registry0/devices/{num_id}`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
SendCommandToDeviceRequest:
  binary-data: string
  subfolder: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    binary-data=vero`
    - The command data to send to the device.
* `subfolder=diam`
    - Optional subfolder for the command. If empty, the command will be delivered
        to the /devices/{device-id}/commands topic, otherwise it will be delivered
        to the /devices/{device-id}/commands/{subfolder} topic. Multi-level
        subfolders are allowed. This field must not have more than 256 characters,
        and must not contain any MQTT wildcards (&#34;+&#34; or &#34;#&#34;) or null characters.


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
