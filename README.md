# project

## make project folder

```bash
cd \
&& mkdir rust_project \
&& cd $_ 
```

## init project - enviroment  raspi debian12 

```bash
touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup  show \
&& rustup  check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& rustup show \
&& touch FROM_HERE.md \ 
&& cargo build 
```

## add ext for vscode 

- press F1 
```
ext install esbenp.prettier-vscode
ext install yzhang.markdown-all-in-one
```

## cold rust update / renew

```bash
# cold update 
# https://github.com/rust-lang/rustup/issues/2729
# rustup toolchain uninstall stable
# rustup toolchain install stable
```

## add crates

[plotters](https://crates.io/crates/plotters)
```bash
# 
cargo add plotters
cargo build
```