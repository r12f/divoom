# Get clock list

Official doc: <http://doc.divoom-gz.com/web/#/12?page_id=190>

Sample request:

```json
{
    "DialType":"Social",
    "Page":1
}
```

Sample response:

```json
{
    "ReturnCode": 0,
    "ReturnMessage": "",
    "TotalNum": 100,
    "DialList": [
        {
            "ClockId": 10,
            "Name": "Classic Digital Clock"
        },
        ...
    ]
}
```