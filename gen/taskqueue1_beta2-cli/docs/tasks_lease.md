Lease 1 or more tasks from a TaskQueue.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/taskqueue*
* *https://www.googleapis.com/auth/taskqueue.consumer*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/taskqueue*.
You can set the scope for this method like this: `taskqueue1-beta2 --scope <scope> tasks lease ...`
# Required Scalar Arguments
* **&lt;project&gt;** *(string)*
    - The project under which the queue lies.
* **&lt;taskqueue&gt;** *(string)*
    - The taskqueue to lease a task from.
* **&lt;num-tasks&gt;** *(integer)*
    - The number of tasks to lease.
* **&lt;lease-secs&gt;** *(integer)*
    - The lease in seconds.

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

* **-p group-by-tag=boolean**
    - When true, all returned tasks will have the same tag

* **-p tag=string**
    - The tag allowed for tasks in the response. Must only be specified if group_by_tag is true. If group_by_tag is true and tag is not specified the tag will be that of the oldest task by eta, i.e. the first available tag

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
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.

* **-p user-ip=string**
    - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
