## Send text animation

The text animation will only work when the device is in the drawing mode (showing image animation).

If we send text animation when device is showing other things such as clock, the text animation will simply be ignored.

Official doc: <http://doc.divoom-gz.com/web/#/12?page_id=219>

This API will generate requests like below:

```json
{
  "Command":"Draw/SendHttpText",
  "TextId":4,
  "x":0,
  "y":40,
  "dir":0,
  "font":4,
  "TextWidth":56,
  "speed":10,
  "TextString":"hello, Divoom",
  "color":"#FFFF00",
  "align":1
}
```