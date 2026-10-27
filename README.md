# PdfCraft 0.14.0 — custom Windows RDS build

Unofficial customization based on [storytold/pdfcraft v0.14.0](https://github.com/storytold/pdfcraft/releases/tag/v0.14.0), commit 5a78b223871add8e3883856205cc653d741e0e94.

On Windows, each new Print dialog starts with grayscale enabled and long-edge duplex. Users can change both per job. Administrators can change defaults through the 64-bit HKLM\SOFTWARE\PdfCraft\PrintDefaults registry key. Stock upstream 0.14.0 does not read this key.

See [configuration and limitations](docs/windows-rds-print-defaults.md) and [Italian instructions](docs/rds/LEGGIMI.md).

The RDS Windows build workflow tests print-dialog logic, builds app and CLI, and attempts unsigned MSI/portable ZIP packaging. Download packages only from a successful run on branch fix/windows-rds-print-defaults. No physical printer, RDS, Easy Print or production installer validation is implied by a successful build. Numeric MSI version remains 0.14.0; test install and rollback on a VM.

Upstream trademark artwork is removed from this branch and the custom binary. The project is based on PdfCraft by the ArtCraft team and contributors; this customization is not an official or endorsed release. Open-source notices and the MIT/Apache-2.0 licenses are preserved.