Creates a Project resource.

Initially, the Project resource is owned by its creator exclusively.
The creator can later grant permission to others to read or update the
Project.

Several APIs are activated automatically for the Project, including
Google Cloud Storage. The parent is identified by a specified
ResourceId, which must include both an ID and a type, such as
project, folder, or organization.

This method does not associate the new project with a billing account.
You can set or update the billing account associated with a project using
the [`projects.updateBillingInfo`]
(/billing/reference/rest/v1/projects/updateBillingInfo) method.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudresourcemanager1-beta1 --scope <scope> projects create ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Project:
  create-time: string
  labels: { string: string }
  lifecycle-state: string
  name: string
  parent:
    id: string
    type: string
  project-id: string
  project-number: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    create-time=amet.`
    - Creation time.
        
        Read-only.
* `labels=key=erat`
    - The labels associated with this Project.
        
        Label keys must be between 1 and 63 characters long and must conform
        to the following regular expression: \[a-z\](\[-a-z0-9\]*\[a-z0-9\])?.
        
        Label values must be between 0 and 63 characters long and must conform
        to the regular expression (\[a-z\](\[-a-z0-9\]*\[a-z0-9\])?)?.
        
        No more than 256 labels can be associated with a given resource.
        
        Clients should store labels in a representation such as JSON that does not
        depend on specific characters being disallowed.
        
        Example: &lt;code&gt;&#34;environment&#34; : &#34;dev&#34;&lt;/code&gt;
        Read-write.
    - the value will be associated with the given `key`
* `lifecycle-state=labore`
    - The Project lifecycle state.
        
        Read-only.
* `name=sea`
    - The user-assigned display name of the Project.
        It must be 4 to 30 characters.
        Allowed characters are: lowercase and uppercase letters, numbers,
        hyphen, single-quote, double-quote, space, and exclamation point.
        
        Example: &lt;code&gt;My Project&lt;/code&gt;
        Read-write.
* `parent    id=nonumy`
    - Required field for the type-specific id. This should correspond to the id
        used in the type-specific API&#39;s.
* `type=dolores`
    - Required field representing the resource type this id is for.
        At present, the valid types are &#34;project&#34;, &#34;folder&#34;, and &#34;organization&#34;.

* `..    project-id=gubergren`
    - The unique, user-assigned ID of the Project.
        It must be 6 to 30 lowercase letters, digits, or hyphens.
        It must start with a letter.
        Trailing hyphens are prohibited.
        
        Example: &lt;code&gt;tokyo-rain-123&lt;/code&gt;
        Read-only after creation.
* `project-number=sadipscing`
    - The number uniquely identifying the project.
        
        Example: &lt;code&gt;415104041262&lt;/code&gt;
        Read-only.


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

* **-p use-legacy-stack=boolean**
    - A safety hatch to opt out of the new reliable project creation process.

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
