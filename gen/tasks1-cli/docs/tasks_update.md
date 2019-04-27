Updates the specified task.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/tasks* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/tasks*.
You can set the scope for this method like this: `tasks1 --scope <scope> tasks update ...`
# Required Scalar Arguments
* **&lt;tasklist&gt;** *(string)*
    - Task list identifier.
* **&lt;task&gt;** *(string)*
    - Task identifier.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Task:
  completed: string
  deleted: boolean
  due: string
  etag: string
  hidden: boolean
  id: string
  kind: string
  notes: string
  parent: string
  position: string
  self-link: string
  status: string
  title: string
  updated: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    completed=accusam`
    - Completion date of the task (as a RFC 3339 timestamp). This field is omitted if the task has not been completed.
* `deleted=false`
    - Flag indicating whether the task has been deleted. The default if False.
* `due=sea`
    - Due date of the task (as a RFC 3339 timestamp). Optional.
* `etag=et`
    - ETag of the resource.
* `hidden=false`
    - Flag indicating whether the task is hidden. This is the case if the task had been marked completed when the task list was last cleared. The default is False. This field is read-only.
* `id=et`
    - Task identifier.
* `kind=eirmod`
    - Type of the resource. This is always &#34;tasks#task&#34;.
* `notes=sanctus`
    - Notes describing the task. Optional.
* `parent=et`
    - Parent task identifier. This field is omitted if it is a top-level task. This field is read-only. Use the &#34;move&#34; method to move the task under a different parent or to the top level.
* `position=amet`
    - String indicating the position of the task among its sibling tasks under the same parent task or at the top level. If this string is greater than another task&#39;s corresponding position string according to lexicographical ordering, the task is positioned after the other task under the same parent task (or at the top level). This field is read-only. Use the &#34;move&#34; method to move the task to another position.
* `self-link=et`
    - URL pointing to this task. Used to retrieve, update, or delete this task.
* `status=consetetur`
    - Status of the task. This is either &#34;needsAction&#34; or &#34;completed&#34;.
* `title=ut`
    - Title of the task.
* `updated=ea`
    - Last modification time of the task (as a RFC 3339 timestamp).


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
    - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.

* **-p user-ip=string**
    - Deprecated. Please use quotaUser instead.
