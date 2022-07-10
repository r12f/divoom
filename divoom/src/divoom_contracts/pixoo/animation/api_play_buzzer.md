## Play buzzer

Trigger buzzer to make some beeps sound:

**NOTE**: For Pixoo-64 device, every buzz is about 50ms long and the device cannot handle request <100ms very well. Hence:

* For active time longer than 100ms, it will play buzz in each cycle multiple times.
* If we set active time smaller than 50ms or off time smaller than 100ms, it might not buzz or off at all.

Official doc: <http://doc.divoom-gz.com/web/#/12?page_id=347>

This API will generate requests like below:
```json
{
  "Command":"Device/PlayTFGif",
  "ActiveTimeInCycle": 500,
  "OffTimeInCycle": 500,
  "PlayTotalTime": 3000
}
```