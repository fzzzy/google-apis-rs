Creates an instance group manager, as well as the instance group and the specified number of instances.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/compute*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `replicapool1-beta2 --scope <scope> instance-group-managers insert ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - The Google Developers Console project name.
* **&lt;zone&gt;** *(string)*
    - The name of the zone in which the instance group manager resides.
* **&lt;size&gt;** *(integer)*
    - Number of instances that should exist.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
InstanceGroupManager:
  base-instance-name: string
  creation-timestamp: string
  current-size: integer
  description: string
  fingerprint: string
  group: string
  id: string
  instance-template: string
  kind: string
  name: string
  self-link: string
  target-pools: [string]
  target-size: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    base-instance-name=stet`
    - The base instance name to use for instances in this group. The value must be a valid RFC1035 name. Supported characters are lowercase letters, numbers, and hyphens (-). Instances are named by appending a hyphen and a random four-character string to the base instance name.
* `creation-timestamp=sed`
    - [Output only] The time the instance group manager was created, in RFC3339 text format.
* `current-size=16`
    - [Output only] The number of instances that currently exist and are a part of this group. This includes instances that are starting but are not yet RUNNING, and instances that are in the process of being deleted or abandoned.
* `description=dolores`
    - An optional textual description of the instance group manager.
* `fingerprint=kasd`
    - [Output only] Fingerprint of the instance group manager. This field is used for optimistic locking. An up-to-date fingerprint must be provided in order to modify the Instance Group Manager resource.
* `group=accusam`
    - [Output only] The full URL of the instance group created by the manager. This group contains all of the instances being managed, and cannot contain non-managed instances.
* `id=takimata`
    - [Output only] A server-assigned unique identifier for the resource.
* `instance-template=justo`
    - The full URL to an instance template from which all new instances will be created.
* `kind=amet.`
    - [Output only] The resource type. Always replicapool#instanceGroupManager.
* `name=erat`
    - The name of the instance group manager. Must be 1-63 characters long and comply with RFC1035. Supported characters include lowercase letters, numbers, and hyphens.
* `self-link=labore`
    - [Output only] The fully qualified URL for this resource.
* `target-pools=sea`
    - The full URL of all target pools to which new instances in the group are added. Updating the target pool values does not affect existing instances.
    - Each invocation of this argument appends the given value to the array.
* `target-size=11`
    - [Output only] The number of instances that the manager is attempting to maintain. Deleting or abandoning instances affects this number, as does resizing the group.


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
