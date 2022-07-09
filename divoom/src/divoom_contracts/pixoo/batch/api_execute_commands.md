## Execute commands from list

Official doc: <http://doc.divoom-gz.com/web/#/12?page_id=241>

This API will generate requests like below:

```json
{
  "Command":"Draw/CommandList",
  "CommandList": [
    {
      "Command":"Device/SetUTC",
      "Utc": 1672416000
    }
  ]
}
```