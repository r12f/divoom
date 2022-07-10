## Play gif file

Play GIF file from specific location. 

**NOTE**: this API onliy supports 16x16, 32x32 and 64x64 GIFs. 
If other format is specified, it could cause the device to crash and reboot.

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