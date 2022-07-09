## Send image animation frame

Official doc: <http://doc.divoom-gz.com/web/#/12?page_id=93>

To create an animation, the first request to draw the frame should always have with `PicOffset` set to 0.
This creates the image, then the subsequent frames can be done in any order.

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