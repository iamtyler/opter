# opter

Turn a series of strings into options exposed as an iterator.

## Option

When parsed, input strings are determined to be one of two types: a name or a value. From the sequence of names and values, the type of option is determined. It is one of:
* **Flag** is a name without a value
* **Ordinal** is a value without a name
* **Named** is a value with a name
* **Value** is simply a raw input string

### Names

Names come in two different types: short and long names. Long names start with `--` and short names with `-`. Short name Flags can be stacked, for example `-abc` represents the flags `a`, `b`, and `c`.

### Special Values

`-` alone is treated as a value. `--` signifies the end of options and is not emitted. All strings after `--` are passed along as a Value option.
