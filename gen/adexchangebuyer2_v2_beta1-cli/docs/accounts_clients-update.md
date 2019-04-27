Updates an existing client buyer.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer2-v2-beta1 --scope <scope> accounts clients-update ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - Unique numerical account ID for the buyer of which the client buyer
        is a customer; the sponsor buyer to update a client for. (required)
* **&lt;client-account-id&gt;** *(string)*
    - Unique numerical account ID of the client to update. (required)
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Client:
  client-account-id: string
  client-name: string
  entity-id: string
  entity-name: string
  entity-type: string
  partner-client-id: string
  role: string
  status: string
  visible-to-seller: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    client-account-id=labore`
    - The globally-unique numerical ID of the client.
        The value of this field is ignored in create and update operations.
* `client-name=sea`
    - Name used to represent this client to publishers.
        You may have multiple clients that map to the same entity,
        but for each client the combination of `clientName` and entity
        must be unique.
        You can specify this field as empty.
* `entity-id=nonumy`
    - Numerical identifier of the client entity.
        The entity can be an advertiser, a brand, or an agency.
        This identifier is unique among all the entities with the same type.
        
        A list of all known advertisers with their identifiers is available in the
        [advertisers.txt](https://storage.googleapis.com/adx-rtb-dictionaries/advertisers.txt)
        file.
        
        A list of all known brands with their identifiers is available in the
        [brands.txt](https://storage.googleapis.com/adx-rtb-dictionaries/brands.txt)
        file.
        
        A list of all known agencies with their identifiers is available in the
        [agencies.txt](https://storage.googleapis.com/adx-rtb-dictionaries/agencies.txt)
        file.
* `entity-name=dolores`
    - The name of the entity. This field is automatically fetched based on
        the type and ID.
        The value of this field is ignored in create and update operations.
* `entity-type=gubergren`
    - The type of the client entity: `ADVERTISER`, `BRAND`, or `AGENCY`.
* `partner-client-id=sadipscing`
    - Optional arbitrary unique identifier of this client buyer from the
        standpoint of its Ad Exchange sponsor buyer.
        
        This field can be used to associate a client buyer with the identifier
        in the namespace of its sponsor buyer, lookup client buyers by that
        identifier and verify whether an Ad Exchange counterpart of a given client
        buyer already exists.
        
        If present, must be unique among all the client buyers for its
        Ad Exchange sponsor buyer.
* `role=aliquyam`
    - The role which is assigned to the client buyer. Each role implies a set of
        permissions granted to the client. Must be one of `CLIENT_DEAL_VIEWER`,
        `CLIENT_DEAL_NEGOTIATOR` or `CLIENT_DEAL_APPROVER`.
* `status=ea`
    - The status of the client buyer.
* `visible-to-seller=false`
    - Whether the client buyer will be visible to sellers.


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
