Deploys new code and resource files to a new version.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `appengine1-beta5 --scope <scope> apps services-versions-create ...`
# Required Scalar Arguments
* **&lt;apps-id&gt;** *(string)*
    - Part of `name`. Name of the resource to update. For example: &#34;apps/myapp/services/default&#34;.
* **&lt;services-id&gt;** *(string)*
    - Part of `name`. See documentation of `appsId`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Version:
  api-config:
    auth-fail-action: string
    login: string
    script: string
    security-level: string
    url: string
  automatic-scaling:
    cool-down-period: string
    cpu-utilization:
      aggregation-window-length: string
      target-utilization: number
    disk-utilization:
      target-read-bytes-per-sec: integer
      target-read-ops-per-sec: integer
      target-write-bytes-per-sec: integer
      target-write-ops-per-sec: integer
    max-concurrent-requests: integer
    max-idle-instances: integer
    max-pending-latency: string
    max-total-instances: integer
    min-idle-instances: integer
    min-pending-latency: string
    min-total-instances: integer
    network-utilization:
      target-received-bytes-per-sec: integer
      target-received-packets-per-sec: integer
      target-sent-bytes-per-sec: integer
      target-sent-packets-per-sec: integer
    request-utilization:
      target-concurrent-requests: integer
      target-request-count-per-sec: integer
  basic-scaling:
    idle-timeout: string
    max-instances: integer
  beta-settings: { string: string }
  creation-time: string
  default-expiration: string
  deployer: string
  deployment:
    container:
      image: string
  disk-usage-bytes: string
  endpoints-api-service:
    config-id: string
    disable-trace-sampling: boolean
    name: string
    rollout-strategy: string
  env: string
  env-variables: { string: string }
  health-check:
    check-interval: string
    disable-health-check: boolean
    healthy-threshold: integer
    host: string
    restart-threshold: integer
    timeout: string
    unhealthy-threshold: integer
  id: string
  inbound-services: [string]
  instance-class: string
  manual-scaling:
    instances: integer
  name: string
  network:
    forwarded-ports: [string]
    instance-tag: string
    name: string
    subnetwork-name: string
  nobuild-files-regex: string
  resources:
    cpu: number
    disk-gb: number
    memory-gb: number
  runtime: string
  runtime-api-version: string
  serving-status: string
  threadsafe: boolean
  vm: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .api-config    auth-fail-action=et`
    - Action to take when users access resources that require authentication. Defaults to redirect.
* `login=duo`
    - Level of login required to access this resource. Defaults to optional.
* `script=aliquyam`
    - Path to the script from the application root directory.
* `security-level=sea`
    - Security (HTTPS) enforcement for this URL.
* `url=lorem`
    - URL to serve the endpoint at.

* `..automatic-scaling    cool-down-period=eos`
    - The time period that the Autoscaler (https://cloud.google.com/compute/docs/autoscaler/) should wait before it starts collecting information from a new instance. This prevents the autoscaler from collecting information when the instance is initializing, during which the collected usage would not be reliable. Only applicable in the App Engine flexible environment.
* `cpu-utilization    aggregation-window-length=erat`
    - Period of time over which CPU utilization is calculated.
* `target-utilization=0.0673002722736`
    - Target CPU utilization ratio to maintain when scaling. Must be between 0 and 1.

* `..disk-utilization    target-read-bytes-per-sec=53`
    - Target bytes read per second.
* `target-read-ops-per-sec=62`
    - Target ops read per second.
* `target-write-bytes-per-sec=58`
    - Target bytes written per second.
* `target-write-ops-per-sec=4`
    - Target ops written per second.

* `..    max-concurrent-requests=41`
    - Number of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.Defaults to a runtime-specific value.
* `max-idle-instances=65`
    - Maximum number of idle instances that should be maintained for this version.
* `max-pending-latency=eirmod`
    - Maximum amount of time that a request should wait in the pending queue before starting a new instance to handle it.
* `max-total-instances=68`
    - Maximum number of instances that should be started to handle requests.
* `min-idle-instances=64`
    - Minimum number of idle instances that should be maintained for this version. Only applicable for the default version of a module.
* `min-pending-latency=aliquyam`
    - Minimum amount of time a request should wait in the pending queue before starting a new instance to handle it.
* `min-total-instances=28`
    - Minimum number of instances that should be maintained for this version.
* `network-utilization    target-received-bytes-per-sec=45`
    - Target bytes received per second.
* `target-received-packets-per-sec=92`
    - Target packets received per second.
* `target-sent-bytes-per-sec=80`
    - Target bytes sent per second.
* `target-sent-packets-per-sec=31`
    - Target packets sent per second.

* `..request-utilization    target-concurrent-requests=80`
    - Target number of concurrent requests.
* `target-request-count-per-sec=61`
    - Target requests per second.


* `...basic-scaling    idle-timeout=sanctus`
    - Duration of time after the last request that an instance must wait before the instance is shut down.
* `max-instances=79`
    - Maximum number of instances to create for this version.

* `..    beta-settings=key=amet`
    - Metadata settings that are supplied to this version to enable beta runtime features.
    - the value will be associated with the given `key`
* `creation-time=et`
    - Time that this version was created.@OutputOnly
* `default-expiration=consetetur`
    - Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#staticfileshandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set.
* `deployer=ut`
    - Email address of the user who created this version.@OutputOnly
* `deployment.container    image=ea`
    - URI to the hosted container image in Google Container Registry. The URI must be fully qualified and include a tag or digest. Examples: &#34;gcr.io/my-project/image:tag&#34; or &#34;gcr.io/my-project/image@digest&#34;


* `...    disk-usage-bytes=sed`
    - Total size of version files hosted on App Engine disk in bytes.@OutputOnly
* `endpoints-api-service    config-id=dolor`
    - Endpoints service configuration id as specified by the Service Management API. For example &#34;2016-09-19r1&#34;By default, the Endpoints service configuration id is fixed and config_id must be specified. To keep the Endpoints service configuration id updated with each rollout, specify RolloutStrategy.MANAGED and omit config_id.
* `disable-trace-sampling=true`
    - Enable or disable trace sampling. By default, this is set to false for enabled.
* `name=dolor`
    - Endpoints service name which is the name of the &#34;service&#34; resource in the Service Management API. For example &#34;myapi.endpoints.myproject.cloud.goog&#34;
* `rollout-strategy=et`
    - Endpoints rollout strategy. If FIXED, config_id must be specified. If MANAGED, config_id must be omitted.

* `..    env=consetetur`
    - App Engine execution environment to use for this version.Defaults to 1.
* `env-variables=key=amet.`
    - Environment variables made available to the application.Only returned in GET requests if view=FULL is set.
    - the value will be associated with the given `key`
* `health-check    check-interval=voluptua.`
    - Interval between health checks.
* `disable-health-check=false`
    - Whether to explicitly disable health checks for this instance.
* `healthy-threshold=90`
    - Number of consecutive successful health checks required before receiving traffic.
* `host=justo`
    - Host header to send when performing an HTTP health check. Example: &#34;myapp.appspot.com&#34;
* `restart-threshold=49`
    - Number of consecutive failed health checks required before an instance is restarted.
* `timeout=vero`
    - Time before the health check is considered failed.
* `unhealthy-threshold=73`
    - Number of consecutive failed health checks required before removing traffic.

* `..    id=rebum.`
    - Relative name of the version within the module. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: &#34;default&#34;, &#34;latest&#34;, and any name with the prefix &#34;ah-&#34;.
* `inbound-services=consetetur`
    - Before an application can receive email or XMPP messages, the application must be configured to enable the service.
    - Each invocation of this argument appends the given value to the array.
* `instance-class=sadipscing`
    - Instance class that is used to run this version. Valid values are:
        AutomaticScaling: F1, F2, F4, F4_1G
        ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling.
* `manual-scaling    instances=25`
    - Number of instances to assign to the service at the start. This number can later be altered by using the Modules API (https://cloud.google.com/appengine/docs/python/modules/functions) set_num_instances() function.

* `..    name=sadipscing`
    - Full path to the Version resource in the API. Example: apps/myapp/services/default/versions/v1.@OutputOnly
* `network    forwarded-ports=invidunt`
    - List of ports, or port pairs, to forward from the virtual machine to the application container.
    - Each invocation of this argument appends the given value to the array.
* `instance-tag=consetetur`
    - Tag to apply to the VM instance during creation.
* `name=dolore`
    - Google Cloud Platform network where the virtual machines are created. Specify the short name, not the resource path.Defaults to default.
* `subnetwork-name=duo`
    - Google Cloud Platform sub-network where the virtual machines are created. Specify the short name, not the resource path.If a subnetwork name is specified, a network name will also be required unless it is for the default network.
        If the network the VM instance is being created in is a Legacy network, then the IP address is allocated from the IPv4Range.
        If the network the VM instance is being created in is an auto Subnet Mode Network, then only network name should be specified (not the subnetwork_name) and the IP address is created from the IPCidrRange of the subnetwork that exists in that zone for that network.
        If the network the VM instance is being created in is a custom Subnet Mode Network, then the subnetwork_name must be specified and the IP address is created from the IPCidrRange of the subnetwork.If specified, the subnetwork must exist in the same region as the Flex app.

* `..    nobuild-files-regex=aliquyam`
    - Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set.
* `resources    cpu=0.952666577253`
    - Number of CPU cores needed.
* `disk-gb=0.833525987365`
    - Disk size (GB) needed.
* `memory-gb=0.877582930428`
    - Memory (GB) needed.

* `..    runtime=consetetur`
    - Desired runtime. Example: python27.
* `runtime-api-version=takimata`
    - The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard/&lt;language&gt;/config/appref
* `serving-status=nonumy`
    - Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING.
* `threadsafe=true`
    - Whether multiple requests can be dispatched to this version at once.
* `vm=true`
    - Whether to deploy this version in a container on a virtual machine.


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
