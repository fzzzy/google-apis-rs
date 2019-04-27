Update Chrome OS Device
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/admin.directory.device.chromeos* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.directory.device.chromeos*.
You can set the scope for this method like this: `admin1-directory --scope <scope> chromeosdevices update ...`
# Required Scalar Arguments
* **&lt;customer-id&gt;** *(string)*
    - Immutable ID of the G Suite account
* **&lt;device-id&gt;** *(string)*
    - Immutable ID of Chrome OS Device
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ChromeOsDevice:
  annotated-asset-id: string
  annotated-location: string
  annotated-user: string
  boot-mode: string
  device-id: string
  etag: string
  ethernet-mac-address: string
  firmware-version: string
  kind: string
  last-enrollment-time: string
  last-sync: string
  mac-address: string
  meid: string
  model: string
  notes: string
  order-number: string
  org-unit-path: string
  os-version: string
  platform-version: string
  serial-number: string
  status: string
  support-end-date: string
  system-ram-total: string
  tpm-version-info:
    family: string
    firmware-version: string
    manufacturer: string
    spec-level: string
    tpm-model: string
    vendor-specific: string
  will-auto-renew: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    annotated-asset-id=dolore`
    - AssetId specified during enrollment or through later annotation
* `annotated-location=invidunt`
    - Address or location of the device as noted by the administrator
* `annotated-user=aliquyam`
    - User of the device
* `boot-mode=accusam`
    - Chromebook boot mode (Read-only)
* `device-id=lorem`
    - Unique identifier of Chrome OS Device (Read-only)
* `etag=sea`
    - ETag of the resource.
* `ethernet-mac-address=et`
    - Chromebook Mac Address on ethernet network interface (Read-only)
* `firmware-version=duo`
    - Chromebook firmware version (Read-only)
* `kind=et`
    - Kind of resource this is.
* `last-enrollment-time=eirmod`
    - Date and time the device was last enrolled (Read-only)
* `last-sync=sanctus`
    - Date and time the device was last synchronized with the policy settings in the G Suite administrator control panel (Read-only)
* `mac-address=et`
    - Chromebook Mac Address on wifi network interface (Read-only)
* `meid=amet`
    - Mobile Equipment identifier for the 3G mobile card in the Chromebook (Read-only)
* `model=et`
    - Chromebook Model (Read-only)
* `notes=consetetur`
    - Notes added by the administrator
* `order-number=ut`
    - Chromebook order number (Read-only)
* `org-unit-path=ea`
    - OrgUnit of the device
* `os-version=sed`
    - Chromebook Os Version (Read-only)
* `platform-version=dolor`
    - Chromebook platform version (Read-only)
* `serial-number=dolor`
    - Chromebook serial number (Read-only)
* `status=dolor`
    - status of the device (Read-only)
* `support-end-date=et`
    - Final date the device will be supported (Read-only)
* `system-ram-total=consetetur`
    - Total RAM on the device [in bytes] (Read-only)
* `tpm-version-info    family=amet.`
    - TPM family.
* `firmware-version=voluptua.`
    - TPM firmware version.
* `manufacturer=lorem`
    - TPM manufacturer code.
* `spec-level=gubergren`
    - TPM specification level.
* `tpm-model=justo`
    - TPM model number.
* `vendor-specific=sit`
    - Vendor-specific information such as Vendor ID.

* `..    will-auto-renew=true`
    - Will Chromebook auto renew after support end date (Read-only)


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

* **-p projection=string**
    - Restrict information returned to a set of selected fields.

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
