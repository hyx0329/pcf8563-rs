# PCF8563 driver

This driver is test on BM8563, which is a PCF8563 clone and used by M5Stack Core2.

One old driver on GitHub uses embedded-hal 0.2 which doesn't fit my purpose, so I write a
new one.

Not all features are implemented.

The implemented features

- Get datetime
- Set datetime
- Alarms
- Power lost
- Interrupt signal
