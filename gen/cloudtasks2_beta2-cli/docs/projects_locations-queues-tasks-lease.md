Leases tasks from a pull queue for
lease_duration.

This method is invoked by the worker to obtain a lease. The
worker must acknowledge the task via
AcknowledgeTask after they have
performed the work associated with the task.

The payload is intended to store data that
the worker needs to perform the work associated with the task. To
return the payloads in the response, set
response_view to
FULL.

A maximum of 10 qps of LeaseTasks
requests are allowed per
queue. RESOURCE_EXHAUSTED
is returned when this limit is
exceeded. RESOURCE_EXHAUSTED
is also returned when
max_tasks_dispatched_per_second
is exceeded.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudtasks2-beta2 --scope <scope> projects locations-queues-tasks-lease ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required.
        
        The queue name. For example:
        `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
LeaseTasksRequest:
  filter: string
  lease-duration: string
  max-tasks: integer
  response-view: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    filter=et`
    - `filter` can be used to specify a subset of tasks to lease.
        
        When `filter` is set to `tag=&lt;my-tag&gt;` then the
        response will contain only tasks whose
        tag is equal to `&lt;my-tag&gt;`. `&lt;my-tag&gt;` must be
        less than 500 characters.
        
        When `filter` is set to `tag_function=oldest_tag()`, only tasks which have
        the same tag as the task with the oldest
        schedule_time will be returned.
        
        Grammar Syntax:
        
        * `filter = &#34;tag=&#34; tag | &#34;tag_function=&#34; function`
        
        * `tag = string`
        
        * `function = &#34;oldest_tag()&#34;`
        
        The `oldest_tag()` function returns tasks which have the same tag as the
        oldest task (ordered by schedule time).
        
        SDK compatibility: Although the SDK allows tags to be either
        string or
        [bytes](https://cloud.google.com/appengine/docs/standard/java/javadoc/com/google/appengine/api/taskqueue/TaskOptions.html#tag-byte:A-),
        only UTF-8 encoded tags can be used in Cloud Tasks. Tag which
        aren&#39;t UTF-8 encoded can&#39;t be used in the
        filter and the task&#39;s
        tag will be displayed as empty in Cloud Tasks.
* `lease-duration=consetetur`
    - 
        After the worker has successfully finished the work associated
        with the task, the worker must call via
        AcknowledgeTask before the
        schedule_time. Otherwise the task will be
        returned to a later LeaseTasks call so
        that another worker can retry it.
        
        The maximum lease duration is 1 week.
        `lease_duration` will be truncated to the nearest second.
* `max-tasks=49`
    - The maximum number of tasks to lease.
        
        The system will make a best effort to return as close to as
        `max_tasks` as possible.
        
        The largest that `max_tasks` can be is 1000.
        
        The maximum total size of a lease tasks response is
        32 MB. If the sum of all task sizes requested reaches this limit,
        fewer tasks than requested are returned.
* `response-view=voluptua.`
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
