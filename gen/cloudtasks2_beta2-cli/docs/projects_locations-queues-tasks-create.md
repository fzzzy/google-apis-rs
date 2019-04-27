Creates a task and adds it to a queue.

Tasks cannot be updated after creation; there is no UpdateTask command.

* For App Engine queues, the maximum task size is
  100KB.
* For pull queues, the maximum task size is 1MB.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudtasks2-beta2 --scope <scope> projects locations-queues-tasks-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required.
        
        The queue name. For example:
        `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
        
        The queue must already exist.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CreateTaskRequest:
  response-view: string
  task:
    app-engine-http-request:
      app-engine-routing:
        host: string
        instance: string
        service: string
        version: string
      headers: { string: string }
      http-method: string
      payload: string
      relative-url: string
    create-time: string
    name: string
    pull-message:
      payload: string
      tag: string
    schedule-time: string
    status:
      attempt-dispatch-count: integer
      attempt-response-count: integer
      first-attempt-status:
        dispatch-time: string
        response-status:
          code: integer
          message: string
        response-time: string
        schedule-time: string
      last-attempt-status:
        dispatch-time: string
        response-status:
          code: integer
          message: string
        response-time: string
        schedule-time: string
    view: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    response-view=eirmod`
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
* `task.app-engine-http-request.app-engine-routing    host=elitr`
    - Output only. The host that the task is sent to.
        
        For more information, see
        [How Requests are Routed](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed).
        
        The host is constructed as:
        
        
        * `host = [application_domain_name]`&lt;/br&gt;
          `| [service] + &#39;.&#39; + [application_domain_name]`&lt;/br&gt;
          `| [version] + &#39;.&#39; + [application_domain_name]`&lt;/br&gt;
          `| [version_dot_service]+ &#39;.&#39; + [application_domain_name]`&lt;/br&gt;
          `| [instance] + &#39;.&#39; + [application_domain_name]`&lt;/br&gt;
          `| [instance_dot_service] + &#39;.&#39; + [application_domain_name]`&lt;/br&gt;
          `| [instance_dot_version] + &#39;.&#39; + [application_domain_name]`&lt;/br&gt;
          `| [instance_dot_version_dot_service] + &#39;.&#39; + [application_domain_name]`
        
        * `application_domain_name` = The domain name of the app, for
          example &lt;app-id&gt;.appspot.com, which is associated with the
          queue&#39;s project ID. Some tasks which were created using the App Engine
          SDK use a custom domain name.
        
        * `service =` service
        
        * `version =` version
        
        * `version_dot_service =`
          version `+ &#39;.&#39; +`
          service
        
        * `instance =` instance
        
        * `instance_dot_service =`
          instance `+ &#39;.&#39; +`
          service
        
        * `instance_dot_version =`
          instance `+ &#39;.&#39; +`
          version
        
        * `instance_dot_version_dot_service =`
          instance `+ &#39;.&#39; +`
          version `+ &#39;.&#39; +`
          service
        
        If service is empty, then the task will be sent
        to the service which is the default service when the task is attempted.
        
        If version is empty, then the task will be sent
        to the version which is the default version when the task is attempted.
        
        If instance is empty, then the task
        will be sent to an instance which is available when the task is
        attempted.
        
        If service,
        version, or
        instance is invalid, then the task
        will be sent to the default version of the default service when
        the task is attempted.
* `instance=amet`
    - App instance.
        
        By default, the task is sent to an instance which is available when
        the task is attempted.
        
        Requests can only be sent to a specific instance if
        [manual scaling is used in App Engine Standard](https://cloud.google.com/appengine/docs/python/an-overview-of-app-engine?hl=en_US#scaling_types_and_instance_classes).
        App Engine Flex does not support instances. For more information, see
        [App Engine Standard request routing](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed)
        and [App Engine Flex request routing](https://cloud.google.com/appengine/docs/flexible/python/how-requests-are-routed).
* `service=no`
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
* `version=labore`
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

* `..    headers=key=eirmod`
    - HTTP request headers.
        
        This map contains the header field names and values.
        Headers can be set when the
        task is created.
        Repeated headers are not supported but a header value can contain commas.
        
        Cloud Tasks sets some headers to default values:
        
        * `User-Agent`: By default, this header is
          `&#34;AppEngine-Google; (+http://code.google.com/appengine)&#34;`.
          This header can be modified, but Cloud Tasks will append
          `&#34;AppEngine-Google; (+http://code.google.com/appengine)&#34;` to the
          modified `User-Agent`.
        
        If the task has a payload, Cloud
        Tasks sets the following headers:
        
        * `Content-Type`: By default, the `Content-Type` header is set to
          `&#34;application/octet-stream&#34;`. The default can be overridden by explicitly
          setting `Content-Type` to a particular media type when the
          task is created.
          For example, `Content-Type` can be set to `&#34;application/json&#34;`.
        * `Content-Length`: This is computed by Cloud Tasks. This value is
          output only.   It cannot be changed.
        
        The headers below cannot be set or overridden:
        
        * `Host`
        * `X-Google-*`
        * `X-AppEngine-*`
        
        In addition, Cloud Tasks sets some headers when the task is dispatched,
        such as headers containing information about the task; see
        [request headers](https://cloud.google.com/appengine/docs/python/taskqueue/push/creating-handlers#reading_request_headers).
        These headers are set only when the task is dispatched, so they are not
        visible when the task is returned in a Cloud Tasks response.
        
        Although there is no specific limit for the maximum number of headers or
        the size, there is a limit on the maximum size of the Task. For more
        information, see the CreateTask documentation.
    - the value will be associated with the given `key`
* `http-method=dolore`
    - The HTTP method to use for the request. The default is POST.
        
        The app&#39;s request handler for the task&#39;s target URL must be able to handle
        HTTP requests with this http_method, otherwise the task attempt will fail
        with error code 405 (Method Not Allowed). See
        [Writing a push task request handler](https://cloud.google.com/appengine/docs/java/taskqueue/push/creating-handlers#writing_a_push_task_request_handler)
        and the documentation for the request handlers in the language your app is
        written in e.g.
        [Python Request Handler](https://cloud.google.com/appengine/docs/python/tools/webapp/requesthandlerclass).
* `payload=invidunt`
    - Payload.
        
        The payload will be sent as the HTTP message body. A message
        body, and thus a payload, is allowed only if the HTTP method is
        POST or PUT. It is an error to set a data payload on a task with
        an incompatible HttpMethod.
* `relative-url=aliquyam`
    - The relative URL.
        
        The relative URL must begin with &#34;/&#34; and must be a valid HTTP relative URL.
        It can contain a path and query string arguments.
        If the relative URL is empty, then the root path &#34;/&#34; will be used.
        No spaces are allowed, and the maximum length allowed is 2083 characters.

* `..    create-time=accusam`
    - Output only. The time that the task was created.
        
        `create_time` will be truncated to the nearest second.
* `name=lorem`
    - Optionally caller-specified in CreateTask.
        
        The task name.
        
        The task name must have the following format:
        `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
        
        * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]),
           hyphens (-), colons (:), or periods (.).
           For more information, see
           [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects)
        * `LOCATION_ID` is the canonical ID for the task&#39;s location.
           The list of available locations can be obtained by calling
           ListLocations.
           For more information, see https://cloud.google.com/about/locations/.
        * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or
          hyphens (-). The maximum length is 100 characters.
        * `TASK_ID` can contain only letters ([A-Za-z]), numbers ([0-9]),
          hyphens (-), or underscores (_). The maximum length is 500 characters.
* `pull-message    payload=sea`
    - A data payload consumed by the worker to execute the task.
* `tag=et`
    - The task&#39;s tag.
        
        Tags allow similar tasks to be processed in a batch. If you label
        tasks with a tag, your worker can
        lease tasks with the same tag using
        filter. For example, if you want to
        aggregate the events associated with a specific user once a day,
        you could tag tasks with the user ID.
        
        The task&#39;s tag can only be set when the
        task is created.
        
        The tag must be less than 500 characters.
        
        SDK compatibility: Although the SDK allows tags to be either
        string or [bytes](https://cloud.google.com/appengine/docs/standard/java/javadoc/com/google/appengine/api/taskqueue/TaskOptions.html#tag-byte:A-),
        only UTF-8 encoded tags can be used in Cloud Tasks. If a tag isn&#39;t UTF-8
        encoded, the tag will be empty when the task is returned by Cloud Tasks.

* `..    schedule-time=duo`
    - The time when the task is scheduled to be attempted.
        
        For App Engine queues, this is when the task will be attempted or retried.
        
        For pull queues, this is the time when the task is available to
        be leased; if a task is currently leased, this is the time when
        the current lease expires, that is, the time that the task was
        leased plus the lease_duration.
        
        `schedule_time` will be truncated to the nearest microsecond.
* `status    attempt-dispatch-count=80`
    - Output only. The number of attempts dispatched.
        
        This count includes tasks which have been dispatched but haven&#39;t
        received a response.
* `attempt-response-count=61`
    - Output only. The number of attempts which have received a response.
        
        This field is not calculated for pull tasks.
* `first-attempt-status    dispatch-time=sanctus`
    - Output only. The time that this attempt was dispatched.
        
        `dispatch_time` will be truncated to the nearest microsecond.
* `response-status    code=79`
    - The status code, which should be an enum value of google.rpc.Code.
* `message=amet`
    - A developer-facing error message, which should be in English. Any
        user-facing error message should be localized and sent in the
        google.rpc.Status.details field, or localized by the client.

* `..    response-time=et`
    - Output only. The time that this attempt response was received.
        
        `response_time` will be truncated to the nearest microsecond.
* `schedule-time=consetetur`
    - Output only. The time that this attempt was scheduled.
        
        `schedule_time` will be truncated to the nearest microsecond.

* `..last-attempt-status    dispatch-time=ut`
    - Output only. The time that this attempt was dispatched.
        
        `dispatch_time` will be truncated to the nearest microsecond.
* `response-status    code=85`
    - The status code, which should be an enum value of google.rpc.Code.
* `message=sed`
    - A developer-facing error message, which should be in English. Any
        user-facing error message should be localized and sent in the
        google.rpc.Status.details field, or localized by the client.

* `..    response-time=dolor`
    - Output only. The time that this attempt response was received.
        
        `response_time` will be truncated to the nearest microsecond.
* `schedule-time=dolor`
    - Output only. The time that this attempt was scheduled.
        
        `schedule_time` will be truncated to the nearest microsecond.


* `...    view=dolor`
    - Output only. The view specifies which subset of the Task has
        been returned.



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
