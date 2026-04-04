# CLI

## Commands

```sh
gui check examples/demo.gui
gui check examples/demo.gui other.gui
gui check

gui page examples/demo.gui
gui page
gui drill
gui inherit
gui node
gui nav

gui scan page1.html page2.html
gui scan --stage summary state.snapshot.yaml
gui compare left.snapshot.yaml right.snapshot.yaml
```

## Input resolution

If file arguments are omitted, `gui` recursively scans the current working
directory and uses the union of all matching `*.gui` files.

If multiple `.gui` files are given explicitly, they are merged into one logical
document before the command runs.

## Command summary

- `check`: parse and validate `.gui` input
- `page`: list nodes that satisfy the current page rules
- `drill`: print the `drill` tree with indentation
- `inherit`: print the `inherit` tree with indentation
- `node`: list node ids
- `nav`: list nav ids
- `scan`: infer `.gui` from rendered HTML files and print to stdout
- `scan --stage summary`: emit a YAML summary instead of `.gui`
- `compare`: compare two HTML or snapshot-manifest inputs through the summary layer

## Typical workflows

Validate one file:

```sh
gui check examples/demo.gui
```

Validate all `.gui` files in the current tree:

```sh
gui check
```

Inspect page ids:

```sh
gui page
```

Scan rendered HTML into `.gui`:

```sh
gui scan saved/home.html saved/pricing.html > site.gui
```

Scan a snapshot manifest into a YAML summary:

```sh
gui scan --stage summary saved/wizard.snapshot.yaml
```

## Compare workflow

Compare two saved states directly:

```sh
gui compare saved/origin.snapshot.yaml saved/clone.snapshot.yaml
```

Compare with app-specific config:

```sh
gui compare --config app-scan-config.yaml \
  saved/origin.snapshot.yaml \
  saved/clone.snapshot.yaml
```

The compare report currently emits findings such as:

- `missing-dialog`
- `missing-control`
- `unexpected-control`
- `state-hint-mismatch`
- `stepper-mismatch`
- `nav-mismatch`
- `nav-label-mismatch`

Useful config sections are:

- `stepper`: selectors and active-state hints for wizard/stepper extraction
- `snapshot`: which `stateHints` keys should be treated as flow or step hints
- `compare.dynamic_selectors`: regions whose text/count/image differences should be treated as dynamic
- `compare.dynamic_text_patterns`: regexes for dynamic text
- `compare.normalize_patterns`: regex replacements applied before text comparison

## Notes

- `gui scan` does not fetch pages or execute JavaScript.
- HTML acquisition is intentionally out of scope for the CLI.
