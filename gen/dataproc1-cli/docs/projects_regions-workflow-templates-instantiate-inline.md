Instantiates a template and begins execution.This method is equivalent to executing the sequence CreateWorkflowTemplate, InstantiateWorkflowTemplate, DeleteWorkflowTemplate.The returned Operation can be used to track execution of workflow by polling operations.get. The Operation will complete when entire workflow is finished.The running workflow can be aborted via operations.cancel. This will cause any inflight jobs to be cancelled and workflow-owned clusters to be deleted.The Operation.metadata will be WorkflowMetadata.On successful completion, Operation.response will be Empty.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dataproc1 --scope <scope> projects regions-workflow-templates-instantiate-inline ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The &#34;resource name&#34; of the workflow template region, as described in https://cloud.google.com/apis/design/resource_names of the form projects/{project_id}/regions/{region}
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
WorkflowTemplate:
  create-time: string
  id: string
  labels: { string: string }
  name: string
  placement:
    cluster-selector:
      cluster-labels: { string: string }
      zone: string
    managed-cluster:
      cluster-name: string
      config:
        config-bucket: string
        encryption-config:
          gce-pd-kms-key-name: string
        gce-cluster-config:
          internal-ip-only: boolean
          metadata: { string: string }
          network-uri: string
          service-account: string
          service-account-scopes: [string]
          subnetwork-uri: string
          tags: [string]
          zone-uri: string
        master-config:
          disk-config:
            boot-disk-size-gb: integer
            boot-disk-type: string
            num-local-ssds: integer
          image-uri: string
          instance-names: [string]
          is-preemptible: boolean
          machine-type-uri: string
          managed-group-config:
            instance-group-manager-name: string
            instance-template-name: string
          num-instances: integer
        secondary-worker-config:
          disk-config:
            boot-disk-size-gb: integer
            boot-disk-type: string
            num-local-ssds: integer
          image-uri: string
          instance-names: [string]
          is-preemptible: boolean
          machine-type-uri: string
          managed-group-config:
            instance-group-manager-name: string
            instance-template-name: string
          num-instances: integer
        software-config:
          image-version: string
          properties: { string: string }
        worker-config:
          disk-config:
            boot-disk-size-gb: integer
            boot-disk-type: string
            num-local-ssds: integer
          image-uri: string
          instance-names: [string]
          is-preemptible: boolean
          machine-type-uri: string
          managed-group-config:
            instance-group-manager-name: string
            instance-template-name: string
          num-instances: integer
      labels: { string: string }
  update-time: string
  version: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    create-time=dolore`
    - Output only. The time template was created.
* `id=lorem`
    - Required. The template id.The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters.
* `labels=key=invidunt`
    - Optional. The labels to associate with this template. These labels will be propagated to all jobs and clusters created by the workflow instance.Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).No more than 32 labels can be associated with a template.
    - the value will be associated with the given `key`
* `name=duo`
    - Output only. The &#34;resource name&#34; of the template, as described in https://cloud.google.com/apis/design/resource_names of the form projects/{project_id}/regions/{region}/workflowTemplates/{template_id}
* `placement.cluster-selector    cluster-labels=key=dolor`
    - Required. The cluster labels. Cluster must have all labels to match.
    - the value will be associated with the given `key`
* `zone=accusam`
    - Optional. The zone where workflow process executes. This parameter does not affect the selection of the cluster.If unspecified, the zone of the first cluster matching the selector is used.

* `..managed-cluster    cluster-name=diam`
    - Required. The cluster name prefix. A unique cluster name will be formed by appending a random suffix.The name must contain only lower-case letters (a-z), numbers (0-9), and hyphens (-). Must begin with a letter. Cannot begin or end with hyphen. Must consist of between 2 and 35 characters.
* `config    config-bucket=dolor`
    - Optional. A Cloud Storage staging bucket used for sharing generated SSH keys and config. If you do not specify a staging bucket, Cloud Dataproc will determine an appropriate Cloud Storage location (US, ASIA, or EU) for your cluster&#39;s staging bucket according to the Google Compute Engine zone where your cluster is deployed, and then it will create and manage this project-level, per-location bucket for you.
* `encryption-config    gce-pd-kms-key-name=eirmod`
    - Optional. The Cloud KMS key name to use for PD disk encryption for all instances in the cluster.

* `..gce-cluster-config    internal-ip-only=false`
    - Optional. If true, all instances in the cluster will only have internal IP addresses. By default, clusters are not restricted to internal IP addresses, and will have ephemeral external IP addresses assigned to each instance. This internal_ip_only restriction can only be enabled for subnetwork enabled networks, and all off-cluster dependencies must be configured to be accessible without external IP addresses.
* `metadata=key=ipsum`
    - The Compute Engine metadata entries to add to all instances (see Project and instance metadata (https://cloud.google.com/compute/docs/storing-retrieving-metadata#project_and_instance_metadata)).
    - the value will be associated with the given `key`
* `network-uri=voluptua.`
    - Optional. The Compute Engine network to be used for machine communications. Cannot be specified with subnetwork_uri. If neither network_uri nor subnetwork_uri is specified, the &#34;default&#34; network of the project is used, if it exists. Cannot be a &#34;Custom Subnet Network&#34; (see Using Subnetworks for more information).A full URL, partial URI, or short name are valid. Examples:
        https://www.googleapis.com/compute/v1/projects/[project_id]/regions/global/default
        projects/[project_id]/regions/global/default
        default
* `service-account=eirmod`
    - Optional. The service account of the instances. Defaults to the default Compute Engine service account. Custom service accounts need permissions equivalent to the following IAM roles:
        roles/logging.logWriter
        roles/storage.objectAdmin(see https://cloud.google.com/compute/docs/access/service-accounts#custom_service_accounts for more information). Example: [account_id]@[project_id].iam.gserviceaccount.com
* `service-account-scopes=sed`
    - Optional. The URIs of service account scopes to be included in Compute Engine instances. The following base set of scopes is always included:
        https://www.googleapis.com/auth/cloud.useraccounts.readonly
        https://www.googleapis.com/auth/devstorage.read_write
        https://www.googleapis.com/auth/logging.writeIf no scopes are specified, the following defaults are also provided:
        https://www.googleapis.com/auth/bigquery
        https://www.googleapis.com/auth/bigtable.admin.table
        https://www.googleapis.com/auth/bigtable.data
        https://www.googleapis.com/auth/devstorage.full_control
    - Each invocation of this argument appends the given value to the array.
* `subnetwork-uri=accusam`
    - Optional. The Compute Engine subnetwork to be used for machine communications. Cannot be specified with network_uri.A full URL, partial URI, or short name are valid. Examples:
        https://www.googleapis.com/compute/v1/projects/[project_id]/regions/us-east1/sub0
        projects/[project_id]/regions/us-east1/sub0
        sub0
* `tags=sanctus`
    - The Compute Engine tags to add to all instances (see Tagging instances).
    - Each invocation of this argument appends the given value to the array.
* `zone-uri=dolor`
    - Optional. The zone where the Compute Engine cluster will be located. On a create request, it is required in the &#34;global&#34; region. If omitted in a non-global Cloud Dataproc region, the service will pick a zone in the corresponding Compute Engine region. On a get request, zone will always be present.A full URL, partial URI, or short name are valid. Examples:
        https://www.googleapis.com/compute/v1/projects/[project_id]/zones/[zone]
        projects/[project_id]/zones/[zone]
        us-central1-f

* `..master-config.disk-config    boot-disk-size-gb=2`
    - Optional. Size in GB of the boot disk (default is 500GB).
* `boot-disk-type=dolore`
    - Optional. Type of the boot disk (default is &#34;pd-standard&#34;). Valid values: &#34;pd-ssd&#34; (Persistent Disk Solid State Drive) or &#34;pd-standard&#34; (Persistent Disk Hard Disk Drive).
* `num-local-ssds=31`
    - Optional. Number of attached SSDs, from 0 to 4 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and HDFS (https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries.

* `..    image-uri=ipsum`
    - Optional. The Compute Engine image resource used for cluster instances. It can be specified or may be inferred from SoftwareConfig.image_version.
* `instance-names=diam`
    - Output only. The list of instance names. Cloud Dataproc derives the names from cluster_name, num_instances, and the instance group.
    - Each invocation of this argument appends the given value to the array.
* `is-preemptible=true`
    - Optional. Specifies that this instance group contains preemptible instances.
* `machine-type-uri=et`
    - Optional. The Compute Engine machine type used for cluster instances.A full URL, partial URI, or short name are valid. Examples:
        https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2
        projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2
        n1-standard-2Auto Zone Exception: If you are using the Cloud Dataproc Auto Zone Placement feature, you must use the short name of the machine type resource, for example, n1-standard-2.
* `managed-group-config    instance-group-manager-name=tempor`
    - Output only. The name of the Instance Group Manager for this group.
* `instance-template-name=justo`
    - Output only. The name of the Instance Template used for the Managed Instance Group.

* `..    num-instances=93`
    - Optional. The number of VM instances in the instance group. For master instance groups, must be set to 1.

* `..secondary-worker-config.disk-config    boot-disk-size-gb=64`
    - Optional. Size in GB of the boot disk (default is 500GB).
* `boot-disk-type=amet`
    - Optional. Type of the boot disk (default is &#34;pd-standard&#34;). Valid values: &#34;pd-ssd&#34; (Persistent Disk Solid State Drive) or &#34;pd-standard&#34; (Persistent Disk Hard Disk Drive).
* `num-local-ssds=48`
    - Optional. Number of attached SSDs, from 0 to 4 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and HDFS (https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries.

* `..    image-uri=voluptua.`
    - Optional. The Compute Engine image resource used for cluster instances. It can be specified or may be inferred from SoftwareConfig.image_version.
* `instance-names=et`
    - Output only. The list of instance names. Cloud Dataproc derives the names from cluster_name, num_instances, and the instance group.
    - Each invocation of this argument appends the given value to the array.
* `is-preemptible=true`
    - Optional. Specifies that this instance group contains preemptible instances.
* `machine-type-uri=kasd`
    - Optional. The Compute Engine machine type used for cluster instances.A full URL, partial URI, or short name are valid. Examples:
        https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2
        projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2
        n1-standard-2Auto Zone Exception: If you are using the Cloud Dataproc Auto Zone Placement feature, you must use the short name of the machine type resource, for example, n1-standard-2.
* `managed-group-config    instance-group-manager-name=sit`
    - Output only. The name of the Instance Group Manager for this group.
* `instance-template-name=justo`
    - Output only. The name of the Instance Template used for the Managed Instance Group.

* `..    num-instances=40`
    - Optional. The number of VM instances in the instance group. For master instance groups, must be set to 1.

* `..software-config    image-version=ipsum`
    - Optional. The version of software inside the cluster. It must be one of the supported Cloud Dataproc Versions, such as &#34;1.2&#34; (including a subminor version, such as &#34;1.2.29&#34;), or the &#34;preview&#34; version. If unspecified, it defaults to the latest version.
* `properties=key=amet.`
    - Optional. The properties to set on daemon config files.Property keys are specified in prefix:property format, such as core:fs.defaultFS. The following are supported prefixes and their mappings:
        capacity-scheduler: capacity-scheduler.xml
        core: core-site.xml
        distcp: distcp-default.xml
        hdfs: hdfs-site.xml
        hive: hive-site.xml
        mapred: mapred-site.xml
        pig: pig.properties
        spark: spark-defaults.conf
        yarn: yarn-site.xmlFor more information, see Cluster properties.
    - the value will be associated with the given `key`

* `..worker-config.disk-config    boot-disk-size-gb=53`
    - Optional. Size in GB of the boot disk (default is 500GB).
* `boot-disk-type=sanctus`
    - Optional. Type of the boot disk (default is &#34;pd-standard&#34;). Valid values: &#34;pd-ssd&#34; (Persistent Disk Solid State Drive) or &#34;pd-standard&#34; (Persistent Disk Hard Disk Drive).
* `num-local-ssds=14`
    - Optional. Number of attached SSDs, from 0 to 4 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and HDFS (https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries.

* `..    image-uri=magna`
    - Optional. The Compute Engine image resource used for cluster instances. It can be specified or may be inferred from SoftwareConfig.image_version.
* `instance-names=amet.`
    - Output only. The list of instance names. Cloud Dataproc derives the names from cluster_name, num_instances, and the instance group.
    - Each invocation of this argument appends the given value to the array.
* `is-preemptible=true`
    - Optional. Specifies that this instance group contains preemptible instances.
* `machine-type-uri=ea`
    - Optional. The Compute Engine machine type used for cluster instances.A full URL, partial URI, or short name are valid. Examples:
        https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2
        projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2
        n1-standard-2Auto Zone Exception: If you are using the Cloud Dataproc Auto Zone Placement feature, you must use the short name of the machine type resource, for example, n1-standard-2.
* `managed-group-config    instance-group-manager-name=ea`
    - Output only. The name of the Instance Group Manager for this group.
* `instance-template-name=et`
    - Output only. The name of the Instance Template used for the Managed Instance Group.

* `..    num-instances=94`
    - Optional. The number of VM instances in the instance group. For master instance groups, must be set to 1.


* `...    labels=key=diam`
    - Optional. The labels to associate with this cluster.Label keys must be between 1 and 63 characters long, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62}Label values must be between 1 and 63 characters long, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}\p{N}_-{0,63}No more than 32 labels can be associated with a given cluster.
    - the value will be associated with the given `key`


* `...    update-time=eos`
    - Output only. The time template was last updated.
* `version=63`
    - Optional. Used to perform a consistent read-modify-write.This field should be left blank for a CreateWorkflowTemplate request. It is required for an UpdateWorkflowTemplate request, and must match the current server version. A typical update template flow would fetch the current template with a GetWorkflowTemplate request, which will return the current template with the version field filled in with the current server version. The user updates other fields in the template, then returns it as part of the UpdateWorkflowTemplate request.


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

* **-p request-id=string**
    - Optional. A tag that prevents multiple concurrent workflow instances with the same tag from running. This mitigates risk of concurrent instances started due to retries.It is recommended to always set this value to a UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier).The tag must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). The maximum length is 40 characters.

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
