# curn

cunrc is a container written in Rust.

## Prerequisites

Make sure you have installed docker:

```sh
$ docker run -d --rm --name ubuntu-fs ubuntu:20.04 sleep 1000
$ docker export ubuntu-fs -o ubuntu-fs.tar
$ docker stop ubuntu-fs
$ mkdir -p ./ubuntu-fs
$ tar xf ubuntu-fs.tar -C ./ubuntu-fs/
```

## Quick Start

```sh
$ ./run.sh
```
