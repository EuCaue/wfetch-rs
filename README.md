# wfetch-rs

# Wfetch-rs is a Rust command-line application for obtaining basic weather information

## How to install

### Binary

Go to [release page](https://github.com/EuCaue/wfetch-rs/releases) download the wfetch .tar.gz

Extract the .tar.gz and move the wfetch to your path (eg: /usr/bin/)

### Compile/Manual

#### Warning for manual install, you will need a apikey from the openweatherapi, or use the example one in the config.toml

Clone the repo:

```sh
git clone https://github.com/EuCaue/wfetch-rs.git
```

Change directory:

```sh
cd wfetch-rs/
```

(OPTIONAL) Change the apikey in config.toml or use the example apikey:

```sh
$EDITOR config.toml
```

Compile:

```sh
cargo build --release
```

Change directory and run:

```sh
cd target/release && ./wfetch
```

After that, you can move the wfetch binary, to your path, for a better experience. (eg: /usr/bin/)

## Contributing

Clone the repo:

```sh
git clone https://github.com/EuCaue/wfetch-rs.git
```

Change directory:

```sh
cd wfetch-rs/
```

After make the changes, open PR. :)
