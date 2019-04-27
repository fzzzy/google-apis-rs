Updates a device.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloudiot*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudiot1 --scope <scope> projects locations-registries-groups-devices-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The resource path name. For example,
        `projects/p1/locations/us-central1/registries/registry0/devices/dev0` or
        `projects/p1/locations/us-central1/registries/registry0/devices/{num_id}`.
        When `name` is populated as a response from the service, it always ends
        in the device numeric ID.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Device:
  blocked: boolean
  config:
    binary-data: string
    cloud-update-time: string
    device-ack-time: string
    version: string
  id: string
  last-config-ack-time: string
  last-config-send-time: string
  last-error-status:
    code: integer
    message: string
  last-error-time: string
  last-event-time: string
  last-heartbeat-time: string
  last-state-time: string
  log-level: string
  metadata: { string: string }
  name: string
  num-id: string
  state:
    binary-data: string
    update-time: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    blocked=true`
    - If a device is blocked, connections or requests from this device will fail.
        Can be used to temporarily prevent the device from connecting if, for
        example, the sensor is generating bad data and needs maintenance.
* `config    binary-data=sanctus`
    - The device configuration data.
* `cloud-update-time=et`
    - [Output only] The time at which this configuration version was updated in
        Cloud IoT Core. This timestamp is set by the server.
* `device-ack-time=amet`
    - [Output only] The time at which Cloud IoT Core received the
        acknowledgment from the device, indicating that the device has received
        this configuration version. If this field is not present, the device has
        not yet acknowledged that it received this version. Note that when
        the config was sent to the device, many config versions may have been
        available in Cloud IoT Core while the device was disconnected, and on
        connection, only the latest version is sent to the device. Some
        versions may never be sent to the device, and therefore are never
        acknowledged. This timestamp is set by Cloud IoT Core.
* `version=et`
    - [Output only] The version of this update. The version number is assigned by
        the server, and is always greater than 0 after device creation. The
        version must be 0 on the `CreateDevice` request if a `config` is
        specified; the response of `CreateDevice` will always have a value of 1.

* `..    id=consetetur`
    - The user-defined device identifier. The device ID must be unique
        within a device registry.
* `last-config-ack-time=ut`
    - [Output only] The last time a cloud-to-device config version acknowledgment
        was received from the device. This field is only for configurations
        sent through MQTT.
* `last-config-send-time=ea`
    - [Output only] The last time a cloud-to-device config version was sent to
        the device.
* `last-error-status    code=21`
    - The status code, which should be an enum value of google.rpc.Code.
* `message=dolor`
    - A developer-facing error message, which should be in English. Any
        user-facing error message should be localized and sent in the
        google.rpc.Status.details field, or localized by the client.

* `..    last-error-time=dolor`
    - [Output only] The time the most recent error occurred, such as a failure to
        publish to Cloud Pub/Sub. This field is the timestamp of
        &#39;last_error_status&#39;.
* `last-event-time=dolor`
    - [Output only] The last time a telemetry event was received. Timestamps are
        periodically collected and written to storage; they may be stale by a few
        minutes.
* `last-heartbeat-time=et`
    - [Output only] The last time an MQTT `PINGREQ` was received. This field
        applies only to devices connecting through MQTT. MQTT clients usually only
        send `PINGREQ` messages if the connection is idle, and no other messages
        have been sent. Timestamps are periodically collected and written to
        storage; they may be stale by a few minutes.
* `last-state-time=consetetur`
    - [Output only] The last time a state event was received. Timestamps are
        periodically collected and written to storage; they may be stale by a few
        minutes.
* `log-level=amet.`
    - **Beta Feature**
        
        The logging verbosity for device activity. If unspecified,
        DeviceRegistry.log_level will be used.
* `metadata=key=voluptua.`
    - The metadata key-value pairs assigned to the device. This metadata is not
        interpreted or indexed by Cloud IoT Core. It can be used to add contextual
        information for the device.
        
        Keys must conform to the regular expression a-zA-Z+ and
        be less than 128 bytes in length.
        
        Values are free-form strings. Each value must be less than or equal to 32
        KB in size.
        
        The total size of all keys and values must be less than 256 KB, and the
        maximum number of key-value pairs is 500.
    - the value will be associated with the given `key`
* `name=lorem`
    - The resource path name. For example,
        `projects/p1/locations/us-central1/registries/registry0/devices/dev0` or
        `projects/p1/locations/us-central1/registries/registry0/devices/{num_id}`.
        When `name` is populated as a response from the service, it always ends
        in the device numeric ID.
* `num-id=gubergren`
    - [Output only] A server-defined unique numeric ID for the device. This is a
        more compact way to identify devices, and it is globally unique.
* `state    binary-data=justo`
    - The device state data.
* `update-time=sit`
    - [Output only] The time at which this state version was updated in Cloud
        IoT Core.



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

* **-p update-mask=string**
    - Only updates the `device` fields indicated by this mask.
        The field mask must not be empty, and it must not contain fields that
        are immutable or only set by the server.
        Mutable top-level fields: `credentials`, `blocked`, and `metadata`

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
