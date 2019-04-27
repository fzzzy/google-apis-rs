Creates a sink that exports specified log entries to a destination. The export of newly-ingested log entries begins immediately, unless the sink&#39;s writer_identity is not permitted to write to the destination. A sink can export log entries only from the resource owning the sink.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/logging.admin*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `logging2 --scope <scope> billing-accounts sinks-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The resource in which to create the sink:
        &#34;projects/[PROJECT_ID]&#34;
        &#34;organizations/[ORGANIZATION_ID]&#34;
        &#34;billingAccounts/[BILLING_ACCOUNT_ID]&#34;
        &#34;folders/[FOLDER_ID]&#34;
        Examples: &#34;projects/my-logging-project&#34;, &#34;organizations/123456789&#34;.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
LogSink:
  destination: string
  end-time: string
  filter: string
  include-children: boolean
  name: string
  output-version-format: string
  start-time: string
  writer-identity: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    destination=takimata`
    - Required. The export destination:
        &#34;storage.googleapis.com/[GCS_BUCKET]&#34;
        &#34;bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]&#34;
        &#34;pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]&#34;
        The sink&#39;s writer_identity, set when the sink is created, must have permission to write to the destination or else the log entries are not exported. For more information, see Exporting Logs With Sinks.
* `end-time=justo`
    - Deprecated. This field is ignored when creating or updating sinks.
* `filter=amet.`
    - Optional. An advanced logs filter. The only exported log entries are those that are in the resource owning the sink and that match the filter. For example:
        logName=&#34;projects/[PROJECT_ID]/logs/[LOG_ID]&#34; AND severity&gt;=ERROR
        
* `include-children=false`
    - Optional. This field applies only to sinks owned by organizations and folders. If the field is false, the default, only the logs owned by the sink&#39;s parent resource are available for export. If the field is true, then logs from all the projects, folders, and billing accounts contained in the sink&#39;s parent resource are also available for export. Whether a particular log entry from the children is exported depends on the sink&#39;s filter expression. For example, if this field is true, then the filter resource.type=gce_instance would export all Compute Engine VM instance log entries from all projects in the sink&#39;s parent. To only export entries from certain child projects, filter on the project part of the log name:
        logName:(&#34;projects/test-project1/&#34; OR &#34;projects/test-project2/&#34;) AND
        resource.type=gce_instance
        
* `name=labore`
    - Required. The client-assigned sink identifier, unique within the project. Example: &#34;my-syslog-errors-to-pubsub&#34;. Sink identifiers are limited to 100 characters and can include only the following characters: upper and lower-case alphanumeric characters, underscores, hyphens, and periods.
* `output-version-format=sea`
    - Deprecated. The log entry format to use for this sink&#39;s exported log entries. The v2 format is used by default and cannot be changed.
* `start-time=nonumy`
    - Deprecated. This field is ignored when creating or updating sinks.
* `writer-identity=dolores`
    - Output only. An IAM identity&amp;mdash;a service account or group&amp;mdash;under which Logging writes the exported log entries to the sink&#39;s destination. This field is set by sinks.create and sinks.update, based on the setting of unique_writer_identity in those methods.Until you grant this identity write-access to the destination, log entry exports from this sink will fail. For more information, see Granting access for a resource. Consult the destination service&#39;s documentation to determine the appropriate IAM roles to assign to the identity.


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

* **-p unique-writer-identity=boolean**
    - Optional. Determines the kind of IAM identity returned as writer_identity in the new sink. If this value is omitted or set to false, and if the sink&#39;s parent is a project, then the value returned as writer_identity is the same group or service account used by Logging before the addition of writer identities to this API. The sink&#39;s destination must be in the same project as the sink itself.If this field is set to true, or if the sink is owned by a non-project resource such as an organization, then the value of writer_identity will be a unique service account used only for exports from the new sink. For more information, see writer_identity in LogSink.

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
