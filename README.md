# gui

Declarative `.gui` format for modeling GUI structure with two forests:

- `inherit`: layout/navigation/capability inheritance
- `drill`: information-space drill-down

The format treats `nav` as a first-class shared component.

- a `nav` holds a target page set
- a `page` acquires navs through `traits`
- many apparent transitions are derived from `drill` and `nav`

## Example

```gui
app: Demo

nav:
  - id: GlobalNav
    targets: [Home, Products, AdminRoot]

  - id: ProductTabs
    targets: [ProductDetail, ProductReviews]

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
  - id: Home
    path: /
    traits: [GlobalNav]

  - id: Products
    path: /products
    traits: [GlobalNav]

  - id: ProductDetail
    path: /products/:id
    traits: [GlobalNav, ProductTabs]
```

## Repository layout

- `examples/`: sample `.gui` files
- `spec/`: draft format specification

## Status

This repository currently contains an initial draft only.
