Modifies the `PushConfig` for a specified subscription.

This may be used to change a push subscription to a pull one (signified by
an empty `PushConfig`) or vice versa, or change the endpoint URL and other
attributes of a push subscription. Messages will accumulate for delivery
continuously through the call regardless of changes to the `PushConfig`.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/pubsub*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `pubsub1-beta2 --scope <scope> projects subscriptions-modify-push-config ...`
# Required Scalar Argument
* **&lt;subscription&gt;** *(string)*
    - The name of the subscription.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ModifyPushConfigRequest:
  push-config:
    attributes: { string: string }
    push-endpoint: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .push-config    attributes=key=justo`
    - Endpoint configuration attributes.
        
        Every endpoint has a set of API supported attributes that can be used to
        control different aspects of the message delivery.
        
        The currently supported attribute is `x-goog-version`, which you can
        use to change the format of the push message. This attribute
        indicates the version of the data expected by the endpoint. This
        controls the shape of the envelope (i.e. its fields and metadata).
        The endpoint version is based on the version of the Pub/Sub
        API.
        
        If not present during the `CreateSubscription` call, it will default to
        the version of the API used to make such call. If not present during a
        `ModifyPushConfig` call, its value will not be changed. `GetSubscription`
        calls will always return a valid version, even if the subscription was
        created without this attribute.
        
        The possible values for this attribute are:
        
        * `v1beta1`: uses the push format defined in the v1beta1 Pub/Sub API.
        * `v1` or `v1beta2`: uses the push format defined in the v1 Pub/Sub API.
    - the value will be associated with the given `key`
* `push-endpoint=amet.`
    - A URL locating the endpoint to which messages should be pushed.
        For example, a Webhook endpoint might use &#34;https://example.com/push&#34;.



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
