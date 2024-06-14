# Tiny Places

Tiny Places is meant to become a collaborative map editor and rpg system. The client is built with
the Love2d Lua game framework, the server is Java code.

Since May 2024 there is an offline/single user version in development, which is programmed in Rust. It uses the "Piston" game engine. 

## Start the client

    cd tiny_places
    love tiny_places_client

### Get Love2D for Linux

The easiest way seems to be to use the official Love2d PPA.

    sudo add-apt-repository ppa:bartbes/love-stable
    sudo apt-get update
    sudo apt-get install love

## Start the server

Instructions to build the server should be here ...

    java -jar tiny_places_server/dist/TinyPlacesServer.jar

## Fractal Lands (formerly Tiny Places Standalone)

You need to have Rust installed on your computer to build fractal lands from source. Please see https://www.rust-lang.org/tools/install for further instructions. Once Rust is installed, you can compile and start fractal lands this way:

    cd fractal_lands
    cargo run

Cargo is the build tool for Rust. It will download all dependencies needed to compile fractal lands and then run the created binary. To just build without running the binary, use "cargo build". "Cargo clean" will delete all artifacts created during the build process.
