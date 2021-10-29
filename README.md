# ProCurve 1810G Switch Command Line Interface

<!-- badges -->
![continuous integration status](https://github.com/bfritz/procurve-cli/actions/workflows/ci.yaml/badge.svg)

## Preface

**Beware:** The code in this repo is all experimental.  One of the primary
goals is to gain experience with [Rust].  Consider the code alpha quality and
assume that it is poorly written and not idiomatic.

## What

Work-in-progress command line interface to HP ProCurve 1810G switches.  Works
by wrapping the web mangement interface.

## How

Assuming:

* [make] along with [Cargo and Rust] are already installed,
* your switch web interface is at http://192.168.3.4/ , and
* you are on Linux, or something, system:

```sh
SWITCH_URL=http://192.168.3.4 make run-rust
```

```
+--------------+-----------------------------------------------------+
| Description  | HP ProCurve 1810G - 8 GE, P.2.22, eCos-2.0, CFE-2.1 |
+--------------+-----------------------------------------------------+
| Name         | PROCURVE J9449A                                     |
+--------------+-----------------------------------------------------+
| Location     |                                                     |
+--------------+-----------------------------------------------------+
| Contact      |                                                     |
+--------------+-----------------------------------------------------+
| Version      | P.2.22                                              |
+--------------+-----------------------------------------------------+
| Object ID    | 1.3.6.1.4.1.11.2.3.7.11.103                         |
+--------------+-----------------------------------------------------+
| Uptime       | 3 days, 2 hours, 45 mins, 38 secs                   |
+--------------+-----------------------------------------------------+
| Current Time | 02:45:38                                            |
+--------------+-----------------------------------------------------+
| Current Date | 01/04/1970                                          |
+--------------+-----------------------------------------------------+
```

## Why

The 1810G web interface is clunky and slow, especially when working with VLAN
configuration.  It would be convenient to have a faster way to dump the VLAN
configuration and review other settings.  Adjusting the VLAN configuration is
a stretch goal.  Also to improve my knowledge of Rust.


[cargo and rust]: https://www.rust-lang.org/tools/install
[make]: https://www.gnu.org/software/make/
[rust]: https://www.rust-lang.org/
