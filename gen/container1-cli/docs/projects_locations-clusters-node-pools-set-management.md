Sets the NodeManagement options for a node pool.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `container1 --scope <scope> projects locations-clusters-node-pools-set-management ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - The name (project, location, cluster, node pool id) of the node pool to set
        management properties. Specified in the format
        &#39;projects/*/locations/*/clusters/*/nodePools/*&#39;.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
SetNodePoolManagementRequest:
  cluster-id: string
  management:
    auto-repair: boolean
    auto-upgrade: boolean
    upgrade-options:
      auto-upgrade-start-time: string
      description: string
  name: string
  node-pool-id: string
  project-id: string
  zone: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    cluster-id=amet`
    - Deprecated. The name of the cluster to update.
        This field has been deprecated and replaced by the name field.
* `management    auto-repair=true`
    - A flag that specifies whether the node auto-repair is enabled for the node
        pool. If enabled, the nodes in this node pool will be monitored and, if
        they fail health checks too many times, an automatic repair action will be
        triggered.
* `auto-upgrade=true`
    - A flag that specifies whether node auto-upgrade is enabled for the node
        pool. If enabled, node auto-upgrade helps keep the nodes in your node pool
        up to date with the latest release version of Kubernetes.
* `upgrade-options    auto-upgrade-start-time=diam`
    - [Output only] This field is set when upgrades are about to commence
        with the approximate start time for the upgrades, in
        [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
* `description=justo`
    - [Output only] This field is set when upgrades are about to commence
        with the description of the upgrade.


* `...    name=est`
    - The name (project, location, cluster, node pool id) of the node pool to set
        management properties. Specified in the format
        &#39;projects/*/locations/*/clusters/*/nodePools/*&#39;.
* `node-pool-id=clita`
    - Deprecated. The name of the node pool to update.
        This field has been deprecated and replaced by the name field.
* `project-id=invidunt`
    - Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field.
* `zone=ut`
    - Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field.


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
