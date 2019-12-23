<div align="center"><img src="./images/logo.png" height="50px"/><br/>
<a href="https://www.recurse.com/scout/click?t=547ff0b401bdb9be36cf2425204017cc" title="Made with love at the Recurse Center"><img src="https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png" height="20px"/></a> <a href="http://newschool.edu"><img src="https://img.shields.io/badge/made%20at-The%20New%20School-E82E21.svg" height="20px"/></a><br/><a href="https://travis-ci.org/zachkrall/artloop" title="Travis CI Build"><img src="https://img.shields.io/travis/zachkrall/artloop.svg" height="20px"/></a> <a href="https://github.com/zachkrall/artloop/issues/"><img src="https://img.shields.io/github/issues/zachkrall/artloop.svg" height="20px"/></a> <a href="https://github.com/zachkrall/artloop/commits"><img src="https://img.shields.io/github/last-commit/zachkrall/artloop.svg" height="20px"/></a></div>

## about

artloop is a command-line interface that cycles through all generative art applications within a folder on a timed interval.

artloop currently only works on macOS due to how the script looks for executables inside of `*.app/Contents/MacOS/*`.

## installation

with `cargo`:
```shell
cargo install artloop
```

with `npm`:
```shell
npm install artloop
```

to confirm artloop is installed, you can run `which artloop` in your terminal.


## usage 

to start artloop, provide a relative folder location and a number of minutes each artwork should run.

```shell
artloop <FOLDER> --time <MINUTES>
# example: artloop ./content --time 10
```

artloop will run until the script is canceled with `CTRL + C` (^C)

## running automatically at startup

to have artloop run at startup, add a line to `$HOME/.bash_profile` which calls the artloop script and supplies a relative folder path containing applications.

## participate

### creating compatible artwork

For now, any creative coding project (e.g. openFrameworks, Processing, Unity) that is bundled as a macOS application package (i.e. a file that ends in `.app`) will work with artloop.

## license
Copyright © 2019 [Zach Krall](https://zachkrall.com)<br/>This project is [MIT](https://github.com/zachkrall/artloop/blob/master/LICENSE) licensed.
