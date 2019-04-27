Creates a snapshot from the requested subscription. Snapshots are used in
&lt;a href=&#34;/pubsub/docs/replay-overview&#34;&gt;Seek&lt;/a&gt; operations, which allow
you to manage message acknowledgments in bulk. That is, you can set the
acknowledgment state of messages in an existing subscription to the state
captured by a snapshot.
&lt;br&gt;&lt;br&gt;
&lt;b&gt;BETA:&lt;/b&gt; This feature is part of a beta release. This API might be
changed in backward-incompatible ways and is not recommended for production
use. It is not subject to any SLA or deprecation policy.&lt;br&gt;&lt;br&gt;
If the snapshot already exists, returns `ALREADY_EXISTS`.
If the requested subscription doesn&#39;t exist, returns `NOT_FOUND`.
If the backlog in the subscription is too old -- and the resulting snapshot
would expire in less than 1 hour -- then `FAILED_PRECONDITION` is returned.
See also the `Snapshot.expire_time` field. If the name is not provided in
the request, the server will assign a random
name for this snapshot on the same project as the subscription, conforming
to the [resource name format](https://cloud.google.com/pubsub/docs/overview#names).
The generated
name is populated in the returned Snapshot object. Note that for REST API
requests, you must specify a name in the request.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/pubsub*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `pubsub1 --scope <scope> projects snapshots-create ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Optional user-provided name for this snapshot.
        If the name is not provided in the request, the server will assign a random
        name for this snapshot on the same project as the subscription.
        Note that for REST API requests, you must specify a name.  See the
        &lt;a href=&#34;/pubsub/docs/admin#resource_names&#34;&gt;resource name rules&lt;/a&gt;.
        Format is `projects/{project}/snapshots/{snap}`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CreateSnapshotRequest:
  labels: { string: string }
  subscription: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    labels=key=eirmod`
    - See &lt;a href=&#34;/pubsub/docs/labels&#34;&gt; Creating and managing labels&lt;/a&gt;.
    - the value will be associated with the given `key`
* `subscription=sit`
    - The subscription whose backlog the snapshot retains.
        Specifically, the created snapshot is guaranteed to retain:
         (a) The existing backlog on the subscription. More precisely, this is
             defined as the messages in the subscription&#39;s backlog that are
             unacknowledged upon the successful completion of the
             `CreateSnapshot` request; as well as:
         (b) Any messages published to the subscription&#39;s topic following the
             successful completion of the CreateSnapshot request.
        Format is `projects/{project}/subscriptions/{sub}`.


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
