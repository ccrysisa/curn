# mini curn

minicurn is a mini container written in C.

## Prerequisites

```sh
$ docker run -d --rm --name ubuntu-fs ubuntu:20.04 sleep 1000
$ docker export ubuntu-fs -o ubuntu-fs.tar
$ docker stop ubuntu-fs
$ mkdir -p /path/to/ubuntu-fs
$ tar xf ubuntu-fs.tar -C /path/to/ubuntu-fs/
```

## Quck start

```sh
$ make
$ sudo ./minicurn run /bin/bash
```
