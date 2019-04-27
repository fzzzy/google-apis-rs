Updates an existing account user profile.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3 --scope <scope> account-user-profiles update ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
AccountUserProfile:
  account-id: string
  active: boolean
  advertiser-filter:
    kind: string
    object-ids: [string]
    status: string
  campaign-filter:
    kind: string
    object-ids: [string]
    status: string
  comments: string
  email: string
  id: string
  kind: string
  locale: string
  name: string
  site-filter:
    kind: string
    object-ids: [string]
    status: string
  subaccount-id: string
  trafficker-type: string
  user-access-type: string
  user-role-filter:
    kind: string
    object-ids: [string]
    status: string
  user-role-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=sea`
    - Account ID of the user profile. This is a read-only field that can be left blank.
* `active=true`
    - Whether this user profile is active. This defaults to false, and must be set true on insert for the user profile to be usable.
* `advertiser-filter    kind=duo`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#objectFilter&#34;.
* `object-ids=et`
    - Applicable when status is ASSIGNED. The user has access to objects with these object IDs.
    - Each invocation of this argument appends the given value to the array.
* `status=eirmod`
    - Status of the filter. NONE means the user has access to none of the objects. ALL means the user has access to all objects. ASSIGNED means the user has access to the objects with IDs in the objectIds list.

* `..campaign-filter    kind=sanctus`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#objectFilter&#34;.
* `object-ids=et`
    - Applicable when status is ASSIGNED. The user has access to objects with these object IDs.
    - Each invocation of this argument appends the given value to the array.
* `status=amet`
    - Status of the filter. NONE means the user has access to none of the objects. ALL means the user has access to all objects. ASSIGNED means the user has access to the objects with IDs in the objectIds list.

* `..    comments=et`
    - Comments for this user profile.
* `email=consetetur`
    - Email of the user profile. The email addresss must be linked to a Google Account. This field is required on insertion and is read-only after insertion.
* `id=ut`
    - ID of the user profile. This is a read-only, auto-generated field.
* `kind=ea`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#accountUserProfile&#34;.
* `locale=sed`
    - Locale of the user profile. This is a required field.
        Acceptable values are:  
        - &#34;cs&#34; (Czech) 
        - &#34;de&#34; (German) 
        - &#34;en&#34; (English) 
        - &#34;en-GB&#34; (English United Kingdom) 
        - &#34;es&#34; (Spanish) 
        - &#34;fr&#34; (French) 
        - &#34;it&#34; (Italian) 
        - &#34;ja&#34; (Japanese) 
        - &#34;ko&#34; (Korean) 
        - &#34;pl&#34; (Polish) 
        - &#34;pt-BR&#34; (Portuguese Brazil)
        - &#34;ru&#34; (Russian) 
        - &#34;sv&#34; (Swedish) 
        - &#34;tr&#34; (Turkish) 
        - &#34;zh-CN&#34; (Chinese Simplified) 
        - &#34;zh-TW&#34; (Chinese Traditional)
* `name=dolor`
    - Name of the user profile. This is a required field. Must be less than 64 characters long, must be globally unique, and cannot contain whitespace or any of the following characters: &#34;&amp;;&#34;#%,&#34;.
* `site-filter    kind=dolor`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#objectFilter&#34;.
* `object-ids=dolor`
    - Applicable when status is ASSIGNED. The user has access to objects with these object IDs.
    - Each invocation of this argument appends the given value to the array.
* `status=et`
    - Status of the filter. NONE means the user has access to none of the objects. ALL means the user has access to all objects. ASSIGNED means the user has access to the objects with IDs in the objectIds list.

* `..    subaccount-id=consetetur`
    - Subaccount ID of the user profile. This is a read-only field that can be left blank.
* `trafficker-type=amet.`
    - Trafficker type of this user profile. This is a read-only field.
* `user-access-type=voluptua.`
    - User type of the user profile. This is a read-only field that can be left blank.
* `user-role-filter    kind=lorem`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#objectFilter&#34;.
* `object-ids=gubergren`
    - Applicable when status is ASSIGNED. The user has access to objects with these object IDs.
    - Each invocation of this argument appends the given value to the array.
* `status=justo`
    - Status of the filter. NONE means the user has access to none of the objects. ALL means the user has access to all objects. ASSIGNED means the user has access to the objects with IDs in the objectIds list.

* `..    user-role-id=sit`
    - User role ID of the user profile. This is a required field.


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
    - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.

* **-p user-ip=string**
    - Deprecated. Please use quotaUser instead.
