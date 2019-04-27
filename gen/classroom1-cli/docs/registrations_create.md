Creates a `Registration`, causing Classroom to start sending notifications
from the provided `feed` to the destination provided in `cloudPubSubTopic`.

Returns the created `Registration`. Currently, this will be the same as
the argument, but with server-assigned fields such as `expiry_time` and
`id` filled in.

Note that any value specified for the `expiry_time` or `id` fields will be
ignored.

While Classroom may validate the `cloudPubSubTopic` and return errors on a
best effort basis, it is the caller&#39;s responsibility to ensure that it
exists and that Classroom has permission to publish to it.

This method may return the following error codes:

* `PERMISSION_DENIED` if:
    * the authenticated user does not have permission to receive
      notifications from the requested field; or
    * the credential provided does not include the appropriate scope for
      the requested feed.
    * another access error is encountered.
* `INVALID_ARGUMENT` if:
    * no `cloudPubsubTopic` is specified, or the specified
      `cloudPubsubTopic` is not valid; or
    * no `feed` is specified, or the specified `feed` is not valid.
* `NOT_FOUND` if:
    * the specified `feed` cannot be located, or the requesting user does
      not have permission to determine whether or not it exists; or
    * the specified `cloudPubsubTopic` cannot be located, or Classroom has
      not been granted permission to publish to it.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/classroom.push-notifications* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/classroom.push-notifications*.
You can set the scope for this method like this: `classroom1 --scope <scope> registrations create ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Registration:
  cloud-pubsub-topic:
    topic-name: string
  expiry-time: string
  feed:
    course-roster-changes-info:
      course-id: string
    course-work-changes-info:
      course-id: string
    feed-type: string
  registration-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .cloud-pubsub-topic    topic-name=ipsum`
    - The `name` field of a Cloud Pub/Sub
        [Topic](https://cloud.google.com/pubsub/docs/reference/rest/v1/projects.topics#Topic).

* `..    expiry-time=justo`
    - The time until which the `Registration` is effective.
        
        This is a read-only field assigned by the server.
* `feed.course-roster-changes-info    course-id=dolore`
    - The `course_id` of the course to subscribe to roster changes for.

* `..course-work-changes-info    course-id=vero`
    - The `course_id` of the course to subscribe to work changes for.

* `..    feed-type=dolor`
    - The type of feed.

* `..    registration-id=takimata`
    - A server-generated unique identifier for this `Registration`.
        
        Read-only.


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
