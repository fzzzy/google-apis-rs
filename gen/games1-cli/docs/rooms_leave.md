Leave a room. For internal use by the Games SDK only. Calling this method directly is unsupported.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/games*
* *https://www.googleapis.com/auth/plus.login*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/games*.
You can set the scope for this method like this: `games1 --scope <scope> rooms leave ...`
# Required Scalar Argument
* **&lt;room-id&gt;** *(string)*
    - The ID of the room.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
RoomLeaveRequest:
  kind: string
  leave-diagnostics:
    android-network-subtype: integer
    android-network-type: integer
    ios-network-type: integer
    kind: string
    network-operator-code: string
    network-operator-name: string
    sockets-used: boolean
  reason: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    kind=eirmod`
    - Uniquely identifies the type of this resource. Value is always the fixed string games#roomLeaveRequest.
* `leave-diagnostics    android-network-subtype=68`
    - Android network subtype. http://developer.android.com/reference/android/net/NetworkInfo.html#getSubtype()
* `android-network-type=64`
    - Android network type. http://developer.android.com/reference/android/net/NetworkInfo.html#getType()
* `ios-network-type=19`
    - iOS network type as defined in Reachability.h.
* `kind=accusam`
    - Uniquely identifies the type of this resource. Value is always the fixed string games#roomLeaveDiagnostics.
* `network-operator-code=lorem`
    - The MCC+MNC code for the client&#39;s network connection. On Android: http://developer.android.com/reference/android/telephony/TelephonyManager.html#getNetworkOperator() On iOS, see: https://developer.apple.com/library/ios/documentation/NetworkingInternet/Reference/CTCarrier/Reference/Reference.html
* `network-operator-name=sea`
    - The name of the carrier of the client&#39;s network connection. On Android: http://developer.android.com/reference/android/telephony/TelephonyManager.html#getNetworkOperatorName() On iOS: https://developer.apple.com/library/ios/documentation/NetworkingInternet/Reference/CTCarrier/Reference/Reference.html#//apple_ref/occ/instp/CTCarrier/carrierName
* `sockets-used=true`
    - Whether or not sockets were used.

* `..    reason=duo`
    - Reason for leaving the match.
        Possible values are:  
        - &#34;PLAYER_LEFT&#34; - The player chose to leave the room.. 
        - &#34;GAME_LEFT&#34; - The game chose to remove the player from the room. 
        - &#34;REALTIME_ABANDONED&#34; - The player switched to another application and abandoned the room. 
        - &#34;REALTIME_PEER_CONNECTION_FAILURE&#34; - The client was unable to establish a connection to other peer(s). 
        - &#34;REALTIME_SERVER_CONNECTION_FAILURE&#34; - The client was unable to communicate with the server. 
        - &#34;REALTIME_SERVER_ERROR&#34; - The client received an error response when it tried to communicate with the server. 
        - &#34;REALTIME_TIMEOUT&#34; - The client timed out while waiting for a room. 
        - &#34;REALTIME_CLIENT_DISCONNECTING&#34; - The client disconnects without first calling Leave. 
        - &#34;REALTIME_SIGN_OUT&#34; - The user signed out of G+ while in the room. 
        - &#34;REALTIME_GAME_CRASHED&#34; - The game crashed. 
        - &#34;REALTIME_ROOM_SERVICE_CRASHED&#34; - RoomAndroidService crashed. 
        - &#34;REALTIME_DIFFERENT_CLIENT_ROOM_OPERATION&#34; - Another client is trying to enter a room. 
        - &#34;REALTIME_SAME_CLIENT_ROOM_OPERATION&#34; - The same client is trying to enter a new room.


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
