#Raxolotl Contributor Primer
The project is structured around two major components the [ Axolotl Trait](https://github.com/mjewkes/Axolotl-Rust/blob/develop/src/axolotl/axolotl.rs) and [AxolotlState](https://github.com/mjewkes/Axolotl-Rust/blob/develop/src/axolotl/state.rs) implementation.

## Axolotl Trait
The Axolotl trait exposes the various objects and required tasks to implementors. An implementation of the Axolotl Protocol is required to define handler functions for the various tasks, such as Encode, Authenticate, encrypt, etc. Heavy use of associated types allow for control over what types are passed in and returned from these functions. These two properties combined enables for custom control over which crypto-primitives are used along with enabling integration into existing message formats.

## AxolotlState
AxolotlState represents an Axolotl session, and has methods that implement the Axolotl protocol. AxolotlState is defined generically over all implementations of the Axolotl trait. AxolotlState handles ratcheting and message encryption/encoding/authentication/etc by invoking the appropriate user implemented trait functions. As this is defined for all trait implementations this removes the burden of reimplementing the Axolotl protocol for each variation of crypto primitives, message formats, etc.

## Flexibility vs Simplicity
This library is intended to be place very little restriction on the choice of crypto primitives or implementations. To reduce barriers to entry the intention is to release a more restricted abstraction layer soon, which enforces some established best practices simplifying the process. The combination of the generic and simplified interfaces should cover most use-cases.
