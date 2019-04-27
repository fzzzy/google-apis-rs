Create a new environment.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `composer1 --scope <scope> projects locations-environments-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - The parent must be of the form &#34;projects/{projectId}/locations/{locationId}&#34;.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Environment:
  config:
    airflow-uri: string
    dag-gcs-prefix: string
    gke-cluster: string
    node-config:
      disk-size-gb: integer
      location: string
      machine-type: string
      network: string
      oauth-scopes: [string]
      service-account: string
      subnetwork: string
      tags: [string]
    node-count: integer
    software-config:
      airflow-config-overrides: { string: string }
      env-variables: { string: string }
      image-version: string
      pypi-packages: { string: string }
  create-time: string
  labels: { string: string }
  name: string
  state: string
  update-time: string
  uuid: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .config    airflow-uri=eirmod`
    - Output only.
        The URI of the Apache Airflow Web UI hosted within this environment (see
        [Airflow web interface](/composer/docs/how-to/accessing/airflow-web-interface)).
* `dag-gcs-prefix=sit`
    - Output only.
        The Cloud Storage prefix of the DAGs for this environment. Although Cloud
        Storage objects reside in a flat namespace, a hierarchical file tree
        can be simulated using &#34;/&#34;-delimited object name prefixes. DAG objects for
        this environment reside in a simulated directory with the given prefix.
* `gke-cluster=stet`
    - Output only.
        The Kubernetes Engine cluster used to run this environment.
* `node-config    disk-size-gb=59`
    - Optional. The disk size in GB used for node VMs. Minimum size is 20GB.
        If unspecified, defaults to 100GB. Cannot be updated.
