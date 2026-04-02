# gui

Declarative `.gui` format for modeling GUI structure with two forests:

- `inherits-from`: layout/navigation/capability inheritance
- `drilldown-from`: information-space drill-down

The format also supports:

- `traits`: cross-cutting shared navigators or behaviors
- `transitions`: screen-to-screen navigation edges
- `groups`: optional analytical grouping

## Example

```gui
app: Demo

traits:
  - GlobalNav
  - AuthRequired

pages:
  - id: Home
    title: Home
    path: /
    traits: [GlobalNav]

  - id: Products
    title: Products
    path: /products
    drilldown-from: Home
    traits: [GlobalNav]

  - id: ProductDetail
    title: Product Detail
    path: /products/:id
    inherits-from: Products
    drilldown-from: Products
    traits: [GlobalNav]

transitions:
  - from: Home
    to: Products
  - from: Products
    to: ProductDetail
```

## Repository layout

- `examples/`: sample `.gui` files
- `spec/`: draft format specification

## Status

This repository currently contains an initial draft only.
