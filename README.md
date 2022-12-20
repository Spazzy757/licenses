# License Checks

## language roadmap

- [x] Golang (requires a go.mod file in the directory you run it in)
- [ ] Rust
- [ ] Javascript
- [ ] Java
- [ ] Python
- [ ] Ruby

## Why?

There is sometimes a need to validate that your dependencies use specific
licenses (this is generally in more regulated companies). There is also the
issues of using software in a way that might not be allowed under certain
licenses.

## Alternatives

There is a much more feature rich [license checker built by
Pivotal](https://github.com/pivotal/LicenseFinder). It works really well,
I just don't like having to install ruby, I find the config files
really untidy, and the remote config file setup is pretty weird so I decided to build my own, also i've been looking to learn rust so hear we are.

## Usage

```bash
Usage: licenses [OPTIONS] --config <CONFIG>

Options:
  -r, --remote           Remote bool to determine if stored in a git repo
  -t, --token <TOKEN>    Token string for remote config, will set the header Authorization: Bearer $token [default: ]
  -c, --config <CONFIG>  Config path or URL to config file
  -h, --help             Print help information
  -V, --version          Print version information
```
> Currently for logged output other than errors you need to specify RUST_LOG=info

## Example Config File

```yaml
# These are licenses you allow
allowed_licenses:
- MIT
- New BSD
- Apache 2.0
- Simplified BSD
- Mozilla Public License 2.0
- ISC
- BSD 3 Clause
- Apache 2.0, MIT
# These are specific packages you allow
whitelisted_dependencies:
- github.com/go-sql-driver/mysql
```

## Development

### Install depenedencies
```bash
cargo build
```

### Run Tests
```bash
cargo test
```
