# Custom Windows print defaults (candidate, not compiled here)

Base: storytold/pdfcraft v0.14.0, 5a78b223871add8e3883856205cc653d741e0e94.
This is an unofficial customization, not an upstream release or a server deployment.

Each opening of Print on Windows seeds Grayscale=true and Duplex=LongEdge. An administrator
can replace these defaults in the **64-bit registry view** at
`HKLM\SOFTWARE\PdfCraft\PrintDefaults`. Values must be REG_DWORD:

| Value | Meaning | Missing/invalid/unreadable |
|---|---|---|
| Grayscale | 0 = colour, 1 = grayscale | grayscale |
| Duplex | 0 = printer default, 1 = off, 2 = long edge, 3 = short edge | long edge |

No registry writes occur in the application. Standard Windows HKLM permissions let users read
and administrators configure; do not grant ordinary RDS users write access to this key.
Every architecture reads the same 64-bit view. The key applies to all sessions on that host,
not automatically to other hosts in an RDS farm. Stock PdfCraft ignores this new key.

The checkbox and Two-sided menu stay editable. The next opening resets these two choices,
including after Cancel; changes while the dialog is open are never overwritten. Other dialog
preferences keep upstream behavior. No per-user colour/duplex persistence is added. Defaults
are read on every opening, so existing processes see administrator changes next time they open
Print. Non-Windows platforms and headless `doc_print` defaults are unchanged.

Booklets retain upstream's short-edge / manual-single-side handling. Cut-and-stack still
requires choosing Off and gives the existing validation error otherwise. Unsupported duplex
devices retain the upstream warning/driver behavior: this patch adds no capability fallback.

Headless UI control (still opt-in): after `file.print`, `ui.set` accepts `print-grayscale`
(`true` or `false`) and `print-duplex` (`printer`, `off`, `long-edge`, `short-edge`). `ui.state`
reports them under `print`. These settings go through the same draft and job as the GUI.

## Verified upstream Windows path

`PrintDraft::job` passes the dialog values through `pdfcraft-print::spool` to `craft-print`.
For this release Cargo.toml pins craft-print to craft-libs commit
`666c358065261a2816b376fa3bb10538dbf53040`. Its `crates/print/src/ticket.rs` writes
`PageOutputColor=Monochrome` for grayscale and **Color** when unchecked; long-edge uses
`JobDuplexAllDocumentsContiguously=TwoSidedLongEdge`. `windows.rs` merges a driver-validated
PrintTicket and prints through Windows.Data.Pdf / XPS, without a PowerShell printing process.
This explains why a queue's black-and-white preference alone does not solve the reported
behavior in 0.14.0. Newer main pins a different craft-print commit and omits the color request
when grayscale is unchecked; that newer behavior is not backported or claimed here.

The release backend checks paper/orientation/scaling but does not reliably reject all driver
substitutions of colour/duplex. Inspect the spooled ticket and actual output on each RDS driver.
Printer redirection / Easy Print can affect capabilities. No hardware validation was possible here.
Save as PDF does not gain vector grayscale conversion from this change: the checkbox normally
requests driver monochrome; Print as image has separate raster grayscale handling.

## Tests and limitations

Added decoder tests cover missing, invalid, independent and every supported DWORD value.
Added UI-to-job/control tests cover overrides, invalid-input preservation, reopening, booklet,
cut-and-stack and the unchanged non-Windows path. No test submits a print job or edits HKLM.

Rust/cargo and MSVC build tools are absent on the preparation machine. A cargo test attempt
failed before compilation because cargo was not found. None of these Rust tests, clippy,
formatting, wasm, asset/layer checks, UI rendering, EXE or MSI builds have passed here.
The registry adapter was checked against winsafe 0.0.29 source, but not compiled or run.
The patch is a reviewable candidate, not a qualified production binary.

The neutral text in the Home/About brand widgets removes the embedded upstream ArtCraft marks
from the custom binary, as required by docs/brand/LICENSE-brand.txt. The patch bundle contains
no upstream logo assets. The upstream clone remains a build input; do not redistribute its
brand directory as part of a modified source distribution. Preserve open-source notices.
