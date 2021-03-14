Dependencies: make, openssl, cargo, rustc <br>
Optional Dependencies: opencl <br>

Build and run with OpenCL:
`USE_OPENCL=y cargo run --release -- ADDRESS:PORT`

Build and run without OpenCL:
`cargo build --release -- ADDRESS:PORT`
