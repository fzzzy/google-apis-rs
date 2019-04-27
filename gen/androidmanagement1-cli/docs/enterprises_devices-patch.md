Updates a device.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidmanagement* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidmanagement*.
You can set the scope for this method like this: `androidmanagement1 --scope <scope> enterprises devices-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Device:
  api-level: integer
  applied-policy-name: string
  applied-policy-version: string
  applied-state: string
  device-settings:
    adb-enabled: boolean
    development-settings-enabled: boolean
    encryption-status: string
    is-device-secure: boolean
    is-encrypted: boolean
    unknown-sources-enabled: boolean
    verify-apps-enabled: boolean
  disabled-reason:
    default-message: string
    localized-messages: { string: string }
  enrollment-time: string
  enrollment-token-data: string
  enrollment-token-name: string
  hardware-info:
    battery-shutdown-temperatures: [number]
    battery-throttling-temperatures: [number]
    brand: string
    cpu-shutdown-temperatures: [number]
    cpu-throttling-temperatures: [number]
    device-baseband-version: string
    gpu-shutdown-temperatures: [number]
    gpu-throttling-temperatures: [number]
    hardware: string
    manufacturer: string
    model: string
    serial-number: string
    skin-shutdown-temperatures: [number]
    skin-throttling-temperatures: [number]
  last-policy-compliance-report-time: string
  last-policy-sync-time: string
  last-status-report-time: string
  management-mode: string
  memory-info:
    total-internal-storage: string
    total-ram: string
  name: string
  network-info:
    imei: string
    meid: string
    network-operator-name: string
    wifi-mac-address: string
  policy-compliant: boolean
  policy-name: string
  previous-device-names: [string]
  software-info:
    android-build-number: string
    android-build-time: string
    android-device-policy-version-code: integer
    android-device-policy-version-name: string
    android-version: string
    bootloader-version: string
    device-build-signature: string
    device-kernel-version: string
    primary-language-code: string
    security-patch-level: string
  state: string
  user:
    account-identifier: string
  user-name: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    api-level=82`
    - The API level of the Android platform version running on the device.
* `applied-policy-name=gubergren`
    - The name of the policy currently applied to the device.
* `applied-policy-version=sadipscing`
    - The version of the policy currently applied to the device.
* `applied-state=aliquyam`
    - The state currently applied to the device.
* `device-settings    adb-enabled=false`
    - Whether ADB (https://developer.android.com/studio/command-line/adb.html) is enabled on the device.
* `development-settings-enabled=false`
    - Whether developer mode is enabled on the device.
* `encryption-status=justo`
    - Encryption status from DevicePolicyManager.
* `is-device-secure=true`
    - Whether the device is secured with PIN/password.
* `is-encrypted=true`
    - Whether the storage encryption is enabled.
* `unknown-sources-enabled=true`
    - Whether installing apps from unknown sources is enabled.
* `verify-apps-enabled=true`
    - Whether Verify Apps (Google Play Protect (https://support.google.com/googleplay/answer/2812853)) is enabled on the device.

* `..disabled-reason    default-message=ipsum`
    - The default message displayed if no localized message is specified or the user&#39;s locale doesn&#39;t match with any of the localized messages. A default message must be provided if any localized messages are provided.
* `localized-messages=key=lorem`
    - A map containing &lt;locale, message&gt; pairs, where locale is a well-formed BCP 47 language (https://www.w3.org/International/articles/language-tags/) code, such as en-US, es-ES, or fr.
    - the value will be associated with the given `key`

* `..    enrollment-time=et`
    - The time of device enrollment.
* `enrollment-token-data=duo`
    - If the device was enrolled with an enrollment token with additional data provided, this field contains that data.
* `enrollment-token-name=aliquyam`
    - If the device was enrolled with an enrollment token, this field contains the name of the token.
* `hardware-info    battery-shutdown-temperatures=0.915099997853`
    - Battery shutdown temperature thresholds in Celsius for each battery on the device.
    - Each invocation of this argument appends the given value to the array.
* `battery-throttling-temperatures=0.45715758654`
    - Battery throttling temperature thresholds in Celsius for each battery on the device.
    - Each invocation of this argument appends the given value to the array.
* `brand=eos`
    - Brand of the device. For example, Google.
* `cpu-shutdown-temperatures=0.200981494395`
    - CPU shutdown temperature thresholds in Celsius for each CPU on the device.
    - Each invocation of this argument appends the given value to the array.
* `cpu-throttling-temperatures=0.0673002722736`
    - CPU throttling temperature thresholds in Celsius for each CPU on the device.
    - Each invocation of this argument appends the given value to the array.
* `device-baseband-version=dolor`
    - Baseband version. For example, MDM9625_104662.22.05.34p.
* `gpu-shutdown-temperatures=0.618673220052`
    - GPU shutdown temperature thresholds in Celsius for each GPU on the device.
    - Each invocation of this argument appends the given value to the array.
* `gpu-throttling-temperatures=0.577297745448`
    - GPU throttling temperature thresholds in Celsius for each GPU on the device.
    - Each invocation of this argument appends the given value to the array.
* `hardware=amet`
    - Name of the hardware. For example, Angler.
* `manufacturer=no`
    - Manufacturer. For example, Motorola.
* `model=labore`
    - The model of the device. For example, Asus Nexus 7.
* `serial-number=eirmod`
    - The device serial number.
* `skin-shutdown-temperatures=0.675598874816`
    - Device skin shutdown temperature thresholds in Celsius.
    - Each invocation of this argument appends the given value to the array.
* `skin-throttling-temperatures=0.634997883415`
    - Device skin throttling temperature thresholds in Celsius.
    - Each invocation of this argument appends the given value to the array.

* `..    last-policy-compliance-report-time=aliquyam`
    - Deprecated.
* `last-policy-sync-time=accusam`
    - The last time the device fetched its policy.
* `last-status-report-time=lorem`
    - The last time the device sent a status report.
* `management-mode=sea`
    - The type of management mode Android Device Policy takes on the device. This influences which policy settings are supported.
* `memory-info    total-internal-storage=et`
    - Total internal storage on device in bytes.
* `total-ram=duo`
    - Total RAM on device in bytes.

* `..    name=et`
    - The name of the device in the form enterprises/{enterpriseId}/devices/{deviceId}.
* `network-info    imei=eirmod`
    - IMEI number of the GSM device. For example, A1000031212.
* `meid=sanctus`
    - MEID number of the CDMA device. For example, A00000292788E1.
* `network-operator-name=et`
    - Alphabetic name of current registered operator. For example, Vodafone.
* `wifi-mac-address=amet`
    - Wi-Fi MAC address of the device. For example, 7c:11:11:11:11:11.

* `..    policy-compliant=true`
    - Whether the device is compliant with its policy.
* `policy-name=consetetur`
    - The name of the policy applied to the device, in the form enterprises/{enterpriseId}/policies/{policyId}. If not specified, the policy_name for the device&#39;s user is applied. This field can be modified by a patch request. You can specify only the policyId when calling enterprises.devices.patch, as long as the policyId doesn’t contain any slashes. The rest of the policy name is inferred.
* `previous-device-names=ut`
    - If the same physical device has been enrolled multiple times, this field contains its previous device names. The serial number is used as the unique identifier to determine if the same physical device has enrolled previously. The names are in chronological order.
    - Each invocation of this argument appends the given value to the array.
* `software-info    android-build-number=ea`
    - Android build ID string meant for displaying to the user. For example, shamu-userdebug 6.0.1 MOB30I 2756745 dev-keys.
* `android-build-time=sed`
    - Build time.
* `android-device-policy-version-code=98`
    - The Android Device Policy app version code.
* `android-device-policy-version-name=dolor`
    - The Android Device Policy app version as displayed to the user.
* `android-version=dolor`
    - The user-visible Android version string. For example, 6.0.1.
* `bootloader-version=et`
    - The system bootloader version number, e.g. 0.6.7.
* `device-build-signature=consetetur`
    - SHA-256 hash of android.content.pm.Signature (https://developer.android.com/reference/android/content/pm/Signature.html) associated with the system package, which can be used to verify that the system build hasn&#39;t been modified.
* `device-kernel-version=amet.`
    - Kernel version, for example, 2.6.32.9-g103d848.
* `primary-language-code=voluptua.`
    - An IETF BCP 47 language code for the primary locale on the device.
* `security-patch-level=lorem`
    - Security patch level, e.g. 2016-05-01.

* `..    state=gubergren`
    - The state to be applied to the device. This field can be modified by a patch request. Note that when calling enterprises.devices.patch, ACTIVE and DISABLED are the only allowable values. To enter the device into a DELETED state, call enterprises.devices.delete.
* `user    account-identifier=justo`
    - A unique identifier you create for this user, such as user342 or asset#44418. This field must be set when the user is created and can&#39;t be updated. This field must not contain personally identifiable information (PII). This identifier must be 1024 characters or less; otherwise, the update policy request will fail.

* `..    user-name=sit`
    - The resource name of the user that owns this device in the form enterprises/{enterpriseId}/users/{userId}.


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
    - The field mask indicating the fields to update. If not set, all modifiable fields will be modified.

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
