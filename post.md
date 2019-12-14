# From zero to main(): Bare metal Rust

<!-- TODO: A brief mention of Rust -->

## Setting the stage

<!-- TODO: Explain the nRF52, the tools, etc. -->

## Writing a Reset_Handler

From the post:

* Entry point symbol
* Zero BSS
* Copy rodata to data
* Branch to main

Additionally in Rust:

* Defining a panic handler
* No Main
* Divergent functions


# Unsorted notes

* Original post
    * https://interrupt.memfault.com/blog/zero-to-main-1
* Linker script post
    * https://interrupt.memfault.com/blog/how-to-write-linker-scripts-for-firmware
