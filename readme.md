# MRML

[![CircleCI](https://circleci.com/gh/jdrouet/mrml.svg?style=shield)](https://app.circleci.com/pipelines/github/jdrouet/mrml)
[![codecov](https://codecov.io/gh/jdrouet/mrml/branch/master/graph/badge.svg?token=L3LKpV3RpR)](https://codecov.io/gh/jdrouet/mrml)

## Introduction

This project is a reimplementation of the nice `MJML` markup language in Rust.

## Why?

- A `Node` server rendering a mjml template takes more than 30Mo of RAM. In `Rust`, less than 10Mo.
- The `JS` implementation cannot be run in the browser. In `Rust` (and `Wasm`), you can.

        The benchmark implementation will come.