Deletes an object and its metadata. Deletions are permanent if versioning is not enabled for the bucket, or if the generation parameter is used.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/devstorage.full_control*
* *https://www.googleapis.com/auth/devstorage.read_write*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `storage1 --scope <scope> objects delete ...`
# Required Scalar Arguments
* **&lt;bucket&gt;** *(string)*
    - Name of the bucket in which the object resides.
* **&lt;object&gt;** *(string)*
    - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p if-generation-not-match=string**
    - Makes the operation conditional on whether the object&#39;s current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object.

* **-p if-metageneration-not-match=string**
    - Makes the operation conditional on whether the object&#39;s current metageneration does not match the given value.

* **-p if-generation-match=string**
    - Makes the operation conditional on whether the object&#39;s current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object.

* **-p generation=string**
    - If present, permanently deletes a specific revision of this object (as opposed to the latest version, the default).

* **-p if-metageneration-match=string**
    - Makes the operation conditional on whether the object&#39;s current metageneration matches the given value.

* **-p user-project=string**
    - The project to be billed for this request. Required for Requester Pays buckets.

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
