# webserver
![GitHub repo size](https://img.shields.io/github/repo-size/EdoardoLaGreca/webserver)
![GitHub last commit](https://img.shields.io/github/last-commit/EdoardoLaGreca/webserver)
![Lines of code](https://img.shields.io/tokei/lines/github/EdoardoLaGreca/webserver)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/EdoardoLaGreca/webserver/Rust)  
The name is temporary, I'm thinking of an original one...

## Why?
I built this webserver for 3 reasons:
  - I needed a minimal and fast webserver that gets the job done.
  - I don't trust big projects as there is a consistent chance of running bloatware or overkilling the whole thing by using less than half features of a full-featured software.
  - I want to write minimal and lightweight webpages in markdown and let the server "compile" them into HTML in real time.
  
## Features
  - [x] A thread limit
  - [ ] A cache system which stores the most requested files in a directory (as HTML).
  - [ ] An update method, either by uploading the repo on the crates registry or some package manager servers or by implementing a custom self-updater.
  - [ ] A template used to give instructions about the creation of the HTML structure (inspired by HTML custom tags).

## TODOs
  - [ ] Write a better documentation by using the repository wiki (on GitHub) 
  - [ ] Choose a better name for the repository

## Requirements
### Operating systems
Note that almost any operating system is supported by building from source. The list below specifies which operating systems the prebuilt binaries will be available for on releases.
  - Windows (Windows 7+)
  - macOS (10.7+, Lion+)
  - Linux (kernel 2.6.32+)
  - FreeBSD

### Hardware
The hardware specified is _recommended_, not required.
  - Microprocessor architecture: i686 or x86_64. No specific speed or core number, the more the better. May still compile on other architechtures.
  - At least 1GiB of available RAM.
  - At least 512MiB of available space in disk.

## Compile & run
***It is advised to run the server using the latest release, not the cloned repository. Use the following procedure only if you're interested in either the latest features or the development and contribution since it may be very unstable or it may not even compile at all.***  

Dependencies:
 - Cargo: the Rust Language package manager. It can be installed through [Rustup](https://rustup.rs) or by using a package manager.

Other dependencies are automatically downloaded, compiled and built by Cargo during compile-time.  
If you're on Linux or a Unix-like OS (such as \*BSD) you should be able to install the dependencies using your OS package manager. No guarantees on Windows or other OSes.

To generate the executable (optimized), use the command below in the repository's root directory.
```
cargo build --release --target-dir bin
```

By running it, you generate an executable (in the `bin/` directory) which you can run through this command.
```
./bin/release/webserver
```

For a help page, type this.
```
./bin/release/webserver -h
```

## Files and directories
Coming soon...

### `config.toml`
This file provides a simple and intuitive way to set up the webserver. An example can be found [here](https://github.com/EdoardoLaGreca/webserver/blob/main/config.toml). This file is loaded on server startup, modifying the file without restarting the server won't make any change effective.

#### `address`
This is the address that the webserver will listen on. You may want to change the port in order to avoid specifying it every time you type the address (port 80 for HTTP, port 443 for HTTPS).

#### `threads`
Since the webserver uses a thread pool to provide content in an efficient way, you may want to specify the number of threads based on need. More threads means faster content delivery but more memory consumption.

#### `err404_path`
You may want to change the error 404 page name and path, here you can do it. The root directory is `www/`.

#### `title`
Here you specify the website title, it will be displayed through the `<title>` tag on each requested page.

#### Default values
This section contains all the default values that will be used in case no data is provided.

For more info, have a look at `src/defaults.rs`.

## Contribution
The main way to contribute is through issues.  
If you think that something is missing or some things may be better written in another way, open an issue.  
If you think that the documentation contains errors or is not clear, open an issue.  
If you think that anything of any kind can be improved, open an issue.  

I'm not a Git expert so no pull requests for now.

## License
This repository uses a BSD-3-Clause license. More info [here](https://github.com/EdoardoLaGreca/webserver/blob/main/LICENSE).
