# timestamp

## How does it work?

1. API endpoint is `/api/timestamp/:date_string`
2. Parses timestamp (RFC-3389, ISO-8601, UNIX are supported)
3. If timestamp is valid, returns following response
```js
{"unix":1095379198000,"utc":"Thu, 16 Sep 2004 23:59:58 +0000"}
```
4. If timestamp is invalid, sends null response
