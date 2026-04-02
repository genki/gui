# gui

Declarative `.gui` format for modeling GUI structure with two forests:

- `drill`: information-space drill-down
- `inherit`: layout/navigation/capability inheritance

The format treats `nav` as a first-class shared component.

- a `nav` is a named ordered target page list
- any `node` may declare attributes such as `path` or `nav`
- those attributes are inherited through `inherit`
- every `inherit` leaf is a page
- every node that appears in `drill` is a page
- non-leaf `inherit` nodes may be layouts or shells
- many apparent transitions are derived from `drill` and `nav`

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

nodes:
  RootLayout:
    nav: [GlobalNav]

  AdminShell:
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

## Status

This repository currently contains an initial draft only.
