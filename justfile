ver := `grep '^version =' Cargo.toml | sed 's/version = "\(.*\)"/\1/'`

dev:
  nix develop ./nix/dev --command fish

build:
  cargo build --release

remote-run:
  nix run github:abhi-xyz/roxide -- help

push:
  cargo fmt --all -v 
  cargo build --release
  git add -A && git commit -m 'refacoring' && git push

build-dev:
  cargo build --release --features extra_commands

build-release:
  cargo build --release

install:
  cargo install --path .

release:
  cargo fmt --all -v 
  cargo build --release
  git tag v{{ver}}
  git add -A && git commit -m 'new release' && git push
  git push --tags
  cargo publish
