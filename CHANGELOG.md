# Changelog

## 0.1.0

Initial release. Forked from [opendeck-akp153](https://github.com/4ndv/opendeck-akp153)
v0.11.1 and adapted to the MiraBox Stream Dock 293 V3 (`6603:1006`).

Changes relative to the upstream plugin:

- Added the `6603:1006` device query; removed the other 17 devices, which are still
  served by the upstream plugin.
- Grid changed from 3x6 (18 keys) to 3x5 (15 keys).
- Key image size changed from 95x95 / 82x82 to a uniform 112x112.
- Image transform changed from `Rot90` + `MirrorBoth` to `Rot180` with no mirroring.
- Image key order changed to row-major from the bottom left.
- Key press order changed to row-major from the top left, which on this device is *not*
  the inverse of the image order.
- Added a keepalive task; this device drops the connection after ~35 s of silence.
- New plugin icon, UUID (`local.plugins.opendeck-mirabox293v3`) and device namespace
  (`93`), so it installs alongside the upstream plugin without conflict.
