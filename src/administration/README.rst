Administration
==============

The administration service provides a CBOR RPC API over ØMQ REQ/REP sockets for
configuring organizations, groups, monitors, events, and alerts.

Rate limiting
-------------

Requests
--------

The encoding of requests is unstable. Each type of request is documented in
this section, with the corresponding type of response. In addition, every
request may be responded to with an Unauthorized response. There is an enforced
limit to the size of each request.

TODO: Do we want to enforce a limit on the size of each response?

Hello
'''''

Send a Hello response back. This is useful for testing if the connection is
working.
