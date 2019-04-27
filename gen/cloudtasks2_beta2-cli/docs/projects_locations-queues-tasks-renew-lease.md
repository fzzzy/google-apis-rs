Renew the current lease of a pull task.

The worker can use this method to extend the lease by a new
duration, starting from now. The new task lease will be
returned in the task&#39;s schedule_time.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudtasks2-beta2 --scope <scope> projects locations-queues-tasks-renew-lease ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Required.
        
        The task name. For example:
        `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
RenewLeaseRequest:
  lease-duration: string
  response-view: string
  schedule-time: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    lease-duration=lorem`
    - Required.
        
        The desired new lease duration, starting from now.
        
        
        The maximum lease duration is 1 week.
        `lease_duration` will be truncated to the nearest second.
* `response-view=gubergren`
    - The response_view specifies which subset of the Task will be
        returned.
        
        By default response_view is BASIC; not all
        information is retrieved by default because some data, such as
        payloads, might be desirable to return only when needed because
        of its large size or because of the sensitivity of data that it
        contains.
        
        Authorization for FULL requires
        `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/)
        permission on the Task resource.
* `schedule-time=justo`
    - Required.
        
        The task&#39;s current schedule time, available in the
        schedule_time returned by
        LeaseTasks response or
        RenewLease response. This restriction is
        to ensure that your worker currently holds the lease.


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
