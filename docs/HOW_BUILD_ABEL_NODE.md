# How to build Abel-BKChain Node

## Step 1.  Prerequisites
You will probably need to do some set-up to prepare your computer for Abel-BKChain development.

If you are using a Unix-based machine (Linux, MacOS), we have created a simple one-liner to help you set up your computer:



```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```
this script will automatically install:
- CMake
- pkg-config
- OpenSSL
- Git
- Rust


## Step 2. Compiling Abel-BKChain
Once the prerequisites are installed, you can use Git to clone the Abel-BKChain Source Code.

1.  Clone the Abel-BKChain (version testnet-v1.0.0).
    ```bash
    git clone -b testnet-v1.0.0 --depth 1 https://github.com/abelbeertech/abel-bkchain.git
    ```
2. Initialize your WebAssembly build environment
    ```bash
    # Load settings into the current shell script if you can't use rustup command
    # If you've run this before, you don't need to run it again. But doing so is harmless.
    source ~/.cargo/env

    #switch rust version
    rustup install nightly-2020-10-06
    rustup update stable

    # Add Wasm target
    rustup target add wasm32-unknown-unknown --toolchain nightly-2020-10-06
    ```    
3. Compile the Abel-BKChain
    ```bash
    cd abel-bkchain/
    cargo +nightly-2020-10-06 build --release
    ```
    The time required for the compilation step depends on the hardware you're using.
 
    eg. 2vCPU-4G server node takes about an hour.
