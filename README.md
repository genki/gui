# gui

Declarative `.gui` format for modeling GUI structure with two forests:

- `inherit`: layout/navigation/capability inheritance
- `drill`: information-space drill-down

The format treats `nav` as a first-class shared component.

- a `nav` is a named ordered target page list
- a `page` directly declares the navs it exposes
- many apparent transitions are derived from `drill` and `nav`

## Example

```gui
app: Demo

nav:
  GlobalNav:
    - Home
    - Products
    - AdminRoot

  ProductTabs:
    - ProductDetail
    - ProductReviews

inherit:
  RootLayout:
    - Products
    - AdminRoot:
        - AdminUsers

drill:
  Home:
    - Products:
        - ProductDetail:
            - ProductReviews
    - AdminRoot:
        - AdminUsers

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
