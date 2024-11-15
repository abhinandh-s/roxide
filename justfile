ver := `grep '^version =' Cargo.toml | sed 's/version = "\(.*\)"/\1/'`

dev:
  nix develop ./nix/dev --command fish

build:
  cargo build --release

remote-run:
  nix run github:abhi-xyz/rid -- help

release:
  git tag v{{ver}}
  git add -A && git commit -m 'new release' && git push
  git push --tags

