# gui

Declarative `.gui` format for modeling GUI structure with two forests:

- `drill`: information-space drill-down
- `inherit`: layout/navigation/capability inheritance

The format treats `nav` as a first-class shared component.

- a `nav` is a named target page set
- any `node` may declare attributes such as `path` or `nav`
- those attributes are inherited through `inherit`
- every `inherit` leaf is a page
- every node that appears in `drill` is a page
- non-leaf `inherit` nodes may be layouts or shells
- scalar attributes override inherited values
- vector attributes merge by set union
- many apparent transitions are derived from `drill` and `nav`

In this abstract language, `nav` is unordered. Concrete UI layers may choose an
ordering or spatial arrangement such as tabs, side menus, or ring menus.

## Example

```gui
app: Demo

drill:
  Home:
    - Products:
        - ProductDetail:
            - ProductReviews
    - AdminRoot:
        - AdminUsers

inherit:
  RootLayout:
    - Home
    - Products
    - AdminShell:
        - AdminRoot
        - AdminUsers

nav:
  GlobalNav:
    - Home
    - Products
    - AdminRoot

  ProductTabs:
    - ProductDetail
    - ProductReviews

node:
  RootLayout:
    nav: [GlobalNav]

  Home:
    path: /

  Products:
    path: /products

  ProductDetail:
    path: /products/:id
    nav: [ProductTabs]
```

`foo: [bar]` is shorthand for:

```gui
foo:
  - bar
```

## Repository layout

- `examples/`: sample `.gui` files
- `spec/`: draft format specification

## CLI

```sh
cargo run -- check examples/demo.gui
cargo run -- pages examples/demo.gui
```

## Status

This repository currently contains an initial draft only.
