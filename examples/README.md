# Disposable demonstration

Run `python3 examples/demo.py` from any directory. It builds the locked release
binary, discovers Cargo's target directory (including configured shared targets),
and exercises the CLI in a temporary journal. It does not inspect or edit live data.
Use `--binary /absolute/path/to/rustylife` to test an existing build.

The script checks daily/weekly progress, both CSV forms and byte-exact backup
recovery, then cleans up the temporary directory. Displayed names and dates are
intentional example entries, not personal records. Python's temporary directory
selection respects `TMPDIR`/`TEMP` when configured.

CI runs this alongside format, lint, integration and documentation checks on
Linux, macOS and Windows. Local execution is not evidence that remote CI ran.
