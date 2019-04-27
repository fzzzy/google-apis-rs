Creates a managed instance group using the information that you specify in the request. After the group is created, it schedules an action to create instances in the group using the specified instance template. This operation is marked as DONE when the group is created even if the instances in the group have not yet been created. You must separately verify the status of the individual instances with the listmanagedinstances method.

A managed instance group can have up to 1000 VM instances per group. Please contact Cloud Support if you need an increase in this limit.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/compute*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `compute1 --scope <scope> instance-group-managers insert ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - Project ID for this request.
* **&lt;zone&gt;** *(string)*
    - The name of the zone where you want to create the managed instance group.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
InstanceGroupManager:
  base-instance-name: string
  creation-timestamp: string
  current-actions:
    abandoning: integer
    creating: integer
    creating-without-retries: integer
    deleting: integer
    none: integer
    recreating: integer
    refreshing: integer
    restarting: integer
  description: string
  fingerprint: string
  id: string
  instance-group: string
  instance-template: string
  kind: string
  name: string
  region: string
  self-link: string
  target-pools: [string]
  target-size: integer
  zone: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    base-instance-name=lorem`
    - The base instance name to use for instances in this group. The value must be 1-58 characters long. Instances are named by appending a hyphen and a random four-character string to the base instance name. The base instance name must comply with RFC1035.
* `creation-timestamp=sanctus`
    - [Output Only] The creation timestamp for this managed instance group in RFC3339 text format.
* `current-actions    abandoning=28`
    - [Output Only] The total number of instances in the managed instance group that are scheduled to be abandoned. Abandoning an instance removes it from the managed instance group without deleting it.
* `creating=68`
    - [Output Only] The number of instances in the managed instance group that are scheduled to be created or are currently being created. If the group fails to create any of these instances, it tries again until it creates the instance successfully.
        
        If you have disabled creation retries, this field will not be populated; instead, the creatingWithoutRetries field will be populated.
* `creating-without-retries=95`
    - [Output Only] The number of instances that the managed instance group will attempt to create. The group attempts to create each instance only once. If the group fails to create any of these instances, it decreases the group&#39;s targetSize value accordingly.
* `deleting=13`
    - [Output Only] The number of instances in the managed instance group that are scheduled to be deleted or are currently being deleted.
* `none=28`
    - [Output Only] The number of instances in the managed instance group that are running and have no scheduled actions.
* `recreating=79`
    - [Output Only] The number of instances in the managed instance group that are scheduled to be recreated or are currently being being recreated. Recreating an instance deletes the existing root persistent disk and creates a new disk from the image that is defined in the instance template.
* `refreshing=64`
    - [Output Only] The number of instances in the managed instance group that are being reconfigured with properties that do not require a restart or a recreate action. For example, setting or removing target pools for the instance.
* `restarting=61`
    - [Output Only] The number of instances in the managed instance group that are scheduled to be restarted or are currently being restarted.

* `..    description=sit`
    - An optional description of this resource. Provide this property when you create the resource.
* `fingerprint=elitr`
    - Fingerprint of this resource. This field may be used in optimistic locking. It will be ignored when inserting an InstanceGroupManager. An up-to-date fingerprint must be provided in order to update the InstanceGroupManager.
        
        To see the latest fingerprint, make a get() request to retrieve an InstanceGroupManager.
* `id=amet`
    - [Output Only] A unique identifier for this resource type. The server generates this identifier.
* `instance-group=ut`
    - [Output Only] The URL of the Instance Group resource.
* `instance-template=duo`
    - The URL of the instance template that is specified for this managed instance group. The group uses this template to create all new instances in the managed instance group.
* `kind=et`
    - [Output Only] The resource type, which is always compute#instanceGroupManager for managed instance groups.
* `name=est`
    - The name of the managed instance group. The name must be 1-63 characters long, and comply with RFC1035.
* `region=et`
    - [Output Only] The URL of the region where the managed instance group resides (for regional resources).
* `self-link=et`
    - [Output Only] The URL for this managed instance group. The server defines this URL.
* `target-pools=clita`
    - The URLs for all TargetPool resources to which instances in the instanceGroup field are added. The target pools automatically apply to all of the instances in the managed instance group.
    - Each invocation of this argument appends the given value to the array.
* `target-size=56`
    - The target number of running instances for this managed instance group. Deleting or abandoning instances reduces this number. Resizing the group changes this number.
* `zone=magna`
    - [Output Only] The URL of the zone where the managed instance group is located (for zonal resources).


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

* **-p request-id=string**
    - An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed.
        
        For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments.
        
        The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).

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
