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
ext install rust-lang.rust-analyzer
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

cargo init
    2  cargo build
    3  apt update
    4  sudo apt update
    5  sudo apt install pkg-config
    6  cargo build
    7  sudo apt install fontconfig
    8  sudo apt upgrade
    9  history 

yeslogic-fontconfig-sys
https://crates.io/crates/yeslogic-fontconfig-sys 

[First time with PKG_CONFIG_PATH](https://forums.debian.net/viewtopic.php?t=159260)

-  Building with cargo fails, unable to run custom build command for servo-fontconfig-sys #5564 
-- https://github.com/alacritty/alacritty/issues/5564

- https://github.com/alacritty/alacritty/blob/master/INSTALL.md#dependencies
apt install cmake pkg-config libfreetype6-dev libfontconfig1-dev libxcb-xfixes0-dev libxkbcommon-dev python3