Updates the device policy
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidenterprise* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidenterprise*.
You can set the scope for this method like this: `androidenterprise1 --scope <scope> devices update ...`
# Required Scalar Arguments
* **&lt;enterprise-id&gt;** *(string)*
    - The ID of the enterprise.
* **&lt;user-id&gt;** *(string)*
    - The ID of the user.
* **&lt;device-id&gt;** *(string)*
    - The ID of the device.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Device:
  android-id: string
  kind: string
  management-type: string
  policy:
    auto-update-policy: string
    maintenance-window:
      duration-ms: string
      start-time-after-midnight-ms: string
    product-availability-policy: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    android-id=justo`
    - The Google Play Services Android ID for the device encoded as a lowercase hex string. For example, &#34;123456789abcdef0&#34;.
* `kind=amet.`
    - Identifies what kind of resource this is. Value: the fixed string &#34;androidenterprise#device&#34;.
* `management-type=erat`
    - Identifies the extent to which the device is controlled by a managed Google Play EMM in various deployment configurations.
        
        Possible values include: 
        - &#34;managedDevice&#34;, a device that has the EMM&#39;s device policy controller (DPC) as the device owner. 
        - &#34;managedProfile&#34;, a device that has a profile managed by the DPC (DPC is profile owner) in addition to a separate, personal profile that is unavailable to the DPC. 
        - &#34;containerApp&#34;, no longer used (deprecated). 
        - &#34;unmanagedProfile&#34;, a device that has been allowed (by the domain&#39;s admin, using the Admin Console to enable the privilege) to use managed Google Play, but the profile is itself not owned by a DPC.
* `policy    auto-update-policy=labore`
    - The auto-update policy for apps installed on the device. &#34;choiceToTheUser&#34; allows the device&#39;s user to configure the app update policy. &#34;always&#34; enables auto updates. &#34;never&#34; disables auto updates. &#34;wifiOnly&#34; enables auto updates only when the device is connected to wifi.
* `maintenance-window    duration-ms=sea`
    - Duration of the maintenance window, in milliseconds. The duration must be between 30 minutes and 24 hours (inclusive).
* `start-time-after-midnight-ms=nonumy`
    - Start time of the maintenance window, in milliseconds after midnight on the device. Windows can span midnight.

* `..    product-availability-policy=dolores`
    - The availability granted to the device for the specified products. &#34;all&#34; gives the device access to all products, regardless of approval status. &#34;all&#34; does not enable automatic visibility of &#34;alpha&#34; or &#34;beta&#34; tracks. &#34;whitelist&#34; grants the device access the products specified in productPolicy[]. Only products that are approved or products that were previously approved (products with revoked approval) by the enterprise can be whitelisted. If no value is provided, the availability set at the user level is applied by default.



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
    - Mask that identifies which fields to update. If not set, all modifiable fields will be modified.
        
        When set in a query parameter, this field should be specified as updateMask=&lt;field1&gt;,&lt;field2&gt;,...

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
