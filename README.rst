Adrenaline
==========

Welcome to the repository root of Adrenaline. Within each directory you can
find the documentation for the concerns covered by that directory, if any.

Adrenaline is a monitoring system. It can monitor any object you want! You can
build and install Adrenaline yourself, or you can use the SaaS solution once
it is ready (stay tuned).

Build instructions
------------------

There is only one build-time dependency: Nix. To build all components of
Adrenaline, ensure Nix is installed, then run the following shell command
from the repository root::

    nix-shell --run 'cargo build'
