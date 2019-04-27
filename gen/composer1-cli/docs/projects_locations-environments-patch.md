Update an environment.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `composer1 --scope <scope> projects locations-environments-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The relative resource name of the environment to update, in the form:
        &#34;projects/{projectId}/locations/{locationId}/environments/{environmentId}&#34;
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

* `-r .config    airflow-uri=justo`
    - Output only.
        The URI of the Apache Airflow Web UI hosted within this environment (see
        [Airflow web interface](/composer/docs/how-to/accessing/airflow-web-interface)).
* `dag-gcs-prefix=et`
    - Output only.
        The Cloud Storage prefix of the DAGs for this environment. Although Cloud
        Storage objects reside in a flat namespace, a hierarchical file tree
        can be simulated using &#34;/&#34;-delimited object name prefixes. DAG objects for
        this environment reside in a simulated directory with the given prefix.
* `gke-cluster=et`
    - Output only.
        The Kubernetes Engine cluster used to run this environment.
* `node-config    disk-size-gb=60`
    - Optional. The disk size in GB used for node VMs. Minimum size is 20GB.
        If unspecified, defaults to 100GB. Cannot be updated.
* `location=ipsum`
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
* `machine-type=lorem`
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
* `network=et`
    - Optional. The Compute Engine network to be used for machine
        communications, specified as a
        [relative resource name](/apis/design/resource_names#relative_resource_name).
        For example: &#34;projects/{projectId}/global/networks/{networkId}&#34;.
        
        [Shared VPC](/vpc/docs/shared-vpc) is not currently supported. The
        network must belong to the environment&#39;s project. If unspecified, the
        &#34;default&#34; network ID in the environment&#39;s project is used.  If a
        [Custom Subnet Network](/vpc/docs/vpc#vpc_networks_and_subnets)
        is provided, `nodeConfig.subnetwork` must also be provided.
* `oauth-scopes=duo`
    - Optional. The set of Google API scopes to be made available on all
        node VMs. If `oauth_scopes` is empty, defaults to
        [&#34;https://www.googleapis.com/auth/cloud-platform&#34;]. Cannot be updated.
    - Each invocation of this argument appends the given value to the array.
* `service-account=aliquyam`
    - Optional. The Google Cloud Platform Service Account to be used by the node
        VMs. If a service account is not specified, the &#34;default&#34; Compute Engine
        service account is used. Cannot be updated.
* `subnetwork=sea`
    - Optional. The Compute Engine subnetwork to be used for machine
        communications, specified as a
        [relative resource name](/apis/design/resource_names#relative_resource_name).
        For example:
        &#34;projects/{projectId}/regions/{regionId}/subnetworks/{subnetworkId}&#34;
        
        If a subnetwork is provided, `nodeConfig.network` must also be provided,
        and the subnetwork must belong to the enclosing environment&#39;s project and
        location.
* `tags=lorem`
    - Optional. The list of instance tags applied to all node VMs. Tags are used
        to identify valid sources or targets for network firewalls. Each tag within
        the list must comply with [RFC1035](https://www.ietf.org/rfc/rfc1035.txt).
        Cannot be updated.
    - Each invocation of this argument appends the given value to the array.

* `..    node-count=26`
    - The number of nodes in the Kubernetes Engine cluster that will be
        used to run this environment.
* `software-config    airflow-config-overrides=key=erat`
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
* `env-variables=key=sadipscing`
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
* `image-version=dolor`
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
* `pypi-packages=key=eirmod`
    - Optional. Custom Python Package Index (PyPI) packages to be installed in
        the environment.
        
        Keys refer to the lowercase package name such as &#34;numpy&#34;
        and values are the lowercase extras and version specifier such as
        &#34;==1.12.0&#34;, &#34;[devel,gcp_api]&#34;, or &#34;[devel]&gt;=1.8.2, &lt;1.9.2&#34;. To specify a
        package without pinning it to a version specifier, use the empty string as
        the value.
    - the value will be associated with the given `key`


* `...    create-time=elitr`
    - Output only.
        The time at which this environment was created.
* `labels=key=amet`
    - Optional. User-defined labels for this environment.
        The labels map can contain no more than 64 entries. Entries of the labels
        map are UTF8 strings that comply with the following restrictions:
        
        * Keys must conform to regexp: \p{Ll}\p{Lo}{0,62}
        * Values must conform to regexp:  [\p{Ll}\p{Lo}\p{N}_-]{0,63}
        * Both keys and values are additionally constrained to be &lt;= 128 bytes in
        size.
    - the value will be associated with the given `key`
* `name=no`
    - The resource name of the environment, in the form:
        &#34;projects/{projectId}/locations/{locationId}/environments/{environmentId}&#34;
* `state=labore`
    - The current state of the environment.
* `update-time=eirmod`
    - Output only.
        The time at which this environment was last modified.
* `uuid=dolore`
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
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p update-mask=string**
    - Required. A comma-separated list of paths, relative to `Environment`, of
        fields to update.
        For example, to set the version of scikit-learn to install in the
        environment to 0.19.0 and to remove an existing installation of
        numpy, the `updateMask` parameter would include the following two
        `paths` values: &#34;config.softwareConfig.pypiPackages.scikit-learn&#34; and
        &#34;config.softwareConfig.pypiPackages.numpy&#34;. The included patch
        environment would specify the scikit-learn version as follows:
        
            {
              &#34;config&#34;:{
                &#34;softwareConfig&#34;:{
                  &#34;pypiPackages&#34;:{
                    &#34;scikit-learn&#34;:&#34;==0.19.0&#34;
                  }
                }
              }
            }
        
        Note that in the above example, any existing PyPI packages
        other than scikit-learn and numpy will be unaffected.
        
        Only one update type may be included in a single request&#39;s `updateMask`.
        For example, one cannot update both the PyPI packages and
        labels in the same request. However, it is possible to update multiple
        members of a map field simultaneously in the same request. For example,
        to set the labels &#34;label1&#34; and &#34;label2&#34; while clearing &#34;label3&#34; (assuming
        it already exists), one can
        provide the paths &#34;labels.label1&#34;, &#34;labels.label2&#34;, and &#34;labels.label3&#34;
        and populate the patch environment as follows:
        
            {
              &#34;labels&#34;:{
                &#34;label1&#34;:&#34;new-label1-value&#34;
                &#34;label2&#34;:&#34;new-label2-value&#34;
              }
            }
        
        Note that in the above example, any existing labels that are not
        included in the `updateMask` will be unaffected.
        
        It is also possible to replace an entire map field by providing the
        map field&#39;s path in the `updateMask`. The new value of the field will
        be that which is provided in the patch environment. For example, to
        delete all pre-existing user-specified PyPI packages and
        install botocore at version 1.7.14, the `updateMask` would contain
        the path &#34;config.softwareConfig.pypiPackages&#34;, and
        the patch environment would be the following:
        
            {
              &#34;config&#34;:{
                &#34;softwareConfig&#34;:{
                  &#34;pypiPackages&#34;:{
                    &#34;botocore&#34;:&#34;==1.7.14&#34;
                  }
                }
              }
            }
        
        **Note:** Only the following fields can be updated:
        
         &lt;table&gt;
         &lt;tbody&gt;
         &lt;tr&gt;
         &lt;td&gt;&lt;strong&gt;Mask&lt;/strong&gt;&lt;/td&gt;
         &lt;td&gt;&lt;strong&gt;Purpose&lt;/strong&gt;&lt;/td&gt;
         &lt;/tr&gt;
         &lt;tr&gt;
         &lt;td&gt;config.softwareConfig.pypiPackages
         &lt;/td&gt;
         &lt;td&gt;Replace all custom custom PyPI packages. If a replacement
         package map is not included in `environment`, all custom
         PyPI packages are cleared. It is an error to provide both this mask and a
         mask specifying an individual package.&lt;/td&gt;
         &lt;/tr&gt;
         &lt;tr&gt;
         &lt;td&gt;config.softwareConfig.pypiPackages.&lt;var&gt;packagename&lt;/var&gt;&lt;/td&gt;
         &lt;td&gt;Update the custom PyPI package &lt;var&gt;packagename&lt;/var&gt;,
         preserving other packages. To delete the package, include it in
         `updateMask`, and omit the mapping for it in
         `environment.config.softwareConfig.pypiPackages`. It is an error
         to provide both a mask of this form and the
         &#34;config.softwareConfig.pypiPackages&#34; mask.&lt;/td&gt;
         &lt;/tr&gt;
         &lt;tr&gt;
         &lt;td&gt;labels&lt;/td&gt;
         &lt;td&gt;Replace all environment labels. If a replacement labels map is not
         included in `environment`, all labels are cleared. It is an error to
         provide both this mask and a mask specifying one or more individual
         labels.&lt;/td&gt;
         &lt;/tr&gt;
         &lt;tr&gt;
         &lt;td&gt;labels.&lt;var&gt;labelName&lt;/var&gt;&lt;/td&gt;
         &lt;td&gt;Set the label named &lt;var&gt;labelName&lt;/var&gt;, while preserving other
         labels. To delete the label, include it in `updateMask` and omit its
         mapping in `environment.labels`. It is an error to provide both a
         mask of this form and the &#34;labels&#34; mask.&lt;/td&gt;
         &lt;/tr&gt;
         &lt;tr&gt;
         &lt;td&gt;config.nodeCount&lt;/td&gt;
         &lt;td&gt;Horizontally scale the number of nodes in the environment. An integer
         greater than or equal to 3 must be provided in the `config.nodeCount` field.
         &lt;/td&gt;
         &lt;/tr&gt;
         &lt;tr&gt;
         &lt;td&gt;config.softwareConfig.airflowConfigOverrides&lt;/td&gt;
         &lt;td&gt;Replace all Apache Airflow config overrides. If a replacement config
         overrides map is not included in `environment`, all config overrides
         are cleared.
         It is an error to provide both this mask and a mask specifying one or
         more individual config overrides.&lt;/td&gt;
         &lt;/tr&gt;
         &lt;tr&gt;
         &lt;td&gt;config.softwareConfig.airflowConfigOverrides.&lt;var&gt;section&lt;/var&gt;-&lt;var&gt;name
         &lt;/var&gt;&lt;/td&gt;
         &lt;td&gt;Override the Apache Airflow config property &lt;var&gt;name&lt;/var&gt; in the
         section named &lt;var&gt;section&lt;/var&gt;, preserving other properties. To delete
         the property override, include it in `updateMask` and omit its mapping
         in `environment.config.softwareConfig.airflowConfigOverrides`.
         It is an error to provide both a mask of this form and the
         &#34;config.softwareConfig.airflowConfigOverrides&#34; mask.&lt;/td&gt;
         &lt;/tr&gt;
         &lt;tr&gt;
         &lt;td&gt;config.softwareConfig.envVariables&lt;/td&gt;
         &lt;td&gt;Replace all environment variables. If a replacement environment
         variable map is not included in `environment`, all custom environment
         variables  are cleared.
         It is an error to provide both this mask and a mask specifying one or
         more individual environment variables.&lt;/td&gt;
         &lt;/tr&gt;
         &lt;/tbody&gt;
         &lt;/table&gt;

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
