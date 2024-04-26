# MomOS
> Group project for CSN-254 course.

In order to meet the needs of developers and enthusiasts looking for a cutting-edge, dependable, memory safe and efficient replacement for conventional OS solutions, our proposed project creates an operating system (OS) in the Rust programming language. <br>
We have named our project MomOS which stands for Memory Optimized Memoization Operating System. We had aimed to create an operating system that prioritizes efficient memory management. This will involve exploring and implementing techniques to optimize memory allocation, minimize memory usage, and prevent memory-related issues. We have achieved all such objectives satisfactorily.<br>



## Developers
This project is developed by:
- Aaditya Aren (22114001)
- Alind Sharma (22113013)
- Aviral Vishwakarma (22114017)
- Divyansh Jain (22114032)
- Kunal Bansal (22115083)
- Parit Gupta (22117100)
- Pratyaksh Bhalla (22115119)


## Building

This project requires a nightly version of Rust because it uses some unstable features. At least nightly _2020-07-15_ is required for building. You might need to run `rustup update nightly --force` to update to the latest nightly even if some components such as `rustfmt` are missing it.

You can build the project by running:

```
cargo build
```

To create a bootable disk image from the compiled kernel, you need to install the [`bootimage`] tool:

[`bootimage`]: https://github.com/rust-osdev/bootimage

```
cargo install bootimage
```

After installing, you can create the bootable disk image by running:

```
cargo bootimage
```

This creates a bootable disk image in the `target/x86_64-blog_os/debug` directory.

Please file an issue if you have any problems.

## Running

You can run the disk image in [QEMU] through:

[QEMU]: https://www.qemu.org/

```
cargo run
```

[QEMU] and the [`bootimage`] tool need to be installed for this.

You can also write the image to an USB stick for booting it on a real machine. On Linux, the command for this is:

```
dd if=target/x86_64-blog_os/debug/bootimage-blog_os.bin of=/dev/sdX && sync
```

Where `sdX` is the device name of your USB stick. **Be careful** to choose the correct device name, because everything on that device is overwritten.

## Testing

To run the unit and integration tests, execute `cargo xtest`.
