## env-replace

[![Build Status](https://travis-ci.org/nicompte/env-replace.svg?branch=master)](https://travis-ci.org/nicompte/env-replace)
[![Build status](https://ci.appveyor.com/api/projects/status/i2i970v5aekca82f?svg=true)](https://ci.appveyor.com/project/nicompte/env-replace)

`env-replace` is a Q&D tool that replaces patterns found in files by environment variable values.

For example, for the given conf.yaml file:

```
host: %%my_host%%
```

`env-replace "(%%([\w_]+)%%)" conf.yaml`, will replace `%%my_host%%` by the value of the `my_host` environment variable, or the variable defined in a `.env` file.

It will not write the file if any of the found variable isn't defined.

### options

```
USAGE:
    env-replace [FLAGS] <REGEX> <INPUT> [OUTPUT]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose, will display found variables

ARGS:
    <REGEX>     Regex e.g. "(%%([\w_]+)%%)"
    <INPUT>     Input file
    <OUTPUT>    Output file, defaults to the input file
```

### build

- you will need a rust compiler, visit [rustup.sh](https://rustup.sh).
- `make build` will produce a binary in `target/release`