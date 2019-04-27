Insert a new task in a TaskQueue
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/taskqueue*
* *https://www.googleapis.com/auth/taskqueue.consumer*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/taskqueue*.
You can set the scope for this method like this: `taskqueue1-beta2 --scope <scope> tasks insert ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - The project under which the queue lies
* **&lt;taskqueue&gt;** *(string)*
    - The taskqueue to insert the task into
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Task:
  enqueue-timestamp: string
  id: string
  kind: string
  lease-timestamp: string
  payload-base64: string
  queue-name: string
  retry-count: integer
  tag: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    enqueue-timestamp=eirmod`
    - Time (in seconds since the epoch) at which the task was enqueued.
* `id=sit`
    - Name of the task.
* `kind=stet`
    - The kind of object returned, in this case set to task.
* `lease-timestamp=sed`
    - Time (in seconds since the epoch) at which the task lease will expire. This value is 0 if the task isnt currently leased out to a worker.
* `payload-base64=et`
    - A bag of bytes which is the task payload. The payload on the JSON side is always Base64 encoded.
* `queue-name=dolores`
    - Name of the queue that the task is in.
* `retry-count=38`
    - The number of leases applied to this task.
* `tag=accusam`
    - Tag for the task, could be used later to lease tasks grouped by a specific tag.


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
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.

* **-p user-ip=string**
    - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
