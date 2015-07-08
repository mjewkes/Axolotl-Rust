## Raxolotl
Raxolotl is a free cross-platform implementation of the Axolotl cryptographic ratcheting protocol in rust. The Axolotl Protocol addresses the issue of securing modern messaging channels, without sacrificing ease of use. This implementation in the rust programming language aims to provide an easy way forward to integrate Axolotl into existing projects while minimizing headaches and security concerns.

## Status
Raxolotl (as of 0.1.0) is currently looking for reviewers and feedback from the crypto and rust communities. 

## Contributions
Contributions are welcome and encouraged. As such we are committed to maintaining a positive and open environment. Please feel free to create tickets, or submit Pull Requests where applicable. We'll make every effort to address them as fast as possible, while allowing for contributor feedback. 

## License
This software is licensed under the GPLv2

## Documentation + Background

Work based off of the [Axolotl Protocol](https://github.com/trevp/axolotl/wiki) (Marlinspike, Perrin), a public domain specification.

Our hope is to provide a solid, generic implementation of Axolotl that takes advantage of Rust’s safety guarantees and portability, for use in all manner of personal communication projects.

## Thanks

Many thanks to:
- Trevor Perrin for support in interpreting the Axolotl protocol, and explaining the finer points;
- The Rust Core Team and community, including the authors of Rust-Crytpo..
- Frederic Jacobs for sparking the idea of a modular, generic implementation of Axolotl in Rust.