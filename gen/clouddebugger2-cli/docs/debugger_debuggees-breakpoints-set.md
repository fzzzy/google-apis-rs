Sets the breakpoint to the debuggee.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloud_debugger*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `clouddebugger2 --scope <scope> debugger debuggees-breakpoints-set ...`
# Required Scalar Argument
* **&lt;debuggee-id&gt;** *(string)*
    - ID of the debuggee where the breakpoint is to be set.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Breakpoint:
  action: string
  condition: string
  create-time: string
  expressions: [string]
  final-time: string
  id: string
  is-final-state: boolean
  labels: { string: string }
  location:
    column: integer
    line: integer
    path: string
  log-level: string
  log-message-format: string
  status:
    description:
      format: string
      parameters: [string]
    is-error: boolean
    refers-to: string
  user-email: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    action=aliquyam`
    - Action that the agent should perform when the code at the
        breakpoint location is hit.
* `condition=sea`
    - Condition that triggers the breakpoint.
        The condition is a compound boolean expression composed using expressions
        in a programming language at the source location.
* `create-time=lorem`
    - Time this breakpoint was created by the server in seconds resolution.
* `expressions=eos`
    - List of read-only expressions to evaluate at the breakpoint location.
        The expressions are composed using expressions in the programming language
        at the source location. If the breakpoint action is `LOG`, the evaluated
        expressions are included in log statements.
    - Each invocation of this argument appends the given value to the array.
* `final-time=erat`
    - Time this breakpoint was finalized as seen by the server in seconds
        resolution.
* `id=sadipscing`
    - Breakpoint identifier, unique in the scope of the debuggee.
* `is-final-state=true`
    - When true, indicates that this is a final result and the
        breakpoint state will not change from here on.
* `labels=key=eirmod`
    - A set of custom breakpoint properties, populated by the agent, to be
        displayed to the user.
    - the value will be associated with the given `key`
* `location    column=58`
    - Column within a line. The first column in a line as the value `1`.
        Agents that do not support setting breakpoints on specific columns ignore
        this field.
* `line=4`
    - Line inside the file. The first line in the file has the value `1`.
* `path=no`
    - Path to the source file within the source context of the target binary.

* `..    log-level=labore`
    - Indicates the severity of the log. Only relevant when action is `LOG`.
* `log-message-format=eirmod`
    - Only relevant when action is `LOG`. Defines the message to log when
        the breakpoint hits. The message may include parameter placeholders `$0`,
        `$1`, etc. These placeholders are replaced with the evaluated value
        of the appropriate expression. Expressions not referenced in
        `log_message_format` are not logged.
        
        Example: `Message received, id = $0, count = $1` with
        `expressions` = `[ message.id, message.count ]`.
* `status.description    format=dolore`
    - Format template for the message. The `format` uses placeholders `$0`,
        `$1`, etc. to reference parameters. `$$` can be used to denote the `$`
        character.
        
        Examples:
        
        *   `Failed to load &#39;$0&#39; which helps debug $1 the first time it
            is loaded.  Again, $0 is very important.`
        *   `Please pay $$10 to use $0 instead of $1.`
* `parameters=invidunt`
    - Optional parameters to be embedded into the message.
    - Each invocation of this argument appends the given value to the array.

* `..    is-error=false`
    - Distinguishes errors from informational messages.
* `refers-to=accusam`
    - Reference to which the message applies.

* `..    user-email=lorem`
    - E-mail address of the user that created this breakpoint


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

* **-p client-version=string**
    - The client version making the call.
        Schema: `domain/type/version` (e.g., `google.com/intellij/v1`).

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
