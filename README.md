# OpenDeck Mirabox 293 V3 Plugin

An unofficial [OpenDeck](https://github.com/nekename/OpenDeck) device support plugin for
the **MiraBox Stream Dock 293 V3** (USB `6603:1006`).

This device is closely related to the Ajazz AKP153 / Mirabox HSV293S family, but it is
not compatible with the plugin that serves them. It reports an unlisted product ID, has a
different key grid, different screens, a different image orientation, a different key
numbering scheme in each direction, and it requires a keepalive. This plugin handles all
of that.

## Is this the right plugin for my device?

Only if your device reports **`6603:1006`**. Other members of the family — including other
293-series revisions — are supported by
[opendeck-akp153](https://github.com/4ndv/opendeck-akp153), which covers `6603:1014`,
`6603:1005` and fifteen other devices. Check yours first:

**Windows** (PowerShell):

```powershell
Get-PnpDevice -PresentOnly | Where-Object { $_.InstanceId -like "*VID_6603*" } | Select-Object FriendlyName, InstanceId
```

**Linux:**

```bash
lsusb | grep -i 6603
```

If you see a different product ID, use the upstream plugin. If you see `1006`, read on.

## Requirements

- OpenDeck 2.14.0 or newer
- The device connected by USB

## Installation

1. Download `opendeck-mirabox293v3.plugin.zip` from the
   [releases page](https://github.com/Grizbeard/opendeck-mirabox293v3/releases). It
   contains binaries for Windows, Linux and macOS; OpenDeck selects the right one at
   runtime from `manifest.json`.
2. In OpenDeck: **Plugins -> Install from file**, and choose the zip.
3. **Linux only:** install the udev rules, or the plugin will not be able to open the
   device:

   ```bash
   sudo cp 40-opendeck-mirabox293v3.rules /etc/udev/rules.d/
   sudo udevadm control --reload-rules && sudo udevadm trigger
   ```

4. Unplug and replug the device, then restart OpenDeck.

<details>
<summary>Installing by hand instead</summary>

Extract the zip so that the `local.plugins.opendeck-mirabox293v3.sdPlugin` folder inside
it lands directly in OpenDeck's `plugins` directory — on Windows that is
`%APPDATA%\opendeck\plugins\`. Leave the folder name and the binary names exactly as
they are: OpenDeck resolves both from `manifest.json`.

</details>

Only the Windows build has been tested on hardware. The Linux and macOS binaries are
built by CI and are expected to work, but are unverified.

## Building

Requires Rust 1.87 or newer and a working C toolchain for your platform.

**Windows** (from a shell where the MSVC environment has been initialised, e.g. after
running `vcvars64.bat`):

```bash
cargo build --release --target x86_64-pc-windows-msvc
```

The binary lands at `target/x86_64-pc-windows-msvc/release/opendeck-mirabox293v3.exe`.

**Linux / macOS:**

```bash
cargo build --release
```

The binary lands at `target/release/opendeck-mirabox293v3`.

### Packaging

`package.py` assembles the `.sdPlugin` folder and produces an installable zip. Pass
whichever binaries you have built:

```bash
python package.py --win target/x86_64-pc-windows-msvc/release/opendeck-mirabox293v3.exe
```

A full release zip carries all three platforms. Since that cannot be produced on a single
machine without cross-compilation, CI does it: pushing a `v*` tag runs
`.github/workflows/release.yml`, which builds each platform natively — macOS as a
universal `x86_64` + `arm64` binary — then packages and attaches the zip to the release.

## Device characteristics

These were established by observation on real hardware. They are recorded here because
each one differs from the AKP153-family devices, and because anyone adapting this to
another unlisted revision will need to work out the same list.

| | AKP153 family | MiraBox 293 V3 |
|---|---|---|
| Key grid | 3x6 — 18 keys | **3x5 — 15 keys** |
| Key screens | 95x95, plus 82x82 in the sixth column | **112x112, uniform** |
| Image transform | `Rot90` + mirror both (net 270°) | **`Rot180`, no mirroring** |
| Image key order | column-major from the right | **row-major from the bottom left** |
| Key press order | same as the image order | **row-major from the top left** |
| Idle keepalive | not required | **required, ~35 s timeout** |

Two of these are worth calling out.

**The key numbering is asymmetric.** Images are addressed row-major from the bottom left,
but key presses arrive row-major from the top left. The two are not inverses of each
other, so they need separate mappings:

```text
  image order      press order      opendeck
 10 11 12 13 14    0  1  2  3  4    0  1  2  3  4
  5  6  7  8  9    5  6  7  8  9    5  6  7  8  9
  0  1  2  3  4   10 11 12 13 14   10 11 12 13 14
```

The middle row is identical under all three, so a mapping error here shows up only as the
top and bottom rows being swapped — easy to misread as a rotation problem.

**The device requires a keepalive.** Without traffic it closes the connection after about
35 seconds, reporting `HidError(Disconnected)`, and then immediately reconnects. The
`mirajazz` library provides `Device::keep_alive()` for this; this plugin calls it every
15 seconds.

## Adapting this to another revision

If you have a 293-series device with a product ID that neither this plugin nor the
upstream one recognises, the sequence that worked here was:

1. Find the VID/PID and confirm the HID usage page is `65440` / usage `1`.
2. Add the query in `src/mappings.rs`. The device should now be detected, even if it
   renders nothing useful.
3. Set a distinctive image on every key and photograph the device. Which keys stay dark
   tells you the real key count; where each image lands gives you the image key order;
   how the images are turned gives you the rotation.
4. Render each key at a different pixel size and photograph it again. The key that
   exactly fills its screen is the panel resolution.
5. Press keys and read the log — `process_input` logs every press. Do not assume this is
   the inverse of the image order; on this device it is not.

## Troubleshooting

**The device is not detected.** Confirm the product ID is `1006` (see above). On Linux,
confirm the udev rules are installed. The plugin logs every candidate it finds to
`%LOCALAPPDATA%\opendeck\logs\plugins\` on Windows, or
`~/.local/share/opendeck/logs/plugins/` on Linux.

**It connects, then disconnects every ~35 seconds.** The keepalive task is not running.
This is what `device_keepalive_task` in `src/device.rs` exists to prevent.

**Images are rotated, or land in the wrong place.** The constants in
`get_image_format_for_key` and the tables in `src/inputs.rs` do not match your hardware —
see [Adapting this to another revision](#adapting-this-to-another-revision).

**Images do not fill the screens.** The size in `get_image_format_for_key` is smaller than
the panel. The device draws from its own origin, so an undersized image sits against one
corner rather than centring.

## Acknowledgments

This is a fork of [opendeck-akp153](https://github.com/4ndv/opendeck-akp153) by Andrey
Viktorov, which did all of the hard work — the device task structure, the watcher, the
image pipeline and the OpenDeck integration are all his. It is built on
[mirajazz](https://github.com/4ndv/mirajazz), the same author's library for Mirabox and
Ajazz stream controllers, and targets [OpenDeck](https://github.com/nekename/OpenDeck) by
Aman Khanna.

The port to this device was written with the help of an LLM coding assistant, working from
observations taken on physical hardware. Every device characteristic documented above was
verified on the device itself rather than inferred.

Please do not report issues with this plugin to the upstream projects. Note also that
opendeck-akp153 has an explicit policy prohibiting LLM-written text in issues and pull
requests; if you intend to contribute anything from here back to it, write it yourself.

## License

GPL-3.0, inherited from opendeck-akp153. See [LICENSE](LICENSE).
