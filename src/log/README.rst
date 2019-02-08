Logging
=======

Logging helps you understand what is going on in a running system. Logging must
be structured, otherwise it is impossible to analyze large volumes of log
messages. This means that you must not construct prose using string
concatenation. Instead, pick a short identifier for your type_ of log message,
and use that as a key. Log any additional information with serde_json.

.. _type: https://en.wikipedia.org/wiki/Type%E2%80%93token_distinction
