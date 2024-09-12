# WizCtl

WizCtl is a simple command line utility to controll Wiz Connected LED light strips.

## How to install?
You can easily install this tool using `cargo install wizctl`.

## How to build?
This tool requires you to have a decently up to date version of Rust (>= 1.80.0) installed.

```bash
$ git clone https://github.com/jooris-hadeler/wizctl.git
$ cd wizctl
$ cargo build --release
```

## How to use?
In order to talk to your LED light strip you need to know its IP address, 
this is usually easy to find in your routers web panel.

Once you know the IP address you can talk to the light strip by using the `-i <IP>` flag, 
this also works for multiple light strips.

Here is an example to set the light strip to the warm white dynamic scene:
```bash
$ wizctl -i <IP> dynamic warm-white
```