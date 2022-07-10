## Send image animation

This API sends the image animation definitions to Pixoo devices and trigger it play.

To create an animation, the first request to draw the frame should always have with `PicOffset` set to 0, because it creates the image, otherwise other request cannot go through correctly.

Official doc: <http://doc.divoom-gz.com/web/#/12?page_id=93>

This API will generate requests like below:

```json
{
  "Command":"Draw/SendHttpGif",
  "PicNum":2,
  "PicWidth":64,
  "PicOffset":0,
  "PicID":3,
  "PicSpeed":100,
  "PicData":"AAIpAAIp..."
}
```