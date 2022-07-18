## Send image animation

This API sends the image animation definitions to Pixoo devices and trigger it play.

To create an animation, the first request to draw the frame should always have with `PicOffset` set to 0, because it creates the image, otherwise other request cannot go through correctly.

When using the API in our library, we will ask for the `id` in the parameter, which maps to `PicId` in the final request. Note that:

- This `id` should be the id that returned by `get_next_animation_id`.
- If `DIVOOM_IMAGE_ANIMATION_ID_AUTO` is used, we will automatically get the latest animation id and use it.

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