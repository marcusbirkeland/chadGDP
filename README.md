# How to use

1. Install Rust: https://www.rust-lang.org/learn/get-started
2. Install nvidia CUDA toolkit: https://developer.nvidia.com/cuda-downloads. This is to enable GPU-acceleration for Nvidia GPUs.
3. Setup WASM build with `rustup target add wasm32-unknown-unknown`.
4. Add the path of the LLM model to the .env file as `MODEL_PATH=<PATH HERE>`. Wizard Vicuna (LLAMA) is currently used: https://huggingface.co/TheBloke/Wizard-Vicuna-7B-Uncensored-GGML.
5. Open this repo, and run `cargo build`. Then run `cargo leptos watch`.
6. Go to https://localhost:3000