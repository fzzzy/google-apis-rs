Creates a node pool for a cluster.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `container1 --scope <scope> projects locations-clusters-node-pools-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - The parent (project, location, cluster id) where the node pool will be
        created. Specified in the format
        &#39;projects/*/locations/*/clusters/*&#39;.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CreateNodePoolRequest:
  cluster-id: string
  node-pool:
    autoscaling:
      enabled: boolean
      max-node-count: integer
      min-node-count: integer
    config:
      disk-size-gb: integer
      disk-type: string
      image-type: string
      labels: { string: string }
      local-ssd-count: integer
      machine-type: string
      metadata: { string: string }
      min-cpu-platform: string
      oauth-scopes: [string]
      preemptible: boolean
      service-account: string
      tags: [string]
    initial-node-count: integer
    instance-group-urls: [string]
    management:
      auto-repair: boolean
      auto-upgrade: boolean
      upgrade-options:
        auto-upgrade-start-time: string
        description: string
    name: string
    self-link: string
    status: string
    status-message: string
    version: string
  parent: string
  project-id: string
  zone: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    cluster-id=consetetur`
    - Deprecated. The name of the cluster.
        This field has been deprecated and replaced by the parent field.
* `node-pool.autoscaling    enabled=false`
    - Is autoscaling enabled for this node pool.
* `max-node-count=82`
    - Maximum number of nodes in the NodePool. Must be &gt;= min_node_count. There
        has to enough quota to scale up the cluster.
* `min-node-count=19`
    - Minimum number of nodes in the NodePool. Must be &gt;= 1 and &lt;=
        max_node_count.

* `..config    disk-size-gb=96`
    - Size of the disk attached to each node, specified in GB.
        The smallest allowed disk size is 10GB.
        
        If unspecified, the default disk size is 100GB.
