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

License
-------

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, but only version 3 of the License.

This program is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
