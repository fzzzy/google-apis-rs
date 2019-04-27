Updates the metadata and configuration of a specific Redis instance.

Completed longrunning.Operation will contain the new instance object
in the response field. The returned operation is automatically deleted
after a few hours, so there is no need to call DeleteOperation.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `redis1 --scope <scope> projects locations-instances-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Required. Unique name of the resource in this scope including project and
        location using the form:
            `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
        
        Note: Redis instances are managed and addressed at regional level so
        location_id here refers to a GCP region; however, users may choose which
        specific zone (or collection of zones for cross-zone instances) an instance
        should be provisioned in. Refer to [location_id] and
        [alternative_location_id] fields for more details.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Instance:
  alternative-location-id: string
  authorized-network: string
  create-time: string
  current-location-id: string
  display-name: string
  host: string
  labels: { string: string }
  location-id: string
  memory-size-gb: integer
  name: string
  port: integer
  redis-configs: { string: string }
  redis-version: string
  reserved-ip-range: string
  state: string
  status-message: string
  tier: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    alternative-location-id=sadipscing`
    - Optional. Only applicable to STANDARD_HA tier which protects the instance
        against zonal failures by provisioning it across two zones. If provided, it
        must be a different zone from the one provided in [location_id].
* `authorized-network=aliquyam`
    - Optional. The full name of the Google Compute Engine
        [network](/compute/docs/networks-and-firewalls#networks) to which the
        instance is connected. If left unspecified, the `default` network
        will be used.
* `create-time=ea`
    - Output only. The time the instance was created.
* `current-location-id=no`
    - Output only. The current zone where the Redis endpoint is placed. For Basic
        Tier instances, this will always be the same as the [location_id]
        provided by the user at creation time. For Standard Tier instances,
        this can be either [location_id] or [alternative_location_id] and can
        change after a failover event.
* `display-name=justo`
    - An arbitrary and optional user-provided name for the instance.
* `host=justo`
    - Output only. Hostname or IP address of the exposed Redis endpoint used by
        clients to connect to the service.
* `labels=key=et`
    - Resource labels to represent user provided metadata
    - the value will be associated with the given `key`
* `location-id=et`
    - Optional. The zone where the instance will be provisioned. If not provided,
        the service will choose a zone for the instance. For STANDARD_HA tier,
        instances will be created across two zones for protection against zonal
        failures. If [alternative_location_id] is also provided, it must be
        different from [location_id].
* `memory-size-gb=60`
    - Required. Redis memory size in GiB.
* `name=ipsum`
    - Required. Unique name of the resource in this scope including project and
        location using the form:
            `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
        
        Note: Redis instances are managed and addressed at regional level so
        location_id here refers to a GCP region; however, users may choose which
        specific zone (or collection of zones for cross-zone instances) an instance
        should be provisioned in. Refer to [location_id] and
        [alternative_location_id] fields for more details.
* `port=96`
    - Output only. The port number of the exposed Redis endpoint.
* `redis-configs=key=et`
    - Optional. Redis configuration parameters, according to
        http://redis.io/topics/config. Currently, the only supported parameters
        are:
        
         *   maxmemory-policy
         *   notify-keyspace-events
    - the value will be associated with the given `key`
* `redis-version=duo`
    - Optional. The version of Redis software.
        If not provided, latest supported version will be used. Updating the
        version will perform an upgrade/downgrade to the new version. Currently,
        the supported values are `REDIS_3_2` for Redis 3.2.
* `reserved-ip-range=aliquyam`
    - Optional. The CIDR range of internal addresses that are reserved for this
        instance. If not provided, the service will choose an unused /29 block,
        for example, 10.0.0.0/29 or 192.168.0.0/29. Ranges must be unique
        and non-overlapping with existing subnets in an authorized network.
* `state=sea`
    - Output only. The current state of this instance.
* `status-message=lorem`
    - Output only. Additional information about the current status of this
        instance, if available.
* `tier=eos`
    - Required. The service tier of the instance.


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

* **-p update-mask=string**
    - Required. Mask of fields to update. At least one path must be supplied in
        this field. The elements of the repeated paths field may only include these
        fields from Instance:
        
         *   `displayName`
         *   `labels`
         *   `memorySizeGb`
         *   `redisConfig`

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
