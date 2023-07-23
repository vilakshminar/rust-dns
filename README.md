# rust-dns

[![Coverage](https://github.com/vilakshminar/rust-dns/blob/gh-pages/badges/flat.svg)](https://github.com/vilakshminar/rust-dns)
[![Quality Gate Status](https://img.shields.io/badge/quality%20gate-passed-brightgreen)](https://github.com/vilakshminar/rust-dns)
[![Bugs](https://img.shields.io/badge/bugs-0-brightgreen)](https://github.com/vilakshminar/rust-dns)
[![Vulnerabilities](https://img.shields.io/badge/vulnerabilities-1-yellow)](https://github.com/vilakshminar/rust-dns)


## Description :scroll: :page_with_curl:

My own implementation of Julia Evans's [Implementing DNS in a weekend](https://implement-dns.wizardzines.com) project.

## Getting Started :clapper:

Clone the repository, ensure all the dependencies mentioned in the 'Dependencies' section are installed
and run the following command from the project root:
```bash
make all
```
The above make command will build, test and generate documentation for the project.

The generated doc will provide with enough details on the data structures and methods/functions created.

### Dependencies :computer: :keyboard:

* A Linux/MacOS machine.
* Rust >= 1.71.0

### Installing :yellow_circle:

Use rustup to install Rust & its dependencies. This project does not need anything else.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Executing program :white_check_mark:

In order to build & run the project:
```bash
make all
./target/debug/rust-dns
```

Running `make all` will build & test the app & will also generate docs for the project.

Running `./target/debug/rust-dns` will run the app.

## Help :sos: :information_source:

## Authors :man_technologist:

* Vineeth Lakshminarayanan - vinitlaks@gmail.com

## Version History

## License :copyright:
