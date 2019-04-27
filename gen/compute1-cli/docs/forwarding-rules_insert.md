Creates a ForwardingRule resource in the specified project and region using the data included in the request.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/compute*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `compute1 --scope <scope> forwarding-rules insert ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - Project ID for this request.
* **&lt;region&gt;** *(string)*
    - Name of the region scoping this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ForwardingRule:
  ip-address: string
  ip-protocol: string
  backend-service: string
  creation-timestamp: string
  description: string
  id: string
  ip-version: string
  kind: string
  load-balancing-scheme: string
  name: string
  network: string
  network-tier: string
  port-range: string
  ports: [string]
  region: string
  self-link: string
  subnetwork: string
  target: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    ip-address=dolore`
    - The IP address that this forwarding rule is serving on behalf of.
        
        Addresses are restricted based on the forwarding rule&#39;s load balancing scheme (EXTERNAL or INTERNAL) and scope (global or regional).
        
        When the load balancing scheme is EXTERNAL, for global forwarding rules, the address must be a global IP, and for regional forwarding rules, the address must live in the same region as the forwarding rule. If this field is empty, an ephemeral IPv4 address from the same scope (global or regional) will be assigned. A regional forwarding rule supports IPv4 only. A global forwarding rule supports either IPv4 or IPv6.
        
        When the load balancing scheme is INTERNAL_SELF_MANAGED, this must be a URL reference to an existing Address resource ( internal regional static IP address), with a purpose of GCE_END_POINT and address_type of INTERNAL.
        
        When the load balancing scheme is INTERNAL, this can only be an RFC 1918 IP address belonging to the network/subnet configured for the forwarding rule. By default, if this field is empty, an ephemeral internal IP address will be automatically allocated from the IP range of the subnet or network configured for this forwarding rule.
        
        An address can be specified either by a literal IP address or a URL reference to an existing Address resource. The following examples are all valid:  
        - 100.1.2.3 
        - https://www.googleapis.com/compute/v1/projects/project/regions/region/addresses/address 
        - projects/project/regions/region/addresses/address 
        - regions/region/addresses/address 
        - global/addresses/address 
        - address
* `ip-protocol=et`
    - The IP protocol to which this rule applies. Valid options are TCP, UDP, ESP, AH, SCTP or ICMP.
        
        When the load balancing scheme is INTERNAL, only TCP and UDP are valid. When the load balancing scheme is INTERNAL_SELF_MANAGED, only TCPis valid.
* `backend-service=at`
    - This field is only used for INTERNAL load balancing.
        
        For internal load balancing, this field identifies the BackendService resource to receive the matched traffic.
* `creation-timestamp=sit`
    - [Output Only] Creation timestamp in RFC3339 text format.
* `description=ut`
    - An optional description of this resource. Provide this property when you create the resource.
* `id=diam`
    - [Output Only] The unique identifier for the resource. This identifier is defined by the server.
* `ip-version=tempor`
    - The IP Version that will be used by this forwarding rule. Valid options are IPV4 or IPV6. This can only be specified for an external global forwarding rule.
* `kind=et`
    - [Output Only] Type of the resource. Always compute#forwardingRule for Forwarding Rule resources.
* `load-balancing-scheme=erat`
    - This signifies what the ForwardingRule will be used for and can only take the following values: INTERNAL, INTERNAL_SELF_MANAGED, EXTERNAL. The value of INTERNAL means that this will be used for Internal Network Load Balancing (TCP, UDP). The value of INTERNAL_SELF_MANAGED means that this will be used for Internal Global HTTP(S) LB. The value of EXTERNAL means that this will be used for External Load Balancing (HTTP(S) LB, External TCP/UDP LB, SSL Proxy)
* `name=dolores`
    - Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
* `network=kasd`
    - This field is not used for external load balancing.
        
        For INTERNAL and INTERNAL_SELF_MANAGED load balancing, this field identifies the network that the load balanced IP should belong to for this Forwarding Rule. If this field is not specified, the default network will be used.
* `network-tier=et`
    - This signifies the networking tier used for configuring this load balancer and can only take the following values: PREMIUM , STANDARD.
        
        For regional ForwardingRule, the valid values are PREMIUM and STANDARD. For GlobalForwardingRule, the valid value is PREMIUM.
        
        If this field is not specified, it is assumed to be PREMIUM. If IPAddress is specified, this value must be equal to the networkTier of the Address.
* `port-range=clita`
    - This field is used along with the target field for TargetHttpProxy, TargetHttpsProxy, TargetSslProxy, TargetTcpProxy, TargetVpnGateway, TargetPool, TargetInstance.
        
        Applicable only when IPProtocol is TCP, UDP, or SCTP, only packets addressed to ports in the specified range will be forwarded to target. Forwarding rules with the same [IPAddress, IPProtocol] pair must have disjoint port ranges.
        
        Some types of forwarding target have constraints on the acceptable ports:  
        - TargetHttpProxy: 80, 8080 
        - TargetHttpsProxy: 443 
        - TargetTcpProxy: 25, 43, 110, 143, 195, 443, 465, 587, 700, 993, 995, 1688, 1883, 5222 
        - TargetSslProxy: 25, 43, 110, 143, 195, 443, 465, 587, 700, 993, 995, 1688, 1883, 5222 
        - TargetVpnGateway: 500, 4500
* `ports=sed`
    - This field is used along with the backend_service field for internal load balancing.
        
        When the load balancing scheme is INTERNAL, a single port or a comma separated list of ports can be configured. Only packets addressed to these ports will be forwarded to the backends configured with this forwarding rule.
        
        You may specify a maximum of up to 5 ports.
    - Each invocation of this argument appends the given value to the array.
* `region=dolores`
    - [Output Only] URL of the region where the regional forwarding rule resides. This field is not applicable to global forwarding rules. You must specify this field as part of the HTTP request URL. It is not settable as a field in the request body.
* `self-link=clita`
    - [Output Only] Server-defined URL for the resource.
* `subnetwork=eos`
    - This field is only used for INTERNAL load balancing.
        
        For internal load balancing, this field identifies the subnetwork that the load balanced IP should belong to for this Forwarding Rule.
        
        If the network specified is in auto subnet mode, this field is optional. However, if the network is in custom subnet mode, a subnetwork must be specified.
* `target=amet`
    - The URL of the target resource to receive the matched traffic. For regional forwarding rules, this target must live in the same region as the forwarding rule. For global forwarding rules, this target must be a global load balancing resource. The forwarded traffic must be of a type appropriate to the target object. For INTERNAL_SELF_MANAGED&#34; load balancing, only HTTP and HTTPS targets are valid.


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
    - An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed.
        
        For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments.
        
        The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p alt=string**
    - Data format for the response.

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.

* **-p user-ip=string**
    - Deprecated. Please use quotaUser instead.
