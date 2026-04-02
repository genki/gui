# `.gui` draft format

## Core model

Each page is a node with up to two parent relations:

- `inherits-from`: visual or behavioral inheritance
- `drilldown-from`: semantic drill-down in the information space

Each page may also have:

- `traits`: reusable shared navigators, layouts, or capabilities
- `transitions`: navigation edges between pages
- `groups`: optional clusters for analysis or visualization

## Minimal shape

```yaml
app: Example

traits:
  - GlobalNav

pages:
  - id: Home
    title: Home
    path: /
    traits: [GlobalNav]

transitions:
  - from: Home
    to: Home
```

## Rules

- `id` must be unique within a file.
- `inherits-from` is optional and singular.
- `drilldown-from` is optional and singular.
- `traits` is a set.
- `transitions` is a list of directed edges.
- `groups` is optional and may overlap.

## Intended visualization

- inheritance forest
- drill-down forest
- transition graph
- trait/group overlays