* `location=et`
    - Optional. The Compute Engine [zone](/compute/docs/regions-zones) in which
        to deploy the VMs used to run the Apache Airflow software, specified as a
        [relative resource name](/apis/design/resource_names#relative_resource_name).
        For example: &#34;projects/{projectId}/zones/{zoneId}&#34;.
        
        This `location` must belong to the enclosing environment&#39;s project and
        location. If both this field and `nodeConfig.machineType` are specified,
        `nodeConfig.machineType` must belong to this `location`; if both are
        unspecified, the service will pick a zone in the Compute Engine region
        corresponding to the Cloud Composer location, and propagate that choice to
        both fields. If only one field (`location` or `nodeConfig.machineType`) is
        specified, the location information from the specified field will be
        propagated to the unspecified field.
* `machine-type=dolores`
    - Optional. The Compute Engine
        [machine type](/compute/docs/machine-types) used for cluster instances,
        specified as a
        [relative resource name](/apis/design/resource_names#relative_resource_name).
        For example:
        &#34;projects/{projectId}/zones/{zoneId}/machineTypes/{machineTypeId}&#34;.
        
        The `machineType` must belong to the enclosing environment&#39;s project and
        location. If both this field and `nodeConfig.location` are specified,
        this `machineType` must belong to the `nodeConfig.location`; if both are
        unspecified, the service will pick a zone in the Compute Engine region
        corresponding to the Cloud Composer location, and propagate that choice to
        both fields. If exactly one of this field and `nodeConfig.location` is
        specified, the location information from the specified field will be
        propagated to the unspecified field.
        
        If this field is unspecified, the `machineTypeId` defaults
        to &#34;n1-standard-1&#34;.
* `network=kasd`
    - Optional. The Compute Engine network to be used for machine
        communications, specified as a
        [relative resource name](/apis/design/resource_names#relative_resource_name).
        For example: &#34;projects/{projectId}/global/networks/{networkId}&#34;.
        
        [Shared VPC](/vpc/docs/shared-vpc) is not currently supported. The
        network must belong to the environment&#39;s project. If unspecified, the
        &#34;default&#34; network ID in the environment&#39;s project is used.  If a
        [Custom Subnet Network](/vpc/docs/vpc#vpc_networks_and_subnets)
        is provided, `nodeConfig.subnetwork` must also be provided.
* `oauth-scopes=accusam`
    - Optional. The set of Google API scopes to be made available on all
        node VMs. If `oauth_scopes` is empty, defaults to
        [&#34;https://www.googleapis.com/auth/cloud-platform&#34;]. Cannot be updated.
    - Each invocation of this argument appends the given value to the array.
* `service-account=takimata`
    - Optional. The Google Cloud Platform Service Account to be used by the node
        VMs. If a service account is not specified, the &#34;default&#34; Compute Engine
        service account is used. Cannot be updated.
* `subnetwork=justo`
    - Optional. The Compute Engine subnetwork to be used for machine
        communications, specified as a
        [relative resource name](/apis/design/resource_names#relative_resource_name).
        For example:
        &#34;projects/{projectId}/regions/{regionId}/subnetworks/{subnetworkId}&#34;
        
        If a subnetwork is provided, `nodeConfig.network` must also be provided,
        and the subnetwork must belong to the enclosing environment&#39;s project and
        location.
* `tags=amet.`
    - Optional. The list of instance tags applied to all node VMs. Tags are used
        to identify valid sources or targets for network firewalls. Each tag within
        the list must comply with [RFC1035](https://www.ietf.org/rfc/rfc1035.txt).
        Cannot be updated.
    - Each invocation of this argument appends the given value to the array.

* `..    node-count=20`
    - The number of nodes in the Kubernetes Engine cluster that will be
        used to run this environment.
* `software-config    airflow-config-overrides=key=labore`
    - Optional. Apache Airflow configuration properties to override.
        
        Property keys contain the section and property names, separated by a hyphen,
        for example &#34;core-dags_are_paused_at_creation&#34;. Section names must not
        contain hyphens (&#34;-&#34;), opening square brackets (&#34;[&#34;),  or closing square
        brackets (&#34;]&#34;). The property name must not be empty and must not contain
        an equals sign (&#34;=&#34;) or semicolon (&#34;;&#34;). Section and property names must
        not contain a period (&#34;.&#34;). Apache Airflow configuration property names
        must be written in [snake_case](https://en.wikipedia.org/wiki/Snake_case).
        Property values can contain any character, and can be written in any
        lower/upper case format.
        
        Certain Apache Airflow configuration property values are
        [blacklisted](/composer/docs/how-to/managing/setting-airflow-configurations#airflow_configuration_blacklists),
        and cannot be overridden.
    - the value will be associated with the given `key`
* `env-variables=key=sea`
    - Optional. Additional environment variables to provide to the Apache Airflow
        scheduler, worker, and webserver processes.
        
        Environment variable names must match the regular expression
        `a-zA-Z_*`. They cannot specify Apache Airflow
        software configuration overrides (they cannot match the regular expression
        `AIRFLOW__[A-Z0-9_]+__[A-Z0-9_]+`), and they cannot match any of the
        following reserved names:
        
        * `AIRFLOW_HOME`
        * `C_FORCE_ROOT`
        * `CONTAINER_NAME`
        * `DAGS_FOLDER`
        * `GCP_PROJECT`
        * `GCS_BUCKET`
        * `GKE_CLUSTER_NAME`
        * `SQL_DATABASE`
        * `SQL_INSTANCE`
        * `SQL_PASSWORD`
        * `SQL_PROJECT`
        * `SQL_REGION`
        * `SQL_USER`
    - the value will be associated with the given `key`
* `image-version=nonumy`
    - Output only.
        The version of the software running in the environment.
        This encapsulates both the version of Cloud Composer functionality and the
        version of Apache Airflow. It must match the regular expression
        `composer-[0-9]+\.[0-9]+(\.[0-9]+)?-airflow-[0-9]+\.[0-9]+(\.[0-9]+.*)?`.
        
        The Cloud Composer portion of the version is a
        [semantic version](https://semver.org). The portion of the image version
        following _airflow-_ is an official Apache Airflow repository
        [release name](https://github.com/apache/incubator-airflow/releases).
        
        See also [Release Notes](/composer/docs/release-notes).
* `pypi-packages=key=dolores`
    - Optional. Custom Python Package Index (PyPI) packages to be installed in
        the environment.
        
        Keys refer to the lowercase package name such as &#34;numpy&#34;
        and values are the lowercase extras and version specifier such as
        &#34;==1.12.0&#34;, &#34;[devel,gcp_api]&#34;, or &#34;[devel]&gt;=1.8.2, &lt;1.9.2&#34;. To specify a
        package without pinning it to a version specifier, use the empty string as
        the value.
    - the value will be associated with the given `key`


* `...    create-time=gubergren`
    - Output only.
        The time at which this environment was created.
* `labels=key=sadipscing`
    - Optional. User-defined labels for this environment.
        The labels map can contain no more than 64 entries. Entries of the labels
        map are UTF8 strings that comply with the following restrictions:
        
        * Keys must conform to regexp: \p{Ll}\p{Lo}{0,62}
        * Values must conform to regexp:  [\p{Ll}\p{Lo}\p{N}_-]{0,63}
        * Both keys and values are additionally constrained to be &lt;= 128 bytes in
        size.
    - the value will be associated with the given `key`
* `name=aliquyam`
    - The resource name of the environment, in the form:
        &#34;projects/{projectId}/locations/{locationId}/environments/{environmentId}&#34;
* `state=ea`
    - The current state of the environment.
* `update-time=no`
    - Output only.
        The time at which this environment was last modified.
* `uuid=justo`
    - Output only.
        The UUID (Universally Unique IDentifier) associated with this environment.
        This value is generated when the environment is created.


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
