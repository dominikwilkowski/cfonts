```sh
 █▀█ █▀▀ █   █▀▀ ▄▀█ █▀▀ █▀▀
 █▀▄ ██▄ █▄▄ ██▄ █▀█ ▄▄█ ██▄ cfonts
```
> To release a new version of cfonts follow the below steps

## NodeJS

Just publish via npm:

```sh
λ npm publish
```

And tag the release with `vx.x.xnodejs`.

## Rust

### git

Make sure you tag each release with `vx.x.xrust`.

### Crates.io

We have to use the flag `--allow-dirty` since we include build artifacts like `fonts/*` and `/LICENSE`

```sh
λ cargo publish --allow-dirty
```

### Homebrew

https://docs.brew.sh/Manpage#bump-formula-pr-options-formula

Fork https://github.com/Homebrew/homebrew-core and push a new version.

`cd` into the folder and run below command.

Download the `tar.gz` file from the GitHub release tag and run `openssl dgst -sha256 [filepath].tar.gz` and copy the checksum.

```sh
λ brew bump-formula-pr cfonts --url=https://github.com/dominikwilkowski/cfonts/archive/refs/tags/v[version]rust.tar.gz --sha256=[checksum] --dry-run
```

### AUR

Repo: https://aur.archlinux.org/cfonts.git

Reference: https://wiki.archlinux.org/title/PKGBUILD

Download the release zip from GitHub (`https://github.com/dominikwilkowski/cfonts/archive/refs/tags/v[version].tar.gz`) and run:

```sh
λ openssl dgst -sha256 path/to/zip
```

Edit the `PKGBUILD` file:

```diff
- pkgver=1.1.0rust
- pkgrel=3
- sha256sums=('45c40dfc867234efc5c5a2df687ccfc40a6702fa5a82f2380b555f9e755508e6')
+ pkgver=1.2.0rust
+ pkgrel=1
+ sha256sums=('0d2671b410aeddc23345b984984d70fa7da22c1df9a218bb3e9711d571a8d4ad')
```

Edit the `.SRCINFO` file:

```diff
-     pkgver = 1.1.0rust
-     pkgrel = 3
-     source = https://github.com/dominikwilkowski/cfonts/archive/refs/tags/v1.1.0rust.tar.gz
-     sha256sums = 45c40dfc867234efc5c5a2df687ccfc40a6702fa5a82f2380b555f9e755508e6
+     pkgver = 1.2.0rust
+     pkgrel = 1
+     source = https://github.com/dominikwilkowski/cfonts/archive/refs/tags/v1.2.0rust.tar.gz
+     sha256sums = 0d2671b410aeddc23345b984984d70fa7da22c1df9a218bb3e9711d571a8d4ad
```

Note: `pkgrel` starts with `1`

### Macports

Fork https://github.com/macports/macports-ports and push a new version.
Reference PR: https://github.com/macports/macports-ports/pull/15303 and https://github.com/macports/macports-ports/pull/15130

Run `cargo2port` in the rust folder of the cfonts repo:

```sh
λ cargo2port
cargo.crates \
    something             x.x.x  fa3d466004a8b4cb1bc34044240a2fd29d17607e2e3bd613eb44fd48e8100da3 \
    [...]
    something             x.x.x  f40009d85759725a34da6d89a94e63d7bdc50a862acf0dbc7c8e488f1edcb6f5
```

and copy the result into the `textproc/cfonts/Portfile` of the macports fork.

Also change `github.setup` and `checksums` inside the `Portfile` file:

```diff
- github.setup        DominikWilkowski cfonts 1.0.4 v rust
+ github.setup        DominikWilkowski cfonts 1.1.0 v rust
```

And

```diff
checksums           ${distfiles} \
-                     rmd160  9bc402d9c18048175ff25079cdbd60c31436ccd8 \
-                     sha256  efe44d74c2a4a9510413ee7429bf248e3bee4e1ae5da17c4ad8b96c1d8606dc1 \
-                     size    3313925
+                     rmd160  cd1099abdbf581dd4bd3a1a0bd156ee3f9a6e953 \
+                     sha256  edd9a1f4ce246eee3f10f084d72e4e650bd68539d8bc8f9e4eca61f2f9c79293 \
+                     size    3323439
```

To get checksums download the zip from `https://codeload.github.com/DominikWilkowski/cfonts/legacy.tar.gz/v[version]rust?dummy=` and run:

```sh
λ openssl dgst -rmd160 path/to/zip
λ openssl dgst -sha256 path/to/zip
λ stat -f%z path/to/zip
```

Lint the Portfile via:

```sh
λ port lint --nitpick textproc/cfonts
```

And finally test it all via:

```sh
λ sudo port test textproc/cfonts
λ sudo port -vst install textproc/cfonts
λ sudo port uninstall cfonts
```

Make sure you format the git message as `cfonts: update to 1.1.0`.


Copyleft (c) 2023 Dominik Wilkowski.
Licensed under the [GNU GPL-3.0-or-later](https://github.com/dominikwilkowski/cfonts/blob/released/LICENSE).
