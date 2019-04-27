Update the entire content of the Autoscaler resource.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/compute* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/compute*.
You can set the scope for this method like this: `autoscaler1-beta2 --scope <scope> autoscalers update ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - Project ID of Autoscaler resource.
* **&lt;zone&gt;** *(string)*
    - Zone name of Autoscaler resource.
* **&lt;autoscaler&gt;** *(string)*
    - Name of the Autoscaler resource.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Autoscaler:
  autoscaling-policy:
    cool-down-period-sec: integer
    cpu-utilization:
      utilization-target: number
    load-balancing-utilization:
      utilization-target: number
    max-num-replicas: integer
    min-num-replicas: integer
  creation-timestamp: string
  description: string
  id: string
  kind: string
  name: string
  self-link: string
  target: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .autoscaling-policy    cool-down-period-sec=84`
    - The number of seconds that the Autoscaler should wait between two succeeding changes to the number of virtual machines. You should define an interval that is at least as long as the initialization time of a virtual machine and the time it may take for replica pool to create the virtual machine. The default is 60 seconds.
* `cpu-utilization    utilization-target=0.594456807662`
    - The target utilization that the Autoscaler should maintain. It is represented as a fraction of used cores. For example: 6 cores used in 8-core VM are represented here as 0.75. Must be a float value between (0, 1]. If not defined, the default is 0.8.

* `..load-balancing-utilization    utilization-target=0.460933679688`
    - Fraction of backend capacity utilization (set in HTTP load balancing configuration) that Autoscaler should maintain. Must be a positive float value. If not defined, the default is 0.8. For example if your maxRatePerInstance capacity (in HTTP Load Balancing configuration) is set at 10 and you would like to keep number of instances such that each instance receives 7 QPS on average, set this to 0.7.

* `..    max-num-replicas=96`
    - The maximum number of replicas that the Autoscaler can scale up to.
* `min-num-replicas=80`
    - The minimum number of replicas that the Autoscaler can scale down to.

* `..    creation-timestamp=duo`
    - [Output Only] Creation timestamp in RFC3339 text format.
* `description=aliquyam`
    - An optional textual description of the resource provided by the client.
* `id=sea`
    - [Output Only] Unique identifier for the resource; defined by the server.
* `kind=lorem`
    - Type of resource.
* `name=eos`
    - Name of the Autoscaler resource. Must be unique per project and zone.
* `self-link=erat`
    - [Output Only] A self-link to the Autoscaler configuration resource.
* `target=sadipscing`
    - URL to the entity which will be autoscaled. Currently the only supported value is ReplicaPool?s URL. Note: it is illegal to specify multiple Autoscalers for the same target.


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
