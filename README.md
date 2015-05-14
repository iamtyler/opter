# opter

Turn a series of strings into options exposed as an iterator.

[![Build Status](https://travis-ci.org/iamtyler/opter.svg?branch=master)](https://travis-ci.org/iamtyler/opter)
[![](http://meritbadge.herokuapp.com/opter)](https://crates.io/crates/opter)
[![Coverage Status](https://coveralls.io/repos/iamtyler/opter/badge.svg)](https://coveralls.io/r/iamtyler/opter)

## Option

When parsed, input strings are determined to be one of two types: a name or a value. From the sequence of names and values, the type of option is determined. It is one of:
* **Flag** is a name without a value
* **Ordinal** is a value without a name
* **Named** is a value with a name
* **Raw** is simply a raw input string

### Names

Names come in two different types: short (1 `char`) and long (2+ `char`s). Long names start with `--` and short names with `-`. Short name Flags can be stacked, for example `-abc` represents the flags `a`, `b`, and `c`.

### Special Values

`-` alone is treated as a value. `--` signifies the end of options and is not emitted. All strings after `--` are passed along as a Raw option.

## Using opter

`opter` consumes iterators over strings, or `Iterator<Item = String>`. To use opter, pass a struct that has an implementation for `IntoIterator` to `opter::parse`. Parsing command-line options is a common use case that is met by `opter::parse_env`.
