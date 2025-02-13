# Getting started with Ariel OS

This guide is intended to get you started in about 5 minutes.
It explains how to compile and run the `hello-word` example to verify your setup, and how to bootstrap a new application.

> Currently only GNU/Linux is supported in this guide.

## Installing the build prerequisites

1. Install the needed build dependencies.
   On Ubuntu, the following is sufficient:

    ```sh
    apt install git rustup ninja-build pkg-config libudev-dev clang gcc-arm-none-eabi
    ```

1. Install the Rust installer [rustup](https://rustup.rs/) using the website's instructions or through your distribution package manager.

1. Install the build system [laze](https://github.com/kaspar030/laze):

    ```sh
    cargo install laze
    ```

1. Install the debugging and flashing utility [probe-rs](https://github.com/probe-rs/probe-rs):

    ```sh
    cargo install --locked probe-rs-tools
    ```

1. Clone the [Ariel OS repository][ariel-os-repo] and `cd` into it.

1. Install the Rust targets:

    ```sh
    laze build install-toolchain
    ```

## Running the `hello-world` example

To check that everything is installed correctly, the `hello-word` example can be compiled and run from the `ariel-os` directory.
The following assumes you have your target board connected to your host computer.

Find the Ariel OS name of your supported board in the [support matrix](./hardware_functionality_support.html).

> The following assumes the Nordic nRF52840-DK, whose Ariel OS name is `nrf52840dk`.
> Replace that name with your board's.

Then, **from the `ariel-os` directory**, compile and run the example, as follows:

```sh
laze -C examples/hello-world build -b nrf52840dk run
```

<details>
    <summary>(This might fail if the flash is locked, click here for unlocking instructions.)</summary>
This might fail due to a locked chip, e.g., on most nRF52840-DK boards that are fresh from the factory.
In that case, the above command throws an error that ends with something like this:

```sh
An operation could not be performed because it lacked the permission to do so: erase_all
```

The chip can be unlocked using this command:

```sh
laze -C examples/hello-world build -b nrf52840dk flash-erase-all
```
</details>

![Terminal screencast of compiling and flashing the hello-world example](./hello-world_render.svg)

> If you do not plan on working on Ariel OS *itself*, this repository is not needed anymore and can be deleted.

## Starting an application project from a template repository

Applications are expected to be developed out-of-tree, outside of the `ariel-os` directory.
This is made possible by [laze's `imports`][laze-imports-book] feature.

To start a new application project, you can either clone the [`ariel-os-hello` repository][ariel-os-hello-repo] or, *alternatively*, use one of the [`cargo-generate`][cargo-generate-repo] templates.

### Cloning `ariel-os-hello`

```sh
git clone https://github.com/ariel-os/ariel-os-hello
```

### Using a `cargo-generate` project template

This requires installing [`cargo-generate`][cargo-generate-repo], then a new application project can be created as follows:

```sh
cargo generate --git https://github.com/ariel-os/ariel-os-template --name <new-project-name>
```

### Running the template example

To check your setup, the default application can be compiled and run as follows:

```sh
laze build -b nrf52840dk run
```

> The board name needs to be replaced with your board's.

See the [Build System page](./build_system.md) to learn more about laze and how to work with out-of-tree applications.

[ariel-os-repo]: https://github.com/ariel-os/ariel-os
[ariel-os-hello-repo]: https://github.com/ariel-os/ariel-os-hello
[laze-imports-book]: https://kaspar030.github.io/laze/dev/reference/imports.html
[cargo-generate-repo]: https://github.com/cargo-generate/cargo-generate
