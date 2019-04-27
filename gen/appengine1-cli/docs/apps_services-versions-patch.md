Updates the specified Version resource. You can specify the following fields depending on the App Engine environment and type of scaling that the version resource uses:Standard environment
instance_class (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.instance_class)automatic scaling in the standard environment:
automatic_scaling.min_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
automatic_scaling.max_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
automaticScaling.standard_scheduler_settings.max_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)
automaticScaling.standard_scheduler_settings.min_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)
automaticScaling.standard_scheduler_settings.target_cpu_utilization (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)
automaticScaling.standard_scheduler_settings.target_throughput_utilization (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)basic scaling or manual scaling in the standard environment:
serving_status (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.serving_status)Flexible environment
serving_status (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.serving_status)automatic scaling in the flexible environment:
automatic_scaling.min_total_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
automatic_scaling.max_total_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
automatic_scaling.cool_down_period_sec (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
automatic_scaling.cpu_utilization.target_utilization (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `appengine1 --scope <scope> apps services-versions-patch ...`
# Required Scalar Arguments
* **&lt;apps-id&gt;** *(string)*
    - Part of `name`. Name of the resource to update. Example: apps/myapp/services/default/versions/1.
* **&lt;services-id&gt;** *(string)*
    - Part of `name`. See documentation of `appsId`.
* **&lt;versions-id&gt;** *(string)*
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

* `-r .api-config    auth-fail-action=ipsum`
    - Action to take when users access resources that require authentication. Defaults to redirect.
* `login=ut`
    - Level of login required to access this resource. Defaults to optional.
* `script=dolor`
    - Path to the script from the application root directory.
* `security-level=sea`
    - Security (HTTPS) enforcement for this URL.
* `url=ut`
    - URL to serve the endpoint at.

* `..automatic-scaling    cool-down-period=eirmod`
    - The time period that the Autoscaler (https://cloud.google.com/compute/docs/autoscaler/) should wait before it starts collecting information from a new instance. This prevents the autoscaler from collecting information when the instance is initializing, during which the collected usage would not be reliable. Only applicable in the App Engine flexible environment.
* `cpu-utilization    aggregation-window-length=sanctus`
    - Period of time over which CPU utilization is calculated.
* `target-utilization=0.735189875377`
    - Target CPU utilization ratio to maintain when scaling. Must be between 0 and 1.

* `..disk-utilization    target-read-bytes-per-second=2`
    - Target bytes read per second.
* `target-read-ops-per-second=79`
    - Target ops read per seconds.
* `target-write-bytes-per-second=16`
    - Target bytes written per second.
* `target-write-ops-per-second=75`
    - Target ops written per second.

* `..    max-concurrent-requests=65`
    - Number of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.Defaults to a runtime-specific value.
* `max-idle-instances=71`
    - Maximum number of idle instances that should be maintained for this version.
* `max-pending-latency=et`
    - Maximum amount of time that a request should wait in the pending queue before starting a new instance to handle it.
* `max-total-instances=46`
    - Maximum number of instances that should be started to handle requests for this version.
* `min-idle-instances=81`
    - Minimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service.
* `min-pending-latency=dolore`
    - Minimum amount of time a request should wait in the pending queue before starting a new instance to handle it.
* `min-total-instances=75`
    - Minimum number of running instances that should be maintained for this version.
* `network-utilization    target-received-bytes-per-second=2`
    - Target bytes received per second.
* `target-received-packets-per-second=43`
    - Target packets received per second.
* `target-sent-bytes-per-second=80`
    - Target bytes sent per second.
* `target-sent-packets-per-second=10`
    - Target packets sent per second.

* `..request-utilization    target-concurrent-requests=17`
    - Target number of concurrent requests.
* `target-request-count-per-second=72`
    - Target requests per second.

* `..standard-scheduler-settings    max-instances=91`
    - Maximum number of instances to run for this version. Set to zero to disable max_instances configuration.
* `min-instances=63`
    - Minimum number of instances to run for this version. Set to zero to disable min_instances configuration.
* `target-cpu-utilization=0.858884522272`
    - Target CPU utilization ratio to maintain when scaling.
* `target-throughput-utilization=0.156922986241`
    - Target throughput utilization ratio to maintain when scaling


* `...basic-scaling    idle-timeout=aliquyam`
    - Duration of time after the last request that an instance must wait before the instance is shut down.
* `max-instances=7`
    - Maximum number of instances to create for this version.

* `..    beta-settings=key=consetetur`
    - Metadata settings that are supplied to this version to enable beta runtime features.
    - the value will be associated with the given `key`
* `create-time=sea`
    - Time that this version was created.@OutputOnly
* `created-by=elitr`
    - Email address of the user who created this version.@OutputOnly
* `default-expiration=at`
    - Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StaticFilesHandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set.
* `deployment.cloud-build-options    app-yaml-path=sea`
    - Path to the yaml file used in deployment, used to determine runtime configuration details.Required for flexible environment builds.See https://cloud.google.com/appengine/docs/standard/python/config/appref for more details.
* `cloud-build-timeout=consetetur`
    - The Cloud Build timeout used as part of any dependent builds performed by version creation. Defaults to 10 minutes.

* `..container    image=diam`
    - URI to the hosted container image in Google Container Registry. The URI must be fully qualified and include a tag or digest. Examples: &#34;gcr.io/my-project/image:tag&#34; or &#34;gcr.io/my-project/image@digest&#34;

* `..zip    files-count=79`
    - An estimate of the number of files in a zip for a zip deployment. If set, must be greater than or equal to the actual number of files. Used for optimizing performance; if not provided, deployment may be slow.
* `source-url=dolores`
    - URL of the zip file to deploy from. Must be a URL to a resource in Google Cloud Storage in the form &#39;http(s)://storage.googleapis.com/&lt;bucket&gt;/&lt;object&gt;&#39;.


* `...    disk-usage-bytes=consetetur`
    - Total size in bytes of all the files that are included in this version and currently hosted on the App Engine disk.@OutputOnly
* `endpoints-api-service    config-id=dolor`
    - Endpoints service configuration ID as specified by the Service Management API. For example &#34;2016-09-19r1&#34;.By default, the rollout strategy for Endpoints is RolloutStrategy.FIXED. This means that Endpoints starts up with a particular configuration ID. When a new configuration is rolled out, Endpoints must be given the new configuration ID. The config_id field is used to give the configuration ID and is required in this case.Endpoints also has a rollout strategy called RolloutStrategy.MANAGED. When using this, Endpoints fetches the latest configuration and does not need the configuration ID. In this case, config_id must be omitted.
* `disable-trace-sampling=false`
    - Enable or disable trace sampling. By default, this is set to false for enabled.
* `name=elitr`
    - Endpoints service name which is the name of the &#34;service&#34; resource in the Service Management API. For example &#34;myapi.endpoints.myproject.cloud.goog&#34;
* `rollout-strategy=ea`
    - Endpoints rollout strategy. If FIXED, config_id must be specified. If MANAGED, config_id must be omitted.

* `..entrypoint    shell=et`
    - The format should be a shell command that can be fed to bash -c.

* `..    env=stet`
    - App Engine execution environment for this version.Defaults to standard.
* `env-variables=key=sed`
    - Environment variables available to the application.Only returned in GET requests if view=FULL is set.
    - the value will be associated with the given `key`
* `health-check    check-interval=dolor`
    - Interval between health checks.
* `disable-health-check=true`
    - Whether to explicitly disable health checks for this instance.
* `healthy-threshold=68`
    - Number of consecutive successful health checks required before receiving traffic.
* `host=lorem`
    - Host header to send when performing an HTTP health check. Example: &#34;myapp.appspot.com&#34;
* `restart-threshold=5`
    - Number of consecutive failed health checks required before an instance is restarted.
* `timeout=consetetur`
    - Time before the health check is considered failed.
* `unhealthy-threshold=11`
    - Number of consecutive failed health checks required before removing traffic.

* `..    id=labore`
    - Relative name of the version within the service. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: &#34;default&#34;, &#34;latest&#34;, and any name with the prefix &#34;ah-&#34;.
* `inbound-services=gubergren`
    - Before an application can receive email or XMPP messages, the application must be configured to enable the service.
    - Each invocation of this argument appends the given value to the array.
* `instance-class=et`
    - Instance class that is used to run this version. Valid values are:
        AutomaticScaling: F1, F2, F4, F4_1G
        ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling.
* `liveness-check    check-interval=sadipscing`
    - Interval between health checks.
* `failure-threshold=28`
    - Number of consecutive failed checks required before considering the VM unhealthy.
* `host=magna`
    - Host header to send when performing a HTTP Liveness check. Example: &#34;myapp.appspot.com&#34;
* `initial-delay=lorem`
    - The initial delay before starting to execute the checks.
* `path=rebum.`
    - The request path.
* `success-threshold=30`
    - Number of consecutive successful checks required before considering the VM healthy.
* `timeout=clita`
    - Time before the check is considered failed.

* `..manual-scaling    instances=77`
    - Number of instances to assign to the service at the start. This number can later be altered by using the Modules API (https://cloud.google.com/appengine/docs/python/modules/functions) set_num_instances() function.

* `..    name=dolores`
    - Full path to the Version resource in the API. Example: apps/myapp/services/default/versions/v1.@OutputOnly
* `network    forwarded-ports=vero`
    - List of ports, or port pairs, to forward from the virtual machine to the application container. Only applicable in the App Engine flexible environment.
    - Each invocation of this argument appends the given value to the array.
* `instance-tag=consetetur`
    - Tag to apply to the instance during creation. Only applicable in the App Engine flexible environment.
* `name=vero`
    - Google Compute Engine network where the virtual machines are created. Specify the short name, not the resource path.Defaults to default.
* `subnetwork-name=consetetur`
    - Google Cloud Platform sub-network where the virtual machines are created. Specify the short name, not the resource path.If a subnetwork name is specified, a network name will also be required unless it is for the default network.
        If the network that the instance is being created in is a Legacy network, then the IP address is allocated from the IPv4Range.
        If the network that the instance is being created in is an auto Subnet Mode Network, then only network name should be specified (not the subnetwork_name) and the IP address is created from the IPCidrRange of the subnetwork that exists in that zone for that network.
        If the network that the instance is being created in is a custom Subnet Mode Network, then the subnetwork_name must be specified and the IP address is created from the IPCidrRange of the subnetwork.If specified, the subnetwork must exist in the same region as the App Engine flexible environment application.

* `..    nobuild-files-regex=eos`
    - Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set.
* `readiness-check    app-start-timeout=justo`
    - A maximum time limit on application initialization, measured from moment the application successfully replies to a healthcheck until it is ready to serve traffic.
* `check-interval=tempor`
    - Interval between health checks.
* `failure-threshold=40`
    - Number of consecutive failed checks required before removing traffic.
* `host=dolore`
    - Host header to send when performing a HTTP Readiness check. Example: &#34;myapp.appspot.com&#34;
* `path=amet.`
    - The request path.
* `success-threshold=68`
    - Number of consecutive successful checks required before receiving traffic.
* `timeout=magna`
    - Time before the check is considered failed.

* `..resources    cpu=0.073479543813`
    - Number of CPU cores needed.
* `disk-gb=0.189075671793`
    - Disk size (GB) needed.
* `memory-gb=0.966361549129`
    - Memory (GB) needed.

* `..    runtime=invidunt`
    - Desired runtime. Example: python27.
* `runtime-api-version=accusam`
    - The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard/&lt;language&gt;/config/appref
* `runtime-channel=labore`
    - The channel of the runtime to use. Only available for some runtimes. Defaults to the default channel.
* `serving-status=diam`
    - Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING.
* `threadsafe=true`
    - Whether multiple requests can be dispatched to this version at once.
* `version-url=sed`
    - Serving URL for this version. Example: &#34;https://myversion-dot-myservice-dot-myapp.appspot.com&#34;@OutputOnly
* `vm=false`
    - Whether to deploy this version in a container on a virtual machine.
* `zones=magna`
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
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p update-mask=string**
    - Standard field mask for the set of fields to be updated.

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
