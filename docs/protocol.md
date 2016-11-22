# Rackety Protocol

This document provides instructions on how to communicate with a Rackety server.

### Overview

At the core of Rackety's protocol lays a simple text-based protocol where each
line represents a command or a response from the server and are delimited by
a carriage return and a line feed: `\r\n`. So make sure you escape that sequence
if present in your key or value.

### Commands

```text
set <key> <value>\r\n
get <key>\r\n
del <key>\r\n
```

### Reponses

```text
error <code> <reason>\r\n
ok <result>\r\n
```

### Examples

```text
client> set "this is a key" "and this is a value"\r\n
server> ok\r\n
client> set this_dont_need_quotes true\r\n
server> ok\r\n
client> get "this is a key"\r\n
server> ok "and this is a value"\r\n
```
