Returns a list of flights.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
TripsSearchRequest:
  request:
    max-price: string
    passengers:
      adult-count: integer
      child-count: integer
      infant-in-lap-count: integer
      infant-in-seat-count: integer
      kind: string
      senior-count: integer
    refundable: boolean
    sale-country: int64
    solutions: integer
    ticketing-country: int64

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .request    max-price=eirmod`
    - Do not return solutions that cost more than this price. The alphabetical part of the price is in ISO 4217. The format, in regex, is [A-Z]{3}\d+(\.\d+)? Example: $102.07
* `passengers    adult-count=53`
    - The number of passengers that are adults.
* `child-count=36`
    - The number of passengers that are children.
* `infant-in-lap-count=59`
    - The number of passengers that are infants travelling in the lap of an adult.
* `infant-in-seat-count=16`
    - The number of passengers that are infants each assigned a seat.
* `kind=dolores`
    - Identifies this as a passenger count object, representing the number of passengers. Value: the fixed string qpxexpress#passengerCounts.
* `senior-count=38`
    - The number of passengers that are senior citizens.

* `..    refundable=true`
    - Return only solutions with refundable fares.
* `sale-country=-8`
    - IATA country code representing the point of sale. This determines the &#34;equivalent amount paid&#34; currency for the ticket.
* `solutions=31`
    - The number of solutions to return, maximum 500.
* `ticketing-country=-1`
    - IATA country code representing the point of ticketing.



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
