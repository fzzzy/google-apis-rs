Creates/Transfers a subscription for the customer.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/apps.order* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/apps.order*.
You can set the scope for this method like this: `reseller1-sandbox --scope <scope> subscriptions insert ...`
# Required Scalar Argument
* **&lt;customer-id&gt;** *(string)*
    - Id of the Customer
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Subscription:
  billing-method: string
  creation-time: string
  customer-domain: string
  customer-id: string
  deal-code: string
  kind: string
  plan:
    commitment-interval:
      end-time: string
      start-time: string
    is-commitment-plan: boolean
    plan-name: string
  purchase-order-id: string
  renewal-settings:
    kind: string
    renewal-type: string
  resource-ui-url: string
  seats:
    kind: string
    licensed-number-of-seats: integer
    maximum-number-of-seats: integer
    number-of-seats: integer
  sku-id: string
  status: string
  subscription-id: string
  suspension-reasons: [string]
  transfer-info:
    minimum-transferable-seats: integer
    transferability-expiration-time: string
  trial-settings:
    is-in-trial: boolean
    trial-end-time: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    billing-method=consetetur`
    - Billing method of this subscription.
* `creation-time=amet.`
    - Creation time of this subscription in milliseconds since Unix epoch.
* `customer-domain=voluptua.`
    - Primary domain name of the customer
* `customer-id=lorem`
    - The id of the customer to whom the subscription belongs.
* `deal-code=gubergren`
    - External name of the deal, if this subscription was provisioned under one. Otherwise this field will be empty.
* `kind=justo`
    - Identifies the resource as a Subscription.
* `plan.commitment-interval    end-time=sit`
    - End time of the commitment interval in milliseconds since Unix epoch.
* `start-time=vero`
    - Start time of the commitment interval in milliseconds since Unix epoch.

* `..    is-commitment-plan=true`
    - Whether the plan is a commitment plan or not.
* `plan-name=rebum.`
    - The plan name of this subscription&#39;s plan.

* `..    purchase-order-id=consetetur`
    - Purchase order id for your order tracking purposes.
* `renewal-settings    kind=sadipscing`
    - Identifies the resource as a subscription renewal setting.
* `renewal-type=vero`
    - Subscription renewal type.

* `..    resource-ui-url=sadipscing`
    - Ui url for subscription resource.
* `seats    kind=invidunt`
    - Identifies the resource as a subscription change plan request.
* `licensed-number-of-seats=5`
    - Read-only field containing the current number of licensed seats for FLEXIBLE Google-Apps subscriptions and secondary subscriptions such as Google-Vault and Drive-storage.
* `maximum-number-of-seats=17`
    - Maximum number of seats that can be purchased. This needs to be provided only for a non-commitment plan. For a commitment plan it is decided by the contract.
* `number-of-seats=82`
    - Number of seats to purchase. This is applicable only for a commitment plan.

* `..    sku-id=aliquyam`
    - Name of the sku for which this subscription is purchased.
* `status=lorem`
    - Status of the subscription.
* `subscription-id=et`
    - The id of the subscription.
* `suspension-reasons=clita`
    - Read-only field containing an enumerable of all the current suspension reasons for a subscription. It is possible for a subscription to have many concurrent, overlapping suspension reasons. A subscription&#39;s STATUS is SUSPENDED until all pending suspensions are removed. Possible options include:  
        - PENDING_TOS_ACCEPTANCE - The customer has not logged in and accepted the Google Apps Resold Terms of Services.  
        - RENEWAL_WITH_TYPE_CANCEL - The customer&#39;s commitment ended and their service was cancelled at the end of their term.  
        - RESELLER_INITIATED - A manual suspension invoked by a Reseller.  
        - TRIAL_ENDED - The customer&#39;s trial expired without a plan selected.  
        - OTHER - The customer is suspended for an internal Google reason (e.g. abuse or otherwise).
    - Each invocation of this argument appends the given value to the array.
* `transfer-info    minimum-transferable-seats=56`
    - No description provided.
* `transferability-expiration-time=takimata`
    - Time when transfer token or intent to transfer will expire.

* `..trial-settings    is-in-trial=true`
    - Whether the subscription is in trial.
* `trial-end-time=kasd`
    - End time of the trial in milliseconds since Unix epoch.



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

* **-p customer-auth-token=string**
    - An auth token needed for transferring a subscription. Can be generated at https://www.google.com/a/cpanel/customer-domain/TransferToken. Optional.

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
