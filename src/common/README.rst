Common
======

In this directory you find types and functions that are component-agnostic. Be
very careful when adding something here: it should really be that general. Do
not attempt to make large types that "model the domain" because they will not,
and it will result in functions taking more information than they use,
inefficient database queries, difficult-to-write tests, and so on and so forth.
