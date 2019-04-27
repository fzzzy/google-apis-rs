Join a room. For internal use by the Games SDK only. Calling this method directly is unsupported.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/games*
* *https://www.googleapis.com/auth/plus.login*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/games*.
You can set the scope for this method like this: `games1 --scope <scope> rooms join ...`
# Required Scalar Argument
* **&lt;room-id&gt;** *(string)*
    - The ID of the room.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
RoomJoinRequest:
  capabilities: [string]
  client-address:
    kind: string
    xmpp-address: string
  kind: string
  network-diagnostics:
    android-network-subtype: integer
    android-network-type: integer
    ios-network-type: integer
    kind: string
    network-operator-code: string
    network-operator-name: string
    registration-latency-millis: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    capabilities=sea`
    - The capabilities that this client supports for realtime communication.
    - Each invocation of this argument appends the given value to the array.
* `client-address    kind=lorem`
    - Uniquely identifies the type of this resource. Value is always the fixed string games#roomClientAddress.
* `xmpp-address=eos`
    - The XMPP address of the client on the Google Games XMPP network.

* `..    kind=erat`
    - Uniquely identifies the type of this resource. Value is always the fixed string games#roomJoinRequest.
* `network-diagnostics    android-network-subtype=6`
    - The Android network subtype.
* `android-network-type=53`
    - The Android network type.
* `ios-network-type=62`
    - iOS network type as defined in Reachability.h.
* `kind=elitr`
    - Uniquely identifies the type of this resource. Value is always the fixed string games#networkDiagnostics.
* `network-operator-code=amet`
    - The MCC+MNC code for the client&#39;s network connection. On Android: http://developer.android.com/reference/android/telephony/TelephonyManager.html#getNetworkOperator() On iOS, see: https://developer.apple.com/library/ios/documentation/NetworkingInternet/Reference/CTCarrier/Reference/Reference.html
* `network-operator-name=no`
    - The name of the carrier of the client&#39;s network connection. On Android: http://developer.android.com/reference/android/telephony/TelephonyManager.html#getNetworkOperatorName() On iOS: https://developer.apple.com/library/ios/documentation/NetworkingInternet/Reference/CTCarrier/Reference/Reference.html#//apple_ref/occ/instp/CTCarrier/carrierName
* `registration-latency-millis=65`
    - The amount of time in milliseconds it took for the client to establish a connection with the XMPP server.



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

* **-p language=string**
    - The preferred language to use for strings returned by this method.

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
