lud
===

*l*ook *u*p a *d*omain

A command-line DNS client inspired by dig, written in rust, a hobby project.
Unlike dig, lud conforms to the POSIX flag specification for its invocation.

Because lud is in part intended for educational value, all information in the reply is printed automatically including additional or nameserver records.

Status
--

  - [x] basic interface: sends and parses (some) DNS Messages
  - [x] nice display for A records
  - [x] nice display for the most popular RRTypes
  - [x] allow the user to specify the resolver
  - [ ] allow the user to specify query flags
  - [ ] nice display for all other RRTypes
  - [ ] support EDNS
  - [ ] all the RFCs, all the edge cases
  - [ ] take over the world

Usage
--

```console
$ lud --help
lud 0.2.0
Joshua Crowgey
DNS Lookup Client

USAGE:
    lud [OPTIONS] <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -q, --qtype <qtype>      what are you asking
    -s, --server <server>    which DNS server to use

ARGS:
    <name>    what to look up
```

By default, `lud` sends a request for A records to the
resolver defined in your /etc/resolv.conf with the recursion
desired flag set.  You may change the requested RR type or
resolver using the optional flags.

Examples
--

Default behavior: using the locally configured resolver to
ask for an A record for example.com.

```console
$ lud example.com
ID: 3597
QR: R; Opcode: 0
FLAGS: AA false; TC false; RD: true; RA: true; Z: 0; NoError
QDCOUNT 1; ANCOUNT 1; NSCOUNT 0; ARCOUNT 0

Question
example.com	QTYPE: A; CLASS: 1

Answer
example.com	A	IN	TTL: 11248, RDLEN: 4
93.184.216.34
```

Asking for a SOA record looks like this:

```console
$ lud example.com -q SOA
ID: 9594
QR: R; Opcode: 0
FLAGS: AA false; TC false; RD: true; RA: true; Z: 0; NoError
QDCOUNT 1; ANCOUNT 1; NSCOUNT 0; ARCOUNT 0

Question
example.com	QTYPE: SOA; CLASS: 1

Answer
example.com	SOA	IN	TTL: 3127, RDLEN: 44
ns.icann.org	noc.dns.icann.org	3224139865	1487798272	471859200	235929618
```

Specifying the resolver to target is possible with `-s` flag.  For example, you may send your queries to one of the public DNS servers like such as cloudflare's 1.1.1.1 like this:

```console
$ lud cloudlflare.com -s 1.1.1.1
ID: 17926
QR: R; Opcode: 0
FLAGS: AA false; TC false; RD: true; RA: true; Z: 0; NoError
QDCOUNT 1; ANCOUNT 1; NSCOUNT 0; ARCOUNT 0

Question
cloudlflare.com	QTYPE: A; CLASS: 1

Answer
cloudlflare.com	A	IN	TTL: 10631, RDLEN: 4
67.227.226.240
```
