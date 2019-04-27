Deploys code and resource files to a new version.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `appengine1 --scope <scope> apps services-versions-create ...`
# Required Scalar Arguments
* **&lt;apps-id&gt;** *(string)*
    - Part of `parent`. Name of the parent resource to create this version under. Example: apps/myapp/services/default.
* **&lt;services-id&gt;** *(string)*
    - Part of `parent`. See documentation of `appsId`.
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
      target-read-bytes-per-second: integer
      target-read-ops-per-second: integer
      target-write-bytes-per-second: integer
      target-write-ops-per-second: integer
    max-concurrent-requests: integer
    max-idle-instances: integer
    max-pending-latency: string
    max-total-instances: integer
    min-idle-instances: integer
    min-pending-latency: string
    min-total-instances: integer
    network-utilization:
      target-received-bytes-per-second: integer
      target-received-packets-per-second: integer
      target-sent-bytes-per-second: integer
      target-sent-packets-per-second: integer
    request-utilization:
      target-concurrent-requests: integer
      target-request-count-per-second: integer
    standard-scheduler-settings:
      max-instances: integer
      min-instances: integer
      target-cpu-utilization: number
      target-throughput-utilization: number
  basic-scaling:
    idle-timeout: string
    max-instances: integer
  beta-settings: { string: string }
  create-time: string
  created-by: string
  default-expiration: string
  deployment:
    cloud-build-options:
      app-yaml-path: string
      cloud-build-timeout: string
    container:
      image: string
    zip:
      files-count: integer
      source-url: string
  disk-usage-bytes: string
  endpoints-api-service:
    config-id: string
    disable-trace-sampling: boolean
    name: string
    rollout-strategy: string
  entrypoint:
    shell: string
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
  liveness-check:
    check-interval: string
    failure-threshold: integer
    host: string
    initial-delay: string
    path: string
    success-threshold: integer
    timeout: string
  manual-scaling:
    instances: integer
  name: string
  network:
    forwarded-ports: [string]
    instance-tag: string
    name: string
    subnetwork-name: string
  nobuild-files-regex: string
  readiness-check:
    app-start-timeout: string
    check-interval: string
    failure-threshold: integer
    host: string
    path: string
    success-threshold: integer
    timeout: string
  resources:
    cpu: number
    disk-gb: number
    memory-gb: number
  runtime: string
  runtime-api-version: string
  runtime-channel: string
  serving-status: string
  threadsafe: boolean
  version-url: string
  vm: boolean
  zones: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .api-config    auth-fail-action=rebum.`
    - Action to take when users access resources that require authentication. Defaults to redirect.
* `login=consetetur`
    - Level of login required to access this resource. Defaults to optional.
* `script=sadipscing`
    - Path to the script from the application root directory.
* `security-level=vero`
    - Security (HTTPS) enforcement for this URL.
* `url=sadipscing`
    - URL to serve the endpoint at.

* `..automatic-scaling    cool-down-period=invidunt`
    - The time period that the Autoscaler (https://cloud.google.com/compute/docs/autoscaler/) should wait before it starts collecting information from a new instance. This prevents the autoscaler from collecting information when the instance is initializing, during which the collected usage would not be reliable. Only applicable in the App Engine flexible environment.
* `cpu-utilization    aggregation-window-length=consetetur`
    - Period of time over which CPU utilization is calculated.
* `target-utilization=0.170115264625`
    - Target CPU utilization ratio to maintain when scaling. Must be between 0 and 1.

* `..disk-utilization    target-read-bytes-per-second=82`
    - Target bytes read per second.
* `target-read-ops-per-second=19`
    - Target ops read per seconds.
* `target-write-bytes-per-second=96`
    - Target bytes written per second.
* `target-write-ops-per-second=84`
    - Target ops written per second.

* `..    max-concurrent-requests=88`
    - Number of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.Defaults to a runtime-specific value.
* `max-idle-instances=56`
    - Maximum number of idle instances that should be maintained for this version.
* `max-pending-latency=takimata`
    - Maximum amount of time that a request should wait in the pending queue before starting a new instance to handle it.
* `max-total-instances=61`
    - Maximum number of instances that should be started to handle requests for this version.
* `min-idle-instances=88`
    - Minimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service.
* `min-pending-latency=sanctus`
    - Minimum amount of time a request should wait in the pending queue before starting a new instance to handle it.
* `min-total-instances=43`
    - Minimum number of running instances that should be maintained for this version.
* `network-utilization    target-received-bytes-per-second=74`
    - Target bytes received per second.
* `target-received-packets-per-second=15`
    - Target packets received per second.
* `target-sent-bytes-per-second=64`
    - Target bytes sent per second.
* `target-sent-packets-per-second=35`
    - Target packets sent per second.

* `..request-utilization    target-concurrent-requests=6`
    - Target number of concurrent requests.
* `target-request-count-per-second=35`
    - Target requests per second.

* `..standard-scheduler-settings    max-instances=68`
    - Maximum number of instances to run for this version. Set to zero to disable max_instances configuration.
* `min-instances=60`
    - Minimum number of instances to run for this version. Set to zero to disable min_instances configuration.
* `target-cpu-utilization=0.717377589745`
    - Target CPU utilization ratio to maintain when scaling.
* `target-throughput-utilization=0.191521742464`
    - Target throughput utilization ratio to maintain when scaling


* `...basic-scaling    idle-timeout=sit`
    - Duration of time after the last request that an instance must wait before the instance is shut down.
* `max-instances=61`
    - Maximum number of instances to create for this version.

* `..    beta-settings=key=consetetur`
    - Metadata settings that are supplied to this version to enable beta runtime features.
    - the value will be associated with the given `key`
* `create-time=labore`
    - Time that this version was created.@OutputOnly
* `created-by=sed`
    - Email address of the user who created this version.@OutputOnly
* `default-expiration=ea`
    - Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StaticFilesHandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set.
* `deployment.cloud-build-options    app-yaml-path=gubergren`
    - Path to the yaml file used in deployment, used to determine runtime configuration details.Required for flexible environment builds.See https://cloud.google.com/appengine/docs/standard/python/config/appref for more details.
* `cloud-build-timeout=aliquyam`
    - The Cloud Build timeout used as part of any dependent builds performed by version creation. Defaults to 10 minutes.

* `..container    image=eos`
    - URI to the hosted container image in Google Container Registry. The URI must be fully qualified and include a tag or digest. Examples: &#34;gcr.io/my-project/image:tag&#34; or &#34;gcr.io/my-project/image@digest&#34;

* `..zip    files-count=63`
    - An estimate of the number of files in a zip for a zip deployment. If set, must be greater than or equal to the actual number of files. Used for optimizing performance; if not provided, deployment may be slow.
* `source-url=sea`
    - URL of the zip file to deploy from. Must be a URL to a resource in Google Cloud Storage in the form &#39;http(s)://storage.googleapis.com/&lt;bucket&gt;/&lt;object&gt;&#39;.


* `...    disk-usage-bytes=labore`
    - Total size in bytes of all the files that are included in this version and currently hosted on the App Engine disk.@OutputOnly
* `endpoints-api-service    config-id=ipsum`
    - Endpoints service configuration ID as specified by the Service Management API. For example &#34;2016-09-19r1&#34;.By default, the rollout strategy for Endpoints is RolloutStrategy.FIXED. This means that Endpoints starts up with a particular configuration ID. When a new configuration is rolled out, Endpoints must be given the new configuration ID. The config_id field is used to give the configuration ID and is required in this case.Endpoints also has a rollout strategy called RolloutStrategy.MANAGED. When using this, Endpoints fetches the latest configuration and does not need the configuration ID. In this case, config_id must be omitted.
* `disable-trace-sampling=true`
    - Enable or disable trace sampling. By default, this is set to false for enabled.
* `name=dolores`
    - Endpoints service name which is the name of the &#34;service&#34; resource in the Service Management API. For example &#34;myapi.endpoints.myproject.cloud.goog&#34;
* `rollout-strategy=sit`
    - Endpoints rollout strategy. If FIXED, config_id must be specified. If MANAGED, config_id must be omitted.

* `..entrypoint    shell=diam`
    - The format should be a shell command that can be fed to bash -c.

* `..    env=ut`
    - App Engine execution environment for this version.Defaults to standard.
* `env-variables=key=justo`
    - Environment variables available to the application.Only returned in GET requests if view=FULL is set.
    - the value will be associated with the given `key`
* `health-check    check-interval=est`
    - Interval between health checks.
* `disable-health-check=true`
    - Whether to explicitly disable health checks for this instance.
* `healthy-threshold=78`
    - Number of consecutive successful health checks required before receiving traffic.
* `host=clita`
    - Host header to send when performing an HTTP health check. Example: &#34;myapp.appspot.com&#34;
* `restart-threshold=22`
    - Number of consecutive failed health checks required before an instance is restarted.
* `timeout=justo`
    - Time before the health check is considered failed.
* `unhealthy-threshold=44`
    - Number of consecutive failed health checks required before removing traffic.

* `..    id=clita`
    - Relative name of the version within the service. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: &#34;default&#34;, &#34;latest&#34;, and any name with the prefix &#34;ah-&#34;.
* `inbound-services=invidunt`
    - Before an application can receive email or XMPP messages, the application must be configured to enable the service.
    - Each invocation of this argument appends the given value to the array.
* `instance-class=ut`
    - Instance class that is used to run this version. Valid values are:
        AutomaticScaling: F1, F2, F4, F4_1G
        ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling.
* `liveness-check    check-interval=dolores`
    - Interval between health checks.
* `failure-threshold=77`
    - Number of consecutive failed checks required before considering the VM unhealthy.
* `host=voluptua.`
    - Host header to send when performing a HTTP Liveness check. Example: &#34;myapp.appspot.com&#34;
* `initial-delay=duo`
    - The initial delay before starting to execute the checks.
* `path=sed`
    - The request path.
* `success-threshold=70`
    - Number of consecutive successful checks required before considering the VM healthy.
* `timeout=ea`
    - Time before the check is considered failed.

* `..manual-scaling    instances=84`
    - Number of instances to assign to the service at the start. This number can later be altered by using the Modules API (https://cloud.google.com/appengine/docs/python/modules/functions) set_num_instances() function.

* `..    name=et`
    - Full path to the Version resource in the API. Example: apps/myapp/services/default/versions/v1.@OutputOnly
* `network    forwarded-ports=dolor`
    - List of ports, or port pairs, to forward from the virtual machine to the application container. Only applicable in the App Engine flexible environment.
    - Each invocation of this argument appends the given value to the array.
* `instance-tag=diam`
    - Tag to apply to the instance during creation. Only applicable in the App Engine flexible environment.
* `name=kasd`
    - Google Compute Engine network where the virtual machines are created. Specify the short name, not the resource path.Defaults to default.
* `subnetwork-name=invidunt`
    - Google Cloud Platform sub-network where the virtual machines are created. Specify the short name, not the resource path.If a subnetwork name is specified, a network name will also be required unless it is for the default network.
        If the network that the instance is being created in is a Legacy network, then the IP address is allocated from the IPv4Range.
        If the network that the instance is being created in is an auto Subnet Mode Network, then only network name should be specified (not the subnetwork_name) and the IP address is created from the IPCidrRange of the subnetwork that exists in that zone for that network.
        If the network that the instance is being created in is a custom Subnet Mode Network, then the subnetwork_name must be specified and the IP address is created from the IPCidrRange of the subnetwork.If specified, the subnetwork must exist in the same region as the App Engine flexible environment application.

* `..    nobuild-files-regex=rebum.`
    - Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set.
* `readiness-check    app-start-timeout=lorem`
    - A maximum time limit on application initialization, measured from moment the application successfully replies to a healthcheck until it is ready to serve traffic.
* `check-interval=clita`
    - Interval between health checks.
* `failure-threshold=64`
    - Number of consecutive failed checks required before removing traffic.
* `host=eirmod`
    - Host header to send when performing a HTTP Readiness check. Example: &#34;myapp.appspot.com&#34;
* `path=at`
    - The request path.
* `success-threshold=5`
    - Number of consecutive successful checks required before receiving traffic.
* `timeout=et`
    - Time before the check is considered failed.

* `..resources    cpu=0.212469304768`
    - Number of CPU cores needed.
* `disk-gb=0.534968289199`
    - Disk size (GB) needed.
* `memory-gb=0.928931283033`
    - Memory (GB) needed.

* `..    runtime=elitr`
    - Desired runtime. Example: python27.
* `runtime-api-version=nonumy`
    - The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard/&lt;language&gt;/config/appref
* `runtime-channel=rebum.`
    - The channel of the runtime to use. Only available for some runtimes. Defaults to the default channel.
* `serving-status=lorem`
    - Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING.
* `threadsafe=true`
    - Whether multiple requests can be dispatched to this version at once.
* `version-url=diam`
    - Serving URL for this version. Example: &#34;https://myversion-dot-myservice-dot-myapp.appspot.com&#34;@OutputOnly
* `vm=true`
    - Whether to deploy this version in a container on a virtual machine.
* `zones=ut`
    - The Google Compute Engine zones that are supported by this version in the App Engine flexible environment.
    - Each invocation of this argument appends the given value to the array.


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
