# MRML

[![CircleCI](https://circleci.com/gh/jdrouet/mrml.svg?style=shield)](https://app.circleci.com/pipelines/github/jdrouet/mrml)
[![codecov](https://codecov.io/gh/jdrouet/mrml/branch/master/graph/badge.svg?token=L3LKpV3RpR)](https://codecov.io/gh/jdrouet/mrml)

## TODO

- Testing
  - [ ] compare properly the generated HTML
    - [x] not take in account empty class/style attributes
    - [ ] not care about orders of attributes
- CI
  - [ ] add code coverage
  - [ ] automatic deploy to crates.io
- Core
  - [x] expose the `to_html` method
  - [ ] add options to minify/not minify
  - [ ] clean by removing consecutive conditions
- components
  - [ ] mjml
    - [x] without attributes
    - [ ] with owa
    - [ ] with lang
  - [x] mj-head
  - [ ] mj-attributes
  - [x] mj-breakpoint
  - [x] mj-font
  - [x] mj-preview
  - [ ] mj-style
    - [x] without attributes
    - [ ] with inline
  - [x] mj-title
  - [x] mj-body
    - [x] without attributes
    - [x] with background-color
    - [x] with css-class
    - [x] with width
  - [ ] mj-accordion
  - [ ] mj-button
    - [x] without attributes
    - [x] documentation example
    - [x] with align (default: center)
    - [x] with background-color (default: #414141)
    - [x] with border, border-(top|right|bottom|left)
    - [x] with border-radius (default: 3px)
    - [x] with color (default: #ffffff)
    - [x] with container-background-color
    - [x] with css-class
    - [x] with font-family (default: Ubuntu, Helvetica, Arial, sans-serif)
    - [x] with font-size (default: 13px)
    - [x] with font-style
    - [x] with font-weight (default: normal)
    - [x] with height
    - [x] with href
    - [x] with inner-padding (default: 10px 25px)
    - [x] with line-height (default: 120%)
    - [x] with padding, padding-(top|right|bottom|left) (default: 10px 25px)
    - [x] with rel
    - [x] with target (default: _blank)
    - [ ] with text-align
    - [x] with text-decoration
    - [x] with text-transform
    - [x] with vertical-align (default: middle)
    - [x] with width
  - [ ] mj-carousel
  - [x] mj-column
    - [x] without attributes
    - [x] with background-color
    - [x] with border, border-(top|right|bottom|left)
    - [x] with border-radius
    - [x] with css-class
    - [x] with padding, padding-(top|right|bottom|left)
    - [x] with vertical-align
    - [x] with width (default: (100 / number of non-raw elements in section)%)
  - [ ] mj-divider
  - [ ] mj-group
  - [ ] mj-hero
  - [ ] mj-image
    - [x] mjml documentation example
    - [x] with align
    - [x] with border
    - [x] with border-radius
    - [x] with container-background-color
    - [x] with css-class
    - [x] with fluid-on-mobile
    - [x] with height
    - [x] with href
    - [x] with padding, padding-(top|right|bottom|left)
    - [x] with rel
    - [x] with src
    - [ ] with srcset
    - [x] with target
    - [x] with title
    - [x] with width
  - [ ] mj-navbar
  - [ ] mj-raw
  - [x] mj-section
    - [x] without attributes
    - [x] with background-color
    - [x] with background-repeat (default: repeat)
    - [x] with background-size (default: auto)
    - [x] with background-url
    - [x] with border, border-(top|right|bottom|left)
    - [x] with border-radius
    - [x] with css-class
    - [x] with direction (default: ltr)
    - [x] with full-width
    - [x] with padding (default: 20px 0)
    - [x] with padding-(top|right|bottom|left)
    - [x] with text-align
  - [ ] mj-social
  - [ ] mj-spacer
  - [ ] mj-table
  - [ ] mj-text
    - [x] without attributes
    - [x] mjml documentation example
    - [x] with color (default: #000000)
    - [x] with font-family (default: Ubuntu, Helvetica, Arial, sans-serif)
    - [x] with font-size (default: 13px)
    - [x] with font-style
    - [x] with font-weight
    - [x] with line-height (default: 1)
    - [x] with letter-spacing (default: none)
    - [x] with height
    - [x] with text-decoration
    - [x] with text-transform
    - [x] with align (default: left)
    - [x] with container-background-color
    - [x] with padding (default: 10px 25px), padding-(top|right|bottom|left)
    - [x] with css-class
  - [ ] mj-wrapper
