#!/usr/bin/env python3
"""Assemble the .sdPlugin folder and zip it for distribution.

Pass whichever platform binaries you have; the zip will contain just those. A release
zip should carry all three, since OpenDeck picks one at runtime from manifest.json.

    python package.py --win target/x86_64-pc-windows-msvc/release/opendeck-mirabox293v3.exe
"""
import argparse
import os
import shutil
import zipfile

PLUGIN_ID = "local.plugins.opendeck-mirabox293v3.sdPlugin"

# flag name -> filename inside the plugin folder, which must match manifest.json
BINARIES = {
    "linux": "opendeck-mirabox293v3-linux",
    "mac": "opendeck-mirabox293v3-mac",
    "win": "opendeck-mirabox293v3-win.exe",
}
EXECUTABLE = {"linux", "mac"}  # need the exec bit preserved in the archive


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    for flag in BINARIES:
        ap.add_argument("--" + flag, metavar="PATH", help="built %s binary" % flag)
    ap.add_argument("--out", default="opendeck-mirabox293v3.plugin.zip")
    ap.add_argument("--build-dir", default="build")
    args = ap.parse_args()

    stage = os.path.join(args.build_dir, PLUGIN_ID)
    if os.path.exists(stage):
        shutil.rmtree(stage)
    os.makedirs(stage)

    shutil.copy("manifest.json", stage)
    shutil.copytree("assets", os.path.join(stage, "assets"))

    included = []
    for flag, target in BINARIES.items():
        src = getattr(args, flag)
        if not src:
            continue
        if not os.path.isfile(src):
            raise SystemExit("no such binary: %s" % src)
        shutil.copy(src, os.path.join(stage, target))
        included.append(flag)

    if not included:
        raise SystemExit("no binaries given -- pass at least one of --linux/--mac/--win")

    out = os.path.join(args.build_dir, args.out)
    if os.path.exists(out):
        os.remove(out)

    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
        for root, _dirs, files in os.walk(stage):
            for name in sorted(files):
                full = os.path.join(root, name)
                # forward slashes per the ZIP spec, so it extracts on every platform
                arc = os.path.relpath(full, args.build_dir).replace(os.sep, "/")
                info = zipfile.ZipInfo.from_file(full, arc)
                info.compress_type = zipfile.ZIP_DEFLATED
                mode = 0o755 if name in {BINARIES[f] for f in EXECUTABLE} else 0o644
                info.external_attr = mode << 16
                with open(full, "rb") as fh, z.open(info, "w") as target_fh:
                    shutil.copyfileobj(fh, target_fh)

    print("%s  (%s)  %.2f MB" % (out, ", ".join(sorted(included)),
                                 os.path.getsize(out) / 1048576))


if __name__ == "__main__":
    main()
