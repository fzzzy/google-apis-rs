Creates a new span.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/trace.append*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudtrace2 --scope <scope> projects traces-spans-create-span ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The resource name of the span in the following format:
        
            projects/[PROJECT_ID]/traces/[TRACE_ID]/spans/SPAN_ID is a unique identifier for a trace within a project;
        it is a 32-character hexadecimal encoding of a 16-byte array.
        
        [SPAN_ID] is a unique identifier for a span within a trace; it
        is a 16-character hexadecimal encoding of an 8-byte array.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Span:
  attributes:
    dropped-attributes-count: integer
  child-span-count: integer
  display-name:
    truncated-byte-count: integer
    value: string
  end-time: string
  links:
    dropped-links-count: integer
  name: string
  parent-span-id: string
  same-process-as-parent-span: boolean
  span-id: string
  stack-trace:
    stack-frames:
      dropped-frames-count: integer
    stack-trace-hash-id: string
  start-time: string
  status:
    code: integer
    message: string
  time-events:
    dropped-annotations-count: integer
    dropped-message-events-count: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .attributes    dropped-attributes-count=62`
    - The number of attributes that were discarded. Attributes can be discarded
        because their keys are too long or because there are too many attributes.
        If this value is 0 then all attributes are valid.

* `..    child-span-count=53`
    - An optional number of child spans that were generated while this span
        was active. If set, allows implementation to detect missing child spans.
* `display-name    truncated-byte-count=36`
    - The number of bytes removed from the original string. If this
        value is 0, then the string was not shortened.
* `value=sed`
    - The shortened string. For example, if the original string is 500
        bytes long and the limit of the string is 128 bytes, then
        `value` contains the first 128 bytes of the 500-byte string.
        
        Truncation always happens on a UTF8 character boundary. If there
        are multi-byte characters in the string, then the length of the
        shortened string might be less than the size limit.

* `..    end-time=et`
    - The end time of the span. On the client side, this is the time kept by
        the local machine where the span execution ends. On the server side, this
        is the time when the server application handler stops running.
* `links    dropped-links-count=83`
    - The number of dropped links after the maximum size was enforced. If
        this value is 0, then no links were dropped.

* `..    name=kasd`
    - The resource name of the span in the following format:
        
            projects/[PROJECT_ID]/traces/[TRACE_ID]/spans/SPAN_ID is a unique identifier for a trace within a project;
        it is a 32-character hexadecimal encoding of a 16-byte array.
        
        [SPAN_ID] is a unique identifier for a span within a trace; it
        is a 16-character hexadecimal encoding of an 8-byte array.
* `parent-span-id=accusam`
    - The [SPAN_ID] of this span&#39;s parent span. If this is a root span,
        then this field must be empty.
* `same-process-as-parent-span=true`
    - (Optional) Set this parameter to indicate whether this span is in
        the same process as its parent. If you do not set this parameter,
        Stackdriver Trace is unable to take advantage of this helpful
        information.
* `span-id=justo`
    - The [SPAN_ID] portion of the span&#39;s resource name.
* `stack-trace.stack-frames    dropped-frames-count=100`
    - The number of stack frames that were dropped because there
        were too many stack frames.
        If this value is 0, then no stack frames were dropped.

* `..    stack-trace-hash-id=erat`
    - The hash ID is used to conserve network bandwidth for duplicate
        stack traces within a single trace.
        
        Often multiple spans will have identical stack traces.
        The first occurrence of a stack trace should contain both the
        `stackFrame` content and a value in `stackTraceHashId`.
        
        Subsequent spans within the same request can refer
        to that stack trace by only setting `stackTraceHashId`.

* `..    start-time=labore`
    - The start time of the span. On the client side, this is the time kept by
        the local machine where the span execution starts. On the server side, this
        is the time when the server&#39;s application handler starts running.
* `status    code=92`
    - The status code, which should be an enum value of google.rpc.Code.
* `message=nonumy`
    - A developer-facing error message, which should be in English. Any
        user-facing error message should be localized and sent in the
        google.rpc.Status.details field, or localized by the client.

* `..time-events    dropped-annotations-count=82`
    - The number of dropped annotations in all the included time events.
        If the value is 0, then no annotations were dropped.
* `dropped-message-events-count=40`
    - The number of dropped message events in all the included time events.
        If the value is 0, then no message events were dropped.



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