* `disk-type=et`
    - Type of the disk attached to each node (e.g. &#39;pd-standard&#39; or &#39;pd-ssd&#39;)
        
        If unspecified, the default disk type is &#39;pd-standard&#39;
* `image-type=clita`
    - The image type to use for this node. Note that for a given image type,
        the latest version of it will be used.
* `labels=key=consetetur`
    - The map of Kubernetes labels (key/value pairs) to be applied to each node.
        These will added in addition to any default label(s) that
        Kubernetes may apply to the node.
        In case of conflict in label keys, the applied set may differ depending on
        the Kubernetes version -- it&#39;s best to assume the behavior is undefined
        and conflicts should be avoided.
        For more information, including usage and the valid values, see:
        https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
    - the value will be associated with the given `key`
* `local-ssd-count=43`
    - The number of local SSD disks to be attached to the node.
        
        The limit for this value is dependant upon the maximum number of
        disks available on a machine per zone. See:
        https://cloud.google.com/compute/docs/disks/local-ssd#local_ssd_limits
        for more information.
* `machine-type=nonumy`
    - The name of a Google Compute Engine [machine
        type](/compute/docs/machine-types) (e.g.
        `n1-standard-1`).
        
        If unspecified, the default machine type is
        `n1-standard-1`.
* `metadata=key=kasd`
    - The metadata key/value pairs assigned to instances in the cluster.
        
        Keys must conform to the regexp [a-zA-Z0-9-_]+ and be less than 128 bytes
        in length. These are reflected as part of a URL in the metadata server.
        Additionally, to avoid ambiguity, keys must not conflict with any other
        metadata keys for the project or be one of the reserved keys:
         &#34;cluster-location&#34;
         &#34;cluster-name&#34;
         &#34;cluster-uid&#34;
         &#34;configure-sh&#34;
         &#34;enable-os-login&#34;
         &#34;gci-update-strategy&#34;
         &#34;gci-ensure-gke-docker&#34;
         &#34;instance-template&#34;
         &#34;kube-env&#34;
         &#34;startup-script&#34;
         &#34;user-data&#34;
        
        Values are free-form strings, and only have meaning as interpreted by
        the image running in the instance. The only restriction placed on them is
        that each value&#39;s size must be less than or equal to 32 KB.
        
        The total size of all keys and values must be less than 512 KB.
    - the value will be associated with the given `key`
* `min-cpu-platform=sanctus`
    - Minimum CPU platform to be used by this instance. The instance may be
        scheduled on the specified or newer CPU platform. Applicable values are the
        friendly names of CPU platforms, such as
        &lt;code&gt;minCpuPlatform: &amp;quot;Intel Haswell&amp;quot;&lt;/code&gt; or
        &lt;code&gt;minCpuPlatform: &amp;quot;Intel Sandy Bridge&amp;quot;&lt;/code&gt;. For more
        information, read [how to specify min CPU
        platform](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform)
* `oauth-scopes=takimata`
    - The set of Google API scopes to be made available on all of the
        node VMs under the &#34;default&#34; service account.
        
        The following scopes are recommended, but not required, and by default are
        not included:
        
        * `https://www.googleapis.com/auth/compute` is required for mounting
        persistent storage on your nodes.
        * `https://www.googleapis.com/auth/devstorage.read_only` is required for
        communicating with **gcr.io**
        (the [Google Container Registry](/container-registry/)).
        
        If unspecified, no scopes are added, unless Cloud Logging or Cloud
        Monitoring are enabled, in which case their required scopes will be added.
    - Each invocation of this argument appends the given value to the array.
* `preemptible=true`
    - Whether the nodes are created as preemptible VM instances. See:
        https://cloud.google.com/compute/docs/instances/preemptible for more
        information about preemptible VM instances.
* `service-account=labore`
    - The Google Cloud Platform Service Account to be used by the node VMs. If
        no Service Account is specified, the &#34;default&#34; service account is used.
* `tags=invidunt`
    - The list of instance tags applied to all nodes. Tags are used to identify
        valid sources or targets for network firewalls and are specified by
        the client during cluster or node pool creation. Each tag within the list
        must comply with RFC1035.
    - Each invocation of this argument appends the given value to the array.

* `..    initial-node-count=35`
    - The initial node count for the pool. You must ensure that your
        Compute Engine &lt;a href=&#34;/compute/docs/resource-quotas&#34;&gt;resource quota&lt;/a&gt;
        is sufficient for this number of instances. You must also have available
        firewall and routes quota.
* `instance-group-urls=sadipscing`
    - [Output only] The resource URLs of the [managed instance
        groups](/compute/docs/instance-groups/creating-groups-of-managed-instances)
        associated with this node pool.
    - Each invocation of this argument appends the given value to the array.
* `management    auto-repair=false`
    - A flag that specifies whether the node auto-repair is enabled for the node
        pool. If enabled, the nodes in this node pool will be monitored and, if
        they fail health checks too many times, an automatic repair action will be
        triggered.
* `auto-upgrade=true`
    - A flag that specifies whether node auto-upgrade is enabled for the node
        pool. If enabled, node auto-upgrade helps keep the nodes in your node pool
        up to date with the latest release version of Kubernetes.
* `upgrade-options    auto-upgrade-start-time=nonumy`
    - [Output only] This field is set when upgrades are about to commence
        with the approximate start time for the upgrades, in
        [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
* `description=sed`
    - [Output only] This field is set when upgrades are about to commence
        with the description of the upgrade.


* `...    name=aliquyam`
    - The name of the node pool.
* `self-link=sit`
    - [Output only] Server-defined URL for the resource.
* `status=eirmod`
    - [Output only] The status of the nodes in this pool instance.
* `status-message=consetetur`
    - [Output only] Additional information about the current status of this
        node pool instance, if available.
* `version=labore`
    - The version of the Kubernetes of this node.

* `..    parent=sed`
    - The parent (project, location, cluster id) where the node pool will be
        created. Specified in the format
        &#39;projects/*/locations/*/clusters/*&#39;.
* `project-id=ea`
    - Deprecated. The Google Developers Console [project ID or project
        number](https://developers.google.com/console/help/new/#projectnumber).
        This field has been deprecated and replaced by the parent field.
* `zone=gubergren`
    - Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the parent field.


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
