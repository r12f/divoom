## Play gif file

Official doc: <http://doc.divoom-gz.com/web/#/12?page_id=195>

This API will generate requests like below:

```json
{
  "Command":"Device/PlayTFGif",
  "FileType": 0,
  "FileName":"divoom_gif/1.gif"
},
{
  "Command":"Device/PlayTFGif",
  "FileType": 1,
  "FileName":"divoom_gif"
},
{
  "Command":"Device/PlayTFGif",
  "FileType": 2,
  "FileName":"http://f.divoom-gz.com/64_64.gif"
}
```