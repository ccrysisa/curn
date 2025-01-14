# cunrc

cunrc is a container written in Rust, and it is inspired by [curn](https://github.com/ccrysisa/curn).

## Quick Start

Make sure you have installed docker:

```sh
$ docker run -d --rm --name ubuntu-fs ubuntu:20.04 sleep 1000
$ docker export ubuntu-fs -o ubuntu-fs.tar
$ docker stop ubuntu-fs
$ mkdir -p ./ubuntu-fs
$ tar xf ubuntu-fs.tar -C ./ubuntu-fs/
$ ./test.sh
```
