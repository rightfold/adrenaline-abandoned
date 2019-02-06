Administration
==============

The administration service provides a CBOR RPC API over ØMQ REQ/REP sockets for
configuring organizations, groups, monitors, events, and alerts.

Authentication
--------------

Every request to the administration service MUST include a valid token. A token
consists of a user identifier with a signature.

Rate limiting
-------------

Requests
--------

The encoding of requests is unstable. Each type of request is documented in
this section.

Hello
'''''

Send a Hello response back. This is useful for testing if the connection and
authentication are working.
