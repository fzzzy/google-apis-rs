Writes log entries to Logging. This API method is the only way to send log entries to Logging. This method is used, directly or indirectly, by the Logging agent (fluentd) and all logging libraries configured to use Logging. A single request may contain log entries for a maximum of 1000 different resources (projects, organizations, billing accounts or folders)
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/logging.admin*
* *https://www.googleapis.com/auth/logging.write*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `logging2-beta1 --scope <scope> entries write ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
WriteLogEntriesRequest:
  dry-run: boolean
  labels: { string: string }
  log-name: string
  partial-success: boolean
  resource:
    labels: { string: string }
    type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    dry-run=false`
    - Optional. If true, the request should expect normal response, but the entries won&#39;t be persisted nor exported. Useful for checking whether the logging API endpoints are working properly before sending valuable data.
* `labels=key=accusam`
    - Optional. Default labels that are added to the labels field of all log entries in entries. If a log entry already has a label with the same key as a label in this parameter, then the log entry&#39;s label is not changed. See LogEntry.
    - the value will be associated with the given `key`
* `log-name=takimata`
    - Optional. A default log resource name that is assigned to all log entries in entries that do not specify a value for log_name:
        &#34;projects/[PROJECT_ID]/logs/[LOG_ID]&#34;
        &#34;organizations/[ORGANIZATION_ID]/logs/[LOG_ID]&#34;
        &#34;billingAccounts/[BILLING_ACCOUNT_ID]/logs/[LOG_ID]&#34;
        &#34;folders/[FOLDER_ID]/logs/[LOG_ID]&#34;
        [LOG_ID] must be URL-encoded. For example:
        &#34;projects/my-project-id/logs/syslog&#34;
        &#34;organizations/1234567890/logs/cloudresourcemanager.googleapis.com%2Factivity&#34;
        The permission &lt;code&gt;logging.logEntries.create&lt;/code&gt; is needed on each project, organization, billing account, or folder that is receiving new log entries, whether the resource is specified in &lt;code&gt;logName&lt;/code&gt; or in an individual log entry.
* `partial-success=false`
    - Optional. Whether valid entries should be written even if some other entries fail due to INVALID_ARGUMENT or PERMISSION_DENIED errors. If any entry is not written, then the response status is the error associated with one of the failed entries and the response includes error details keyed by the entries&#39; zero-based index in the entries.write method.
* `resource    labels=key=amet.`
    - Required. Values for all of the labels listed in the associated monitored resource descriptor. For example, Compute Engine VM instances use the labels &#34;project_id&#34;, &#34;instance_id&#34;, and &#34;zone&#34;.
    - the value will be associated with the given `key`
* `type=erat`
    - Required. The monitored resource type. This field must match the type field of a MonitoredResourceDescriptor object. For example, the type of a Compute Engine VM instance is gce_instance.



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
