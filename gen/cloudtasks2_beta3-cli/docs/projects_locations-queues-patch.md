Updates a queue.

This method creates the queue if it does not exist and updates
the queue if it does exist.

Queues created with this method allow tasks to live for a maximum of 31
days. After a task is 31 days old, the task will be deleted regardless of whether
it was dispatched or not.

WARNING: Using this method may have unintended side effects if you are
using an App Engine `queue.yaml` or `queue.xml` file to manage your queues.
Read
[Overview of Queue Management and queue.yaml](https://cloud.google.com/tasks/docs/queue-yaml)
before using this method.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudtasks2-beta3 --scope <scope> projects locations-queues-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Caller-specified and required in CreateQueue,
        after which it becomes output only.
        
        The queue name.
        
        The queue name must have the following format:
        `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
        
        * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]),
           hyphens (-), colons (:), or periods (.).
           For more information, see
           [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects)
        * `LOCATION_ID` is the canonical ID for the queue&#39;s location.
           The list of available locations can be obtained by calling
           ListLocations.
           For more information, see https://cloud.google.com/about/locations/.
        * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or
          hyphens (-). The maximum length is 100 characters.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Queue:
  app-engine-http-queue:
    app-engine-routing-override:
      host: string
      instance: string
      service: string
      version: string
  name: string
  purge-time: string
  rate-limits:
    max-burst-size: integer
    max-concurrent-dispatches: integer
    max-dispatches-per-second: number
  retry-config:
    max-attempts: integer
    max-backoff: string
    max-doublings: integer
    max-retry-duration: string
    min-backoff: string
  state: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .app-engine-http-queue.app-engine-routing-override    host=dolores`
    - Output only. The host that the task is sent to.
        
        The host is constructed from the domain name of the app associated with
        the queue&#39;s project ID (for example &lt;app-id&gt;.appspot.com), and the
        service, version,
        and instance. Tasks which were created using
        the App Engine SDK might have a custom domain name.
        
        For more information, see
        [How Requests are Routed](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed).
* `instance=gubergren`
    - App instance.
        
        By default, the task is sent to an instance which is available when
        the task is attempted.
        
        Requests can only be sent to a specific instance if
        [manual scaling is used in App Engine Standard](https://cloud.google.com/appengine/docs/python/an-overview-of-app-engine?hl=en_US#scaling_types_and_instance_classes).
        App Engine Flex does not support instances. For more information, see
        [App Engine Standard request routing](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed)
        and [App Engine Flex request routing](https://cloud.google.com/appengine/docs/flexible/python/how-requests-are-routed).
* `service=sadipscing`
    - App service.
        
        By default, the task is sent to the service which is the default
        service when the task is attempted.
        
        For some queues or tasks which were created using the App Engine
        Task Queue API, host is not parsable
        into service,
        version, and
        instance. For example, some tasks
        which were created using the App Engine SDK use a custom domain
        name; custom domains are not parsed by Cloud Tasks. If
        host is not parsable, then
        service,
        version, and
        instance are the empty string.
* `version=aliquyam`
    - App version.
        
        By default, the task is sent to the version which is the default
        version when the task is attempted.
        
        For some queues or tasks which were created using the App Engine
        Task Queue API, host is not parsable
        into service,
        version, and
        instance. For example, some tasks
        which were created using the App Engine SDK use a custom domain
        name; custom domains are not parsed by Cloud Tasks. If
        host is not parsable, then
        service,
        version, and
        instance are the empty string.


* `...    name=ea`
    - Caller-specified and required in CreateQueue,
        after which it becomes output only.
        
        The queue name.
        
        The queue name must have the following format:
        `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
        
        * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]),
           hyphens (-), colons (:), or periods (.).
           For more information, see
           [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects)
        * `LOCATION_ID` is the canonical ID for the queue&#39;s location.
           The list of available locations can be obtained by calling
           ListLocations.
           For more information, see https://cloud.google.com/about/locations/.
        * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or
          hyphens (-). The maximum length is 100 characters.
* `purge-time=no`
    - Output only. The last time this queue was purged.
        
        All tasks that were created before this time
        were purged.
        
        A queue can be purged using PurgeQueue, the
        [App Engine Task Queue SDK, or the Cloud Console](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/deleting-tasks-and-queues#purging_all_tasks_from_a_queue).
        
        Purge time will be truncated to the nearest microsecond. Purge
        time will be unset if the queue has never been purged.
* `rate-limits    max-burst-size=80`
    - Output only. The max burst size.
        
        Max burst size limits how fast tasks in queue are processed when
        many tasks are in the queue and the rate is high. This field
        allows the queue to have a high rate so processing starts shortly
        after a task is enqueued, but still limits resource usage when
        many tasks are enqueued in a short period of time.
        
        The [token bucket](https://wikipedia.org/wiki/Token_Bucket)
        algorithm is used to control the rate of task dispatches. Each
        queue has a token bucket that holds tokens, up to the maximum
        specified by `max_burst_size`. Each time a task is dispatched, a
        token is removed from the bucket. Tasks will be dispatched until
        the queue&#39;s bucket runs out of tokens. The bucket will be
        continuously refilled with new tokens based on
        max_dispatches_per_second.
        
        Cloud Tasks will pick the value of `max_burst_size` based on the
        value of
        max_dispatches_per_second.
        
        For App Engine queues that were created or updated using
        `queue.yaml/xml`, `max_burst_size` is equal to
        [bucket_size](https://cloud.google.com/appengine/docs/standard/python/config/queueref#bucket_size).
        Since `max_burst_size` is output only, if
        UpdateQueue is called on a queue
        created by `queue.yaml/xml`, `max_burst_size` will be reset based
        on the value of
        max_dispatches_per_second,
        regardless of whether
        max_dispatches_per_second
        is updated.
        
* `max-concurrent-dispatches=80`
    - The maximum number of concurrent tasks that Cloud Tasks allows
        to be dispatched for this queue. After this threshold has been
        reached, Cloud Tasks stops dispatching tasks until the number of
        concurrent requests decreases.
        
        If unspecified when the queue is created, Cloud Tasks will pick the
        default.
        
        
        The maximum allowed value is 5,000.
        
        
        This field has the same meaning as
        [max_concurrent_requests in queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#max_concurrent_requests).
* `max-dispatches-per-second=0.663748882844`
    - The maximum rate at which tasks are dispatched from this queue.
        
        If unspecified when the queue is created, Cloud Tasks will pick the
        default.
        
        * For App Engine queues, the maximum allowed value
          is 500.
        
        
        This field has the same meaning as
        [rate in queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#rate).

* `..retry-config    max-attempts=84`
    - Number of attempts per task.
        
        Cloud Tasks will attempt the task `max_attempts` times (that is, if the
        first attempt fails, then there will be `max_attempts - 1` retries). Must
        be &gt;= -1.
        
        If unspecified when the queue is created, Cloud Tasks will pick the
        default.
        
        -1 indicates unlimited attempts.
        
        This field has the same meaning as
        [task_retry_limit in queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters).
* `max-backoff=diam`
    - A task will be scheduled for retry between
        min_backoff and
        max_backoff duration after it fails,
        if the queue&#39;s RetryConfig specifies that the task should be
        retried.
        
        If unspecified when the queue is created, Cloud Tasks will pick the
        default.
        
        
        `max_backoff` will be truncated to the nearest second.
        
        This field has the same meaning as
        [max_backoff_seconds in queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters).
* `max-doublings=46`
    - The time between retries will double `max_doublings` times.
        
        A task&#39;s retry interval starts at
        min_backoff, then doubles
        `max_doublings` times, then increases linearly, and finally
        retries retries at intervals of
        max_backoff up to
        max_attempts times.
        
        For example, if min_backoff is 10s,
        max_backoff is 300s, and
        `max_doublings` is 3, then the a task will first be retried in
        10s. The retry interval will double three times, and then
        increase linearly by 2^3 * 10s.  Finally, the task will retry at
        intervals of max_backoff until the
        task has been attempted max_attempts
        times. Thus, the requests will retry at 10s, 20s, 40s, 80s, 160s,
        240s, 300s, 300s, ....
        
        If unspecified when the queue is created, Cloud Tasks will pick the
        default.
        
        
        This field has the same meaning as
        [max_doublings in queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters).
* `max-retry-duration=lorem`
    - If positive, `max_retry_duration` specifies the time limit for
        retrying a failed task, measured from when the task was first
        attempted. Once `max_retry_duration` time has passed *and* the
        task has been attempted max_attempts
        times, no further attempts will be made and the task will be
        deleted.
        
        If zero, then the task age is unlimited.
        
        If unspecified when the queue is created, Cloud Tasks will pick the
        default.
        
        
        `max_retry_duration` will be truncated to the nearest second.
        
        This field has the same meaning as
        [task_age_limit in queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters).
* `min-backoff=et`
    - A task will be scheduled for retry between
        min_backoff and
        max_backoff duration after it fails,
        if the queue&#39;s RetryConfig specifies that the task should be
        retried.
        
        If unspecified when the queue is created, Cloud Tasks will pick the
        default.
        
        
        `min_backoff` will be truncated to the nearest second.
        
        This field has the same meaning as
        [min_backoff_seconds in queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters).

* `..    state=duo`
    - Output only. The state of the queue.
        
        `state` can only be changed by called
        PauseQueue,
        ResumeQueue, or uploading
        [queue.yaml/xml](https://cloud.google.com/appengine/docs/python/config/queueref).
        UpdateQueue cannot be used to change `state`.


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
    - A mask used to specify which fields of the queue are being updated.
        
        If empty, then all fields will be updated.

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
