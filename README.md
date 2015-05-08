# opter

Turn a series of strings into options exposed as an iterator.

## Option

An option is one of four flavors:
* Flag is a name without a value
* Ordinal is a value without a name
* Named is a value with a name
* Value is simply a raw value

Full names start with "--" and short names with "-".
Short name flags can be stacked, for example "-abc" represents the flags "a", "b", and "c".
"-" alone is treated as a value.
"--" signifies the end of options and is not emitted. All strings after "--" are passed along as a Value.
