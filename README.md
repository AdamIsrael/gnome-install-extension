# gnome-install-extension

[![Continuous integration](https://github.com/AdamIsrael/gnome-install-extension/actions/workflows/ci.yaml/badge.svg)](https://github.com/AdamIsrael/gnome-install-extension/actions/workflows/ci.yaml)

A small CLI utility to install GNOME extensions from extensions.gnome.org.

```bash
Usage: gnome-install-extension [OPTIONS] [SEARCH]...

Arguments:
  [SEARCH]...  The URL, UUID, or keyword(s) to the extension on extensions.gnome.org

Options:
  -q, --quiet    Turn off all output
  -v, --verbose  Be more verbose
      --dry-run  Do everything except install the extension
  -h, --help     Print help
  -V, --version  Print version
```
