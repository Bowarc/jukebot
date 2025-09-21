# Simple discord msuic bot

## Goal

The goal of this project is to make a simple discord bot able to play music from given sources

Meant to be deployed by anyone

## Status

MVP working

## Roadmap
- [ ] TODO

## Notes

Im currently using the songbird ytdlp implementation, if it gets too hard to deal with, I'll make my own, that what I planned at first anyway.

## Installation

### Docker install

#### Download the git repo

```console
git clone https://github.com/bowarc/jukebot
cd ./jukebot
```
#### Build it

```console
sh scripts/container_build.sh
```

#### Deploy it

```console
podman run -d --network host jukebot:latest
```
(should be the same command w/ docker)

### Manual install

#### First, download the projects with

```console
git clone https://github.com/bowarc/jukebot
cd ./jukebot
```

In the build script `./scripts/build*`, you can specify the command line argument `r` or `release` to build the project in release mode  
This will enable some optimisations but make the compilation a bit slower.

#### Init
Start by running `sh scripts/init.sh`  
This will create some important folders in the project directory, which the bot relies on.  

#### Run
To run the bot, use `sh scripts/run.sh`  

## Usage

TODO: talk about commands



