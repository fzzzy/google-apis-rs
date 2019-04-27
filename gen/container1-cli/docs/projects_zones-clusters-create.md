Creates a cluster, consisting of the specified number and type of Google
Compute Engine instances.

By default, the cluster is created in the project&#39;s
[default network](/compute/docs/networks-and-firewalls#networks).

One firewall is added for the cluster. After cluster creation,
the cluster creates routes for each node to allow the containers
on that node to communicate with all other instances in the
cluster.

Finally, an entry is added to the project&#39;s global metadata indicating
which CIDR range is being used by the cluster.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `container1 --scope <scope> projects zones-clusters-create ...`
# Required Scalar Arguments
* **&lt;project-id&gt;** *(string)*
    - Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the parent field.
* **&lt;zone&gt;** *(string)*
    - Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the parent field.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CreateClusterRequest:
  cluster:
    addons-config:
      horizontal-pod-autoscaling:
        disabled: boolean
      http-load-balancing:
        disabled: boolean
      kubernetes-dashboard:
        disabled: boolean
      network-policy-config:
        disabled: boolean
    cluster-ipv4-cidr: string
    create-time: string
    current-master-version: string
    current-node-count: integer
    current-node-version: string
    description: string
    enable-kubernetes-alpha: boolean
    endpoint: string
    expire-time: string
    initial-cluster-version: string
    initial-node-count: integer
    instance-group-urls: [string]
    ip-allocation-policy:
      cluster-ipv4-cidr: string
      cluster-ipv4-cidr-block: string
      cluster-secondary-range-name: string
      create-subnetwork: boolean
      node-ipv4-cidr: string
      node-ipv4-cidr-block: string
      services-ipv4-cidr: string
      services-ipv4-cidr-block: string
      services-secondary-range-name: string
      subnetwork-name: string
      use-ip-aliases: boolean
    label-fingerprint: string
    legacy-abac:
      enabled: boolean
    location: string
    locations: [string]
    logging-service: string
    maintenance-policy:
      window:
        daily-maintenance-window:
          duration: string
          start-time: string
    master-auth:
      client-certificate: string
      client-certificate-config:
        issue-client-certificate: boolean
      client-key: string
      cluster-ca-certificate: string
      password: string
      username: string
    master-authorized-networks-config:
      enabled: boolean
    monitoring-service: string
    name: string
    network: string
    network-config:
      network: string
      subnetwork: string
    network-policy:
      enabled: boolean
      provider: string
    node-config:
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
    node-ipv4-cidr-size: integer
    private-cluster-config:
      enable-private-endpoint: boolean
      enable-private-nodes: boolean
      master-ipv4-cidr-block: string
      private-endpoint: string
      public-endpoint: string
    resource-labels: { string: string }
    self-link: string
    services-ipv4-cidr: string
    status: string
    status-message: string
    subnetwork: string
    zone: string
  parent: string
  project-id: string
  zone: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .cluster.addons-config.horizontal-pod-autoscaling    disabled=true`
    - Whether the Horizontal Pod Autoscaling feature is enabled in the cluster.
        When enabled, it ensures that a Heapster pod is running in the cluster,
        which is also used by the Cloud Monitoring service.

* `..http-load-balancing    disabled=true`
    - Whether the HTTP Load Balancing controller is enabled in the cluster.
        When enabled, it runs a small pod in the cluster that manages the load
        balancers.

* `..kubernetes-dashboard    disabled=true`
    - Whether the Kubernetes Dashboard is enabled for this cluster.

* `..network-policy-config    disabled=false`
    - Whether NetworkPolicy is enabled for this cluster.


* `...    cluster-ipv4-cidr=diam`
    - The IP address range of the container pods in this cluster, in
        [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
        notation (e.g. `10.96.0.0/14`). Leave blank to have
        one automatically chosen or specify a `/14` block in `10.0.0.0/8`.
* `create-time=nonumy`
    - [Output only] The time the cluster was created, in
        [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
* `current-master-version=sed`
    - [Output only] The current software version of the master endpoint.
* `current-node-count=22`
    - [Output only] The number of nodes currently in the cluster.
* `current-node-version=magna`
    - [Output only] Deprecated, use
        [NodePool.version](/kubernetes-engine/docs/reference/rest/v1/projects.zones.clusters.nodePool)
        instead. The current version of the node software components. If they are
        currently at multiple versions because they&#39;re in the process of being
        upgraded, this reflects the minimum version of all nodes.
* `description=dolor`
    - An optional description of this cluster.
* `enable-kubernetes-alpha=true`
    - Kubernetes alpha features are enabled on this cluster. This includes alpha
        API groups (e.g. v1alpha1) and features that may not be production ready in
        the kubernetes version of the master and nodes.
        The cluster has no SLA for uptime and master/node upgrades are disabled.
        Alpha enabled clusters are automatically deleted thirty days after
        creation.
* `endpoint=dolor`
    - [Output only] The IP address of this cluster&#39;s master endpoint.
        The endpoint can be accessed from the internet at
        `https://username:password@endpoint/`.
        
        See the `masterAuth` property of this resource for username and
        password information.
* `expire-time=vero`
    - [Output only] The time the cluster will be automatically
        deleted in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
* `initial-cluster-version=nonumy`
    - The initial Kubernetes version for this cluster.  Valid versions are those
        found in validMasterVersions returned by getServerConfig.  The version can
        be upgraded over time; such upgrades are reflected in
        currentMasterVersion and currentNodeVersion.
        
        Users may specify either explicit versions offered by
        Kubernetes Engine or version aliases, which have the following behavior:
        
        - &#34;latest&#34;: picks the highest valid Kubernetes version
        - &#34;1.X&#34;: picks the highest valid patch+gke.N patch in the 1.X version
        - &#34;1.X.Y&#34;: picks the highest valid gke.N patch in the 1.X.Y version
        - &#34;1.X.Y-gke.N&#34;: picks an explicit Kubernetes version
        - &#34;&#34;,&#34;-&#34;: picks the default Kubernetes version
* `initial-node-count=93`
    - The number of nodes to create in this cluster. You must ensure that your
        Compute Engine &lt;a href=&#34;/compute/docs/resource-quotas&#34;&gt;resource quota&lt;/a&gt;
        is sufficient for this number of instances. You must also have available
        firewall and routes quota.
        For requests, this field should only be used in lieu of a
        &#34;node_pool&#34; object, since this configuration (along with the
        &#34;node_config&#34;) will be used to create a &#34;NodePool&#34; object with an
        auto-generated name. Do not use this and a node_pool at the same time.
* `instance-group-urls=dolores`
    - Deprecated. Use node_pools.instance_group_urls.
    - Each invocation of this argument appends the given value to the array.
* `ip-allocation-policy    cluster-ipv4-cidr=consetetur`
    - This field is deprecated, use cluster_ipv4_cidr_block.
* `cluster-ipv4-cidr-block=erat`
    - The IP address range for the cluster pod IPs. If this field is set, then
        `cluster.cluster_ipv4_cidr` must be left blank.
        
        This field is only applicable when `use_ip_aliases` is true.
        
        Set to blank to have a range chosen with the default size.
        
        Set to /netmask (e.g. `/14`) to have a range chosen with a specific
        netmask.
        
        Set to a
        [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
        notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
        `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
        to use.
* `cluster-secondary-range-name=amet.`
    - The name of the secondary range to be used for the cluster CIDR
        block.  The secondary range will be used for pod IP
        addresses. This must be an existing secondary range associated
        with the cluster subnetwork.
        
        This field is only applicable with use_ip_aliases is true and
        create_subnetwork is false.
* `create-subnetwork=true`
    - Whether a new subnetwork will be created automatically for the cluster.
        
        This field is only applicable when `use_ip_aliases` is true.
* `node-ipv4-cidr=dolores`
    - This field is deprecated, use node_ipv4_cidr_block.
* `node-ipv4-cidr-block=et`
    - The IP address range of the instance IPs in this cluster.
        
        This is applicable only if `create_subnetwork` is true.
        
        Set to blank to have a range chosen with the default size.
        
        Set to /netmask (e.g. `/14`) to have a range chosen with a specific
        netmask.
        
        Set to a
        [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
        notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
        `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
        to use.
* `services-ipv4-cidr=sed`
    - This field is deprecated, use services_ipv4_cidr_block.
* `services-ipv4-cidr-block=et`
    - The IP address range of the services IPs in this cluster. If blank, a range
        will be automatically chosen with the default size.
        
        This field is only applicable when `use_ip_aliases` is true.
        
        Set to blank to have a range chosen with the default size.
        
        Set to /netmask (e.g. `/14`) to have a range chosen with a specific
        netmask.
        
        Set to a
        [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
        notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
        `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
        to use.
* `services-secondary-range-name=aliquyam`
    - The name of the secondary range to be used as for the services
        CIDR block.  The secondary range will be used for service
        ClusterIPs. This must be an existing secondary range associated
        with the cluster subnetwork.
        
        This field is only applicable with use_ip_aliases is true and
        create_subnetwork is false.
* `subnetwork-name=nonumy`
    - A custom subnetwork name to be used if `create_subnetwork` is true.  If
        this field is empty, then an automatic name will be chosen for the new
        subnetwork.
* `use-ip-aliases=true`
    - Whether alias IPs will be used for pod IPs in the cluster.

* `..    label-fingerprint=aliquyam`
    - The fingerprint of the set of labels for this cluster.
* `legacy-abac    enabled=false`
    - Whether the ABAC authorizer is enabled for this cluster. When enabled,
        identities in the system, including service accounts, nodes, and
        controllers, will have statically granted permissions beyond those
        provided by the RBAC configuration or IAM.

* `..    location=magna`
    - [Output only] The name of the Google Compute Engine
        [zone](/compute/docs/regions-zones/regions-zones#available) or
        [region](/compute/docs/regions-zones/regions-zones#available) in which
        the cluster resides.
* `locations=gubergren`
    - The list of Google Compute Engine
        [locations](/compute/docs/zones#available) in which the cluster&#39;s nodes
        should be located.
    - Each invocation of this argument appends the given value to the array.
* `logging-service=sit`
    - The logging service the cluster should use to write logs.
        Currently available options:
        
        * `logging.googleapis.com` - the Google Cloud Logging service.
        * `none` - no logs will be exported from the cluster.
        * if left as an empty string,`logging.googleapis.com` will be used.
* `maintenance-policy.window.daily-maintenance-window    duration=gubergren`
    - [Output only] Duration of the time window, automatically chosen to be
        smallest possible in the given scenario.
        Duration will be in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt)
        format &#34;PTnHnMnS&#34;.
* `start-time=sit`
    - Time within the maintenance window to start the maintenance operations.
        Time format should be in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt)
        format &#34;HH:MM”, where HH : [00-23] and MM : [00-59] GMT.



* `....master-auth    client-certificate=amet`
    - [Output only] Base64-encoded public certificate used by clients to
        authenticate to the cluster endpoint.
* `client-certificate-config    issue-client-certificate=true`
    - Issue a client certificate.

* `..    client-key=sanctus`
    - [Output only] Base64-encoded private key used by clients to authenticate
        to the cluster endpoint.
* `cluster-ca-certificate=lorem`
    - [Output only] Base64-encoded public certificate that is the root of
        trust for the cluster.
* `password=amet.`
    - The password to use for HTTP basic authentication to the master endpoint.
        Because the master endpoint is open to the Internet, you should create a
        strong password.  If a password is provided for cluster creation, username
        must be non-empty.
* `username=diam`
    - The username to use for HTTP basic authentication to the master endpoint.
        For clusters v1.6.0 and later, you can disable basic authentication by
        providing an empty username.

* `..master-authorized-networks-config    enabled=true`
    - Whether or not master authorized networks is enabled.

* `..    monitoring-service=sadipscing`
    - The monitoring service the cluster should use to write metrics.
        Currently available options:
        
        * `monitoring.googleapis.com` - the Google Cloud Monitoring service.
        * `none` - no metrics will be exported from the cluster.
        * if left as an empty string, `monitoring.googleapis.com` will be used.
* `name=lorem`
    - The name of this cluster. The name must be unique within this project
        and zone, and can be up to 40 characters with the following restrictions:
        
        * Lowercase letters, numbers, and hyphens only.
        * Must start with a letter.
        * Must end with a number or a letter.
* `network=sed`
    - The name of the Google Compute Engine
        [network](/compute/docs/networks-and-firewalls#networks) to which the
        cluster is connected. If left unspecified, the `default` network
        will be used.
* `network-config    network=sit`
    - Output only. The relative name of the Google Compute Engine
        network(/compute/docs/networks-and-firewalls#networks) to which
        the cluster is connected.
        Example: projects/my-project/global/networks/my-network
* `subnetwork=dolore`
    - Output only. The relative name of the Google Compute Engine
        [subnetwork](/compute/docs/vpc) to which the cluster is connected.
        Example: projects/my-project/regions/us-central1/subnetworks/my-subnet

* `..network-policy    enabled=true`
    - Whether network policy is enabled on the cluster.
* `provider=at`
    - The selected network policy provider.

* `..node-config    disk-size-gb=3`
    - Size of the disk attached to each node, specified in GB.
        The smallest allowed disk size is 10GB.
        
        If unspecified, the default disk size is 100GB.
* `disk-type=ut`
    - Type of the disk attached to each node (e.g. &#39;pd-standard&#39; or &#39;pd-ssd&#39;)
        
        If unspecified, the default disk type is &#39;pd-standard&#39;
* `image-type=diam`
    - The image type to use for this node. Note that for a given image type,
        the latest version of it will be used.
* `labels=key=tempor`
    - The map of Kubernetes labels (key/value pairs) to be applied to each node.
        These will added in addition to any default label(s) that
        Kubernetes may apply to the node.
        In case of conflict in label keys, the applied set may differ depending on
        the Kubernetes version -- it&#39;s best to assume the behavior is undefined
        and conflicts should be avoided.
        For more information, including usage and the valid values, see:
        https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
    - the value will be associated with the given `key`
* `local-ssd-count=16`
    - The number of local SSD disks to be attached to the node.
        
        The limit for this value is dependant upon the maximum number of
        disks available on a machine per zone. See:
        https://cloud.google.com/compute/docs/disks/local-ssd#local_ssd_limits
        for more information.
* `machine-type=erat`
    - The name of a Google Compute Engine [machine
        type](/compute/docs/machine-types) (e.g.
        `n1-standard-1`).
        
        If unspecified, the default machine type is
        `n1-standard-1`.
* `metadata=key=dolores`
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
* `min-cpu-platform=kasd`
    - Minimum CPU platform to be used by this instance. The instance may be
        scheduled on the specified or newer CPU platform. Applicable values are the
        friendly names of CPU platforms, such as
        &lt;code&gt;minCpuPlatform: &amp;quot;Intel Haswell&amp;quot;&lt;/code&gt; or
        &lt;code&gt;minCpuPlatform: &amp;quot;Intel Sandy Bridge&amp;quot;&lt;/code&gt;. For more
        information, read [how to specify min CPU
        platform](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform)
* `oauth-scopes=et`
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
* `preemptible=false`
    - Whether the nodes are created as preemptible VM instances. See:
        https://cloud.google.com/compute/docs/instances/preemptible for more
        information about preemptible VM instances.
* `service-account=sed`
    - The Google Cloud Platform Service Account to be used by the node VMs. If
        no Service Account is specified, the &#34;default&#34; service account is used.
* `tags=dolores`
    - The list of instance tags applied to all nodes. Tags are used to identify
        valid sources or targets for network firewalls and are specified by
        the client during cluster or node pool creation. Each tag within the list
        must comply with RFC1035.
    - Each invocation of this argument appends the given value to the array.

* `..    node-ipv4-cidr-size=88`
    - [Output only] The size of the address space on each node for hosting
        containers. This is provisioned from within the `container_ipv4_cidr`
        range.
* `private-cluster-config    enable-private-endpoint=false`
    - Whether the master&#39;s internal IP address is used as the cluster endpoint.
* `enable-private-nodes=true`
    - Whether nodes have internal IP addresses only. If enabled, all nodes are
        given only RFC 1918 private addresses and communicate with the master via
        private networking.
* `master-ipv4-cidr-block=sed`
    - The IP range in CIDR notation to use for the hosted master network. This
        range will be used for assigning internal IP addresses to the master or
        set of masters, as well as the ILB VIP. This range must not overlap with
        any other ranges in use within the cluster&#39;s network.
* `private-endpoint=takimata`
    - Output only. The internal IP address of this cluster&#39;s master endpoint.
* `public-endpoint=sit`
    - Output only. The external IP address of this cluster&#39;s master endpoint.

* `..    resource-labels=key=labore`
    - The resource labels for the cluster to use to annotate any related
        Google Compute Engine resources.
    - the value will be associated with the given `key`
* `self-link=nonumy`
    - [Output only] Server-defined URL for the resource.
* `services-ipv4-cidr=erat`
    - [Output only] The IP address range of the Kubernetes services in
        this cluster, in
        [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
        notation (e.g. `1.2.3.4/29`). Service addresses are
        typically put in the last `/16` from the container CIDR.
* `status=gubergren`
    - [Output only] The current status of this cluster.
* `status-message=erat`
    - [Output only] Additional information about the current status of this
        cluster, if available.
* `subnetwork=et`
    - The name of the Google Compute Engine
        [subnetwork](/compute/docs/subnetworks) to which the
        cluster is connected.
* `zone=amet`
    - [Output only] The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field is deprecated, use location instead.

* `..    parent=lorem`
    - The parent (project and location) where the cluster will be created.
        Specified in the format &#39;projects/*/locations/*&#39;.
* `project-id=voluptua.`
    - Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the parent field.
* `zone=rebum.`
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
