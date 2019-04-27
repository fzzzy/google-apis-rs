Updates an existing snapshot. Snapshots are used in
&lt;a href=&#34;/pubsub/docs/replay-overview&#34;&gt;Seek&lt;/a&gt; operations, which allow
you to manage message acknowledgments in bulk. That is, you can set the
acknowledgment state of messages in an existing subscription to the state
captured by a snapshot.&lt;br&gt;&lt;br&gt;
&lt;b&gt;BETA:&lt;/b&gt; This feature is part of a beta release. This API might be
changed in backward-incompatible ways and is not recommended for production
use. It is not subject to any SLA or deprecation policy.
Note that certain properties of a snapshot are not modifiable.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/pubsub*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `pubsub1 --scope <scope> projects snapshots-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The name of the snapshot.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
UpdateSnapshotRequest:
  snapshot:
    expire-time: string
    labels: { string: string }
    name: string
    topic: string
  update-mask: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .snapshot    expire-time=stet`
    - The snapshot is guaranteed to exist up until this time.
        A newly-created snapshot expires no later than 7 days from the time of its
        creation. Its exact lifetime is determined at creation by the existing
        backlog in the source subscription. Specifically, the lifetime of the
        snapshot is `7 days - (age of oldest unacked message in the subscription)`.
        For example, consider a subscription whose oldest unacked message is 3 days
        old. If a snapshot is created from this subscription, the snapshot -- which
        will always capture this 3-day-old backlog as long as the snapshot
        exists -- will expire in 4 days. The service will refuse to create a
        snapshot that would expire in less than 1 hour after creation.
* `labels=key=sed`
    - See &lt;a href=&#34;/pubsub/docs/labels&#34;&gt; Creating and managing labels&lt;/a&gt;.
    - the value will be associated with the given `key`
* `name=et`
    - The name of the snapshot.
* `topic=dolores`
    - The name of the topic from which this snapshot is retaining messages.

* `..    update-mask=kasd`
    - Indicates which fields in the provided snapshot to update.
        Must be specified and non-empty.


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
