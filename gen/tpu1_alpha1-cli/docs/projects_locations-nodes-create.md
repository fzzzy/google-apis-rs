Creates a node.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `tpu1-alpha1 --scope <scope> projects locations-nodes-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - The parent resource name.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Node:
  accelerator-type: string
  cidr-block: string
  create-time: string
  description: string
  health: string
  health-description: string
  ip-address: string
  labels: { string: string }
  name: string
  network: string
  port: string
  scheduling-config:
    preemptible: boolean
  service-account: string
  state: string
  tensorflow-version: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    accelerator-type=eirmod`
    - The type of hardware accelerators associated with this node.
        Required.
* `cidr-block=sit`
    - The CIDR block that the TPU node will use when selecting an IP address.
        This CIDR block must be a /29 block; the Compute Engine networks API
        forbids a smaller block, and using a larger block would be wasteful (a
        node can only consume one IP address). Errors will occur if the CIDR block
        has already been used for a currently existing TPU node, the CIDR block
        conflicts with any subnetworks in the user&#39;s provided network, or the
        provided network is peered with another network that is using that CIDR
        block.
        Required.
* `create-time=stet`
    - Output only.
        The time when the node was created.
* `description=sed`
    - The user-supplied description of the TPU. Maximum of 512 characters.
* `health=et`
    - The health status of the TPU node.
* `health-description=dolores`
    - Output only.
        If this field is populated, it contains a description of why the TPU Node
        is unhealthy.
* `ip-address=kasd`
    - Output only.
        DEPRECATED! Use network_endpoints instead.
        The network address for the TPU Node as visible to Compute Engine
        instances.
* `labels=key=accusam`
    - Resource labels to represent user-provided metadata.
    - the value will be associated with the given `key`
* `name=takimata`
    - Output only.
        The immutable name of the TPU
* `network=justo`
    - The name of a network they wish to peer the TPU node to. It must be a
        preexisting Compute Engine network inside of the project on which this API
        has been activated. If none is provided, &#34;default&#34; will be used.
* `port=amet.`
    - Output only.
        DEPRECATED! Use network_endpoints instead.
        The network port for the TPU Node as visible to Compute Engine instances.
* `scheduling-config    preemptible=false`
    - No description provided.

* `..    service-account=labore`
    - Output only.
        The service account used to run the tensor flow services within the node.
        To share resources, including Google Cloud Storage data, with the
        Tensorflow job running in the Node, this account must have permissions to
        that data.
* `state=sea`
    - Output only.
        The current state for the TPU Node.
* `tensorflow-version=nonumy`
    - The version of Tensorflow running in the Node.
        Required.


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

* **-p node-id=string**
    - The unqualified resource name.

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
