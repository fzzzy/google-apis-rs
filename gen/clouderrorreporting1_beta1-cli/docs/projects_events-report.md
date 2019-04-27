Report an individual error event.

This endpoint accepts **either** an OAuth token,
**or** an [API key](https://support.google.com/cloud/answer/6158862)
for authentication. To use an API key, append it to the URL as the value of
a `key` parameter. For example:

`POST https://clouderrorreporting.googleapis.com/v1beta1/projects/example-project/events:report?key=123ABC456`
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `clouderrorreporting1-beta1 --scope <scope> projects events-report ...`
# Required Scalar Argument
* **&lt;project-name&gt;** *(string)*
    - [Required] The resource name of the Google Cloud Platform project. Written
        as `projects/` plus the
        [Google Cloud Platform project ID](https://support.google.com/cloud/answer/6158840).
        Example: `projects/my-project-123`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ReportedErrorEvent:
  context:
    http-request:
      method: string
      referrer: string
      remote-ip: string
      response-status-code: integer
      url: string
      user-agent: string
    report-location:
      file-path: string
      function-name: string
      line-number: integer
    user: string
  event-time: string
  message: string
  service-context:
    resource-type: string
    service: string
    version: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .context.http-request    method=eirmod`
    - The type of HTTP request, such as `GET`, `POST`, etc.
* `referrer=sit`
    - The referrer information that is provided with the request.
* `remote-ip=stet`
    - The IP address from which the request originated.
        This can be IPv4, IPv6, or a token which is derived from the
        IP address, depending on the data that has been provided
        in the error report.
* `response-status-code=59`
    - The HTTP response status code for the request.
* `url=et`
    - The URL of the request.
* `user-agent=dolores`
    - The user agent information that is provided with the request.

* `..report-location    file-path=kasd`
    - The source code filename, which can include a truncated relative
        path, or a full path from a production machine.
* `function-name=accusam`
    - Human-readable name of a function or method.
        The value can include optional context like the class or package name.
        For example, `my.package.MyClass.method` in case of Java.
* `line-number=93`
    - 1-based. 0 indicates that the line number is unknown.

* `..    user=justo`
    - The user who caused or was affected by the crash.
        This can be a user ID, an email address, or an arbitrary token that
        uniquely identifies the user.
        When sending an error report, leave this field empty if the user was not
        logged in. In this case the
        Error Reporting system will use other data, such as remote IP address, to
        distinguish affected users. See `affected_users_count` in
        `ErrorGroupStats`.

* `..    event-time=amet.`
    - [Optional] Time when the event occurred.
        If not provided, the time when the event was received by the
        Error Reporting system will be used.
* `message=erat`
    - [Required] The error message.
        If no `context.reportLocation` is provided, the message must contain a
        header (typically consisting of the exception type name and an error
        message) and an exception stack trace in one of the supported programming
        languages and formats.
        Supported languages are Java, Python, JavaScript, Ruby, C#, PHP, and Go.
        Supported stack trace formats are:
        
        * **Java**: Must be the return value of [`Throwable.printStackTrace()`](https://docs.oracle.com/javase/7/docs/api/java/lang/Throwable.html#printStackTrace%28%29).
        * **Python**: Must be the return value of [`traceback.format_exc()`](https://docs.python.org/2/library/traceback.html#traceback.format_exc).
        * **JavaScript**: Must be the value of [`error.stack`](https://github.com/v8/v8/wiki/Stack-Trace-API)
        as returned by V8.
        * **Ruby**: Must contain frames returned by [`Exception.backtrace`](https://ruby-doc.org/core-2.2.0/Exception.html#method-i-backtrace).
        * **C#**: Must be the return value of [`Exception.ToString()`](https://msdn.microsoft.com/en-us/library/system.exception.tostring.aspx).
        * **PHP**: Must start with `PHP (Notice|Parse error|Fatal error|Warning)`
        and contain the result of [`(string)$exception`](http://php.net/manual/en/exception.tostring.php).
        * **Go**: Must be the return value of [`runtime.Stack()`](https://golang.org/pkg/runtime/debug/#Stack).
* `service-context    resource-type=labore`
    - Type of the MonitoredResource. List of possible values:
        https://cloud.google.com/monitoring/api/resources
        
        Value is set automatically for incoming errors and must not be set when
        reporting errors.
* `service=sea`
    - An identifier of the service, such as the name of the
        executable, job, or Google App Engine service name. This field is expected
        to have a low number of values that are relatively stable over time, as
        opposed to `version`, which can be changed whenever new code is deployed.
        
        Contains the service name for error reports extracted from Google
        App Engine logs or `default` if the App Engine default service is used.
* `version=nonumy`
    - Represents the source code version that the developer provided,
        which could represent a version label or a Git SHA-1 hash, for example.
        For App Engine standard environment, the version is set to the version of
        the app.



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
