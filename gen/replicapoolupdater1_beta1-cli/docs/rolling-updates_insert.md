Inserts and starts a new update.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/replicapool*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `replicapoolupdater1-beta1 --scope <scope> rolling-updates insert ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - The Google Developers Console project name.
* **&lt;zone&gt;** *(string)*
    - The name of the zone in which the update&#39;s target resides.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
RollingUpdate:
  action-type: string
  creation-timestamp: string
  description: string
  id: string
  instance-group: string
  instance-group-manager: string
  instance-template: string
  kind: string
  old-instance-template: string
  policy:
    auto-pause-after-instances: integer
    instance-startup-timeout-sec: integer
    max-num-concurrent-instances: integer
    max-num-failed-instances: integer
    min-instance-update-time-sec: integer
  progress: integer
  self-link: string
  status: string
  status-message: string
  user: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    action-type=eirmod`
    - Specifies the action to take for each instance within the instance group. This can be RECREATE which will recreate each instance and is only available for managed instance groups. It can also be REBOOT which performs a soft reboot for each instance and is only available for regular (non-managed) instance groups.
* `creation-timestamp=sit`
    - [Output Only] Creation timestamp in RFC3339 text format.
* `description=stet`
    - An optional textual description of the resource; provided by the client when the resource is created.
* `id=sed`
    - [Output Only] Unique identifier for the resource; defined by the server.
* `instance-group=et`
    - Fully-qualified URL of an instance group being updated. Exactly one of instanceGroupManager and instanceGroup must be set.
* `instance-group-manager=dolores`
    - Fully-qualified URL of an instance group manager being updated. Exactly one of instanceGroupManager and instanceGroup must be set.
* `instance-template=kasd`
    - Fully-qualified URL of an instance template to apply.
* `kind=accusam`
    - [Output Only] Type of the resource.
* `old-instance-template=takimata`
    - Fully-qualified URL of the instance template encountered while starting the update.
* `policy    auto-pause-after-instances=31`
    - Number of instances to update before the updater pauses the rolling update.
* `instance-startup-timeout-sec=100`
    - The maximum amount of time that the updater waits for a HEALTHY state after all of the update steps are complete. If the HEALTHY state is not received before the deadline, the instance update is considered a failure.
* `max-num-concurrent-instances=20`
    - The maximum number of instances that can be updated simultaneously. An instance update is considered complete only after the instance is restarted and initialized.
* `max-num-failed-instances=66`
    - The maximum number of instance updates that can fail before the group update is considered a failure. An instance update is considered failed if any of its update actions (e.g. Stop call on Instance resource in Rolling Reboot) failed with permanent failure, or if the instance is in an UNHEALTHY state after it finishes all of the update actions.
* `min-instance-update-time-sec=92`
    - The minimum amount of time that the updater spends to update each instance. Update time is the time it takes to complete all update actions (e.g. Stop call on Instance resource in Rolling Reboot), reboot, and initialize. If the instance update finishes early, the updater pauses for the remainder of the time before it starts the next instance update.

* `..    progress=11`
    - [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess at when the update will be complete. This number should be monotonically increasing as the update progresses.
* `self-link=dolores`
    - [Output Only] The fully qualified URL for the resource.
* `status=gubergren`
    - [Output Only] Status of the update. Possible values are:  
        - &#34;ROLLING_FORWARD&#34;: The update is going forward. 
        - &#34;ROLLING_BACK&#34;: The update is being rolled back. 
        - &#34;PAUSED&#34;: The update is temporarily paused (inactive). 
        - &#34;ROLLED_OUT&#34;: The update is finished, all instances have been updated successfully. 
        - &#34;ROLLED_BACK&#34;: The update is finished, all instances have been reverted to the previous template. 
        - &#34;CANCELLED&#34;: The update is paused and no longer can be resumed, undefined how many instances are running in which template.
* `status-message=sadipscing`
    - [Output Only] An optional textual description of the current status of the update.
* `user=aliquyam`
    - [Output Only] User who requested the update, for example: user@example.com.


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
