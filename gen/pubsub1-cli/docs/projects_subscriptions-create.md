Creates a subscription to a given topic. See the
&lt;a href=&#34;/pubsub/docs/admin#resource_names&#34;&gt; resource name rules&lt;/a&gt;.
If the subscription already exists, returns `ALREADY_EXISTS`.
If the corresponding topic doesn&#39;t exist, returns `NOT_FOUND`.

If the name is not provided in the request, the server will assign a random
name for this subscription on the same project as the topic, conforming
to the
[resource name format](https://cloud.google.com/pubsub/docs/overview#names).
The generated name is populated in the returned Subscription object.
Note that for REST API requests, you must specify a name in the request.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/pubsub*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `pubsub1 --scope <scope> projects subscriptions-create ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The name of the subscription. It must have the format
        `&#34;projects/{project}/subscriptions/{subscription}&#34;`. `{subscription}` must
        start with a letter, and contain only letters (`[A-Za-z]`), numbers
        (`[0-9]`), dashes (`-`), underscores (`_`), periods (`.`), tildes (`~`),
        plus (`+`) or percent signs (`%`). It must be between 3 and 255 characters
        in length, and it must not start with `&#34;goog&#34;`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Subscription:
  ack-deadline-seconds: integer
  labels: { string: string }
  message-retention-duration: string
  name: string
  push-config:
    attributes: { string: string }
    push-endpoint: string
  retain-acked-messages: boolean
  topic: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    ack-deadline-seconds=20`
    - This value is the maximum time after a subscriber receives a message
        before the subscriber should acknowledge the message. After message
        delivery but before the ack deadline expires and before the message is
        acknowledged, it is an outstanding message and will not be delivered
        again during that time (on a best-effort basis).
        
        For pull subscriptions, this value is used as the initial value for the ack
        deadline. To override this value for a given message, call
        `ModifyAckDeadline` with the corresponding `ack_id` if using
        non-streaming pull or send the `ack_id` in a
        `StreamingModifyAckDeadlineRequest` if using streaming pull.
        The minimum custom deadline you can specify is 10 seconds.
        The maximum custom deadline you can specify is 600 seconds (10 minutes).
        If this parameter is 0, a default value of 10 seconds is used.
        
        For push delivery, this value is also used to set the request timeout for
        the call to the push endpoint.
        
        If the subscriber never acknowledges the message, the Pub/Sub
        system will eventually redeliver the message.
* `labels=key=labore`
    - See &lt;a href=&#34;/pubsub/docs/labels&#34;&gt; Creating and managing labels&lt;/a&gt;.
    - the value will be associated with the given `key`
* `message-retention-duration=sea`
    - How long to retain unacknowledged messages in the subscription&#39;s backlog,
        from the moment a message is published.
        If `retain_acked_messages` is true, then this also configures the retention
        of acknowledged messages, and thus configures how far back in time a `Seek`
        can be done. Defaults to 7 days. Cannot be more than 7 days or less than 10
        minutes.&lt;br&gt;&lt;br&gt;
        &lt;b&gt;BETA:&lt;/b&gt; This feature is part of a beta release. This API might be
        changed in backward-incompatible ways and is not recommended for production
        use. It is not subject to any SLA or deprecation policy.
* `name=nonumy`
    - The name of the subscription. It must have the format
        `&#34;projects/{project}/subscriptions/{subscription}&#34;`. `{subscription}` must
        start with a letter, and contain only letters (`[A-Za-z]`), numbers
        (`[0-9]`), dashes (`-`), underscores (`_`), periods (`.`), tildes (`~`),
        plus (`+`) or percent signs (`%`). It must be between 3 and 255 characters
        in length, and it must not start with `&#34;goog&#34;`.
* `push-config    attributes=key=dolores`
    - Endpoint configuration attributes.
        
        Every endpoint has a set of API supported attributes that can be used to
        control different aspects of the message delivery.
        
        The currently supported attribute is `x-goog-version`, which you can
        use to change the format of the pushed message. This attribute
        indicates the version of the data expected by the endpoint. This
        controls the shape of the pushed message (i.e., its fields and metadata).
        The endpoint version is based on the version of the Pub/Sub API.
        
        If not present during the `CreateSubscription` call, it will default to
        the version of the API used to make such call. If not present during a
        `ModifyPushConfig` call, its value will not be changed. `GetSubscription`
        calls will always return a valid version, even if the subscription was
        created without this attribute.
        
        The possible values for this attribute are:
        
        * `v1beta1`: uses the push format defined in the v1beta1 Pub/Sub API.
        * `v1` or `v1beta2`: uses the push format defined in the v1 Pub/Sub API.
    - the value will be associated with the given `key`
* `push-endpoint=gubergren`
    - A URL locating the endpoint to which messages should be pushed.
        For example, a Webhook endpoint might use &#34;https://example.com/push&#34;.

* `..    retain-acked-messages=false`
    - Indicates whether to retain acknowledged messages. If true, then
        messages are not expunged from the subscription&#39;s backlog, even if they are
        acknowledged, until they fall out of the `message_retention_duration`
        window. This must be true if you would like to
        &lt;a href=&#34;/pubsub/docs/replay-overview#seek_to_a_time&#34;&gt;Seek to a timestamp&lt;/a&gt;.
        &lt;br&gt;&lt;br&gt;
        &lt;b&gt;BETA:&lt;/b&gt; This feature is part of a beta release. This API might be
        changed in backward-incompatible ways and is not recommended for production
        use. It is not subject to any SLA or deprecation policy.
* `topic=aliquyam`
    - The name of the topic from which this subscription is receiving messages.
        Format is `projects/{project}/topics/{topic}`.
        The value of this field will be `_deleted-topic_` if the topic has been
        deleted.


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
