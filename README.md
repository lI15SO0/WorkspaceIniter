# wsinit - an easy way to init workspace in cli.

---

These tools can package workspace as a bincode file, to make create project more easier.

### mkwsconfig

This tool can read current dir and package it as bincode file, and save it to dot files direction.

### wsinit

This tool can release bincode files, which created by mkwsconfig cmd.

---

## Install

Clone this project into local.

``` sh
git clone https://github.com/lI15SO0/WorkspaceIniter --depth 1
```

Compile this project.

``` sh
cd WorkspaceIniter
cargo build --release
```

Copy binary into user's path dir.

``` sh
mkdir -p ~/.local/bin/
cp ./target/release/wsinit ./target/release/mkwsconfig ~/.local/bin/
```

or system path dir.

``` sh
cp ./target/release/wsinit ./target/release/mkwsconfig /usr/local/bin/
```

--- 

## Usage

Use "mkwsconfig" to generate current dir's bincode.

example:

``` sh
mkwsconfig -c example -r
```

Use command "wsinit" to build up workspace via bincode.

``` sh
wsinit -c example
```

If workspace has "init.sh" file, and os had install "sh" or "bash".

Then wsinit will run "init.sh" via "sh" command, after build up.

Or can cancle to run "init.sh" by "--no-init" arg.

example:

https://asciinema.org/a/QkIiYxTOUOYM4wSGgZHGFdBRN

### mkwsconfig

```
Save current dir as a profile file

Usage: mkwsconfig [OPTIONS]

Options:
  -n, --name <NAME>  Profile name
  -f, --force        Force create profile
  -r, --raw          Allow empty files
  -h, --help         Print help
  -V, --version      Print version
```

### wsinit

```
Init workspace by profile file

Usage: wsinit [OPTIONS]

Options:
  -c, --profile <PROFILE>  Name of profile
  -d, --target <TARGET>    Where to init. (default: ./)
  -p, --print              Show the files and dirs what will be create
  -f, --force              Force mode
  -l, --list               List of profiles
  -s, --set-default        Set default profile
  -h, --help               Print help
  -V, --version            Print version
```

---
