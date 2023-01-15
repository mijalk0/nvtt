# nvtt

A rust wrapper around the [Nvidia Texture Tools 3 library](https://developer.nvidia.com/gpu-accelerated-texture-compression).

NVTT 3 is a library that can be used to compress image data and files into compressed texture formats, and to handle compressed and uncompressed images.

In NVTT 3, most compression algorithms and image processing algorithms can be accelerated by the GPU. These have CPU fallbacks for GPUs without support for CUDA. CUDA operations can be enabled through the `cuda` feature.

# Dependencies

The NVTT 3 SDK must be installed on the system. A non-standard path to the binaries can be specified via the `NVTT_PATH` environment variable. A compiler supporting at least C99 and dynamic linking is also required.

## Windows

Windows 10 or 11 (64-bit) are required.

## Linux

64-bit only; Ubuntu 16.04+ or a similarly compatible distro is required. `libc.so` version 6 or higher is required as well.

# Using nvtt

``` rust
// Create a surface
let input = InputFormat::Bgra8Ub {
    data: &[0u8; 16 * 16 * 4],
    unsigned_to_signed:  false,
};
let image = Surface::image(input, 16, 16, 1).unwrap();

// Create the compression context; enable CUDA if possible
let mut context = Context::new();
if *CUDA_SUPPORTED {
    context.set_cuda_acceleration(true);
}

// Specify compression settings to use; compress to Bc7
let mut compression_options = CompressionOptions::new();
compression_options.set_format(Format::Bc7);

// Specify how to write the compressed data. Here, we write to temporary file.
let mut output_options = OutputOptions::new_temp().unwrap();

// Write the DDS header.
assert!(context.output_header(
    &image,
    1, // number of mipmaps
    &compression_options,
    &mut output_options,
));

// Compress and write the compressed data.
assert!(context.compress(
    &image,
    &compression_options,
    &mut output_options,
));

// Get raw bytes, and delete temporary file
let _bytes = output_options.to_bytes().unwrap();
```

# License

Licensed under the MIT license. Note that the Nvidia Texture Tools SDK has its own seperate license.
