# `.gui` draft format

## Core model

Each GUI model is defined by four main parts:

- `inherit`: a forest for visual or behavioral inheritance
- `drill`: a forest for semantic drill-down in the information space
- `nav`: named ordered target page lists
- `pages`: page objects that acquire navs and other shared components through `traits`

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
  - id: Home
    path: /
    traits: [GlobalNav]
```

## Rules

- `page.id` must be unique within a file.
- a page may appear in at most one place in the `inherit` forest.
- a page may appear in at most one place in the `drill` forest.
- each `nav` entry is an ordered page id list.
- `traits` is a set of shared component ids, including nav ids.
- `groups` is optional and may overlap.

## Intended visualization

- inheritance forest
- drill-down forest
- nav overlay
- trait/group overlays

## Notes

- `inherit` answers: what does this page share?
- `drill` answers: what is this page drilling into?
- `nav` answers: where can this shared navigator take the user?
