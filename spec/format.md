# `.gui` draft format

## Core model

Each GUI model is defined by four main parts:

- `inherit`: a forest for visual or behavioral inheritance
- `drill`: a forest for semantic drill-down in the information space
- `nav`: named ordered target page lists
- `pages`: page objects keyed by page id

The model intentionally does not require a top-level `transitions` section.
Many practical transitions are derived from:

- movement along the `drill` tree
- selection of a target in a `nav`

## Minimal shape

```yaml
app: Example

nav:
  GlobalNav:
    - Home

inherit:
  RootLayout:
    - Home

drill:
  Home: []

pages:
  Home:
    path: /
    nav: [GlobalNav]
```

## Rules

- `pages` is a map keyed by unique page id.
- a page may appear in at most one place in the `inherit` forest.
- a page may appear in at most one place in the `drill` forest.
- each `nav` entry is an ordered page id list.
- `page.nav` is a set or ordered list of nav ids.
- `groups` is optional and may overlap.

## Shorthand

`foo: [bar]` is shorthand for:

```yaml
foo:
  - bar
```

## Intended visualization

- inheritance forest
- drill-down forest
- nav overlay
- trait/group overlays

## Notes

- `inherit` answers: what does this page share?
- `drill` answers: what is this page drilling into?
- `nav` answers: where can this shared navigator take the user?
