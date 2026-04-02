# gui

Declarative `.gui` format for modeling GUI structure with two forests:

- `drill`: information-space drill-down
- `inherit`: layout/navigation/capability inheritance

The format treats `nav` as a first-class shared component.

- a `nav` is a named ordered target page list
- a `page` directly declares the navs it exposes
- every page must appear exactly once as a leaf in `inherit`
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

pages:
  Home:
    path: /
    nav: [GlobalNav]

  Products:
    path: /products
    nav: [GlobalNav]

  ProductDetail:
    path: /products/:id
    nav: [GlobalNav, ProductTabs]
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
