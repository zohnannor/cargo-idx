# WIP

[![asciicast](https://asciinema.org/a/538980.svg)](https://asciinema.org/a/538980)

```bash
wget https://static.crates.io/db-dump.tar.gz -P db # download crates.io db dump
tar xvf db/db-dump.tar.gz -C db                    # extract tarball
# cargo install --path .                           # install binary, or
cargo b -r                                         # build to .target/release
# cargo-idx --help                                 # see help
./target/release/cargo-idx --help                  #
```

Fuzzy search:

```bash
# cargo-idx -S $(cargo-idx -Sql | fzf --preview 'cargo-idx -Si {}' --layout=reverse)
./target/release/cargo-idx -S $(./target/release/cargo-idx -Sql | fzf --preview './target/release/cargo-idx -Si {}' --layout=reverse)
```
