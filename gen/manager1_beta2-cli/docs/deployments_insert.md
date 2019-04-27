
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/appengine.admin*
* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/compute*
* *https://www.googleapis.com/auth/devstorage.read_write*
* *https://www.googleapis.com/auth/ndev.cloudman*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/appengine.admin*.
You can set the scope for this method like this: `manager1-beta2 --scope <scope> deployments insert ...`
# Required Scalar Arguments
* **&lt;project-id&gt;** *(string)*
    - No description provided.
* **&lt;region&gt;** *(string)*
    - No description provided.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Deployment:
  creation-date: string
  description: string
  name: string
  state:
    details: string
    status: string
  template-name: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    creation-date=eirmod`
    - [Output Only] The time when this deployment was created.
* `description=sit`
    - A user-supplied description of this Deployment.
* `name=stet`
    - Name of this deployment. The name must conform to the following regular expression: [a-zA-Z0-9-_]{1,64}
* `state    details=sed`
    - [Output Only] Human readable details about the current state.
* `status=et`
    - [Output Only] The status of the deployment. Possible values include: 
        - UNKNOWN
        - DEPLOYING
        - DEPLOYED
        - DEPLOYMENT_FAILED
        - DELETING
        - DELETED
        - DELETE_FAILED

* `..    template-name=dolores`
    - The name of the Template on which this deployment is based.


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
