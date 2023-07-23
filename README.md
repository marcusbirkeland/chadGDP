# How to use

1. Install Rust: https://www.rust-lang.org/learn/get-started
2. Install nvidia CUDA toolkit: https://developer.nvidia.com/cuda-downloads. This is to enable GPU-acceleration for Nvidia GPUs. Alternatively, remove ` features=["cublas"]` from Cargo.toml to only use CPU to process prompts.  
3. Setup WASM build with `rustup target add wasm32-unknown-unknown`.
4. Add the path of the LLM model to the .env file as `MODEL_PATH=<PATH HERE>`. Wizard Vicuna (LLAMA) is currently used: https://huggingface.co/TheBloke/Wizard-Vicuna-7B-Uncensored-GGML.
5. Run `cargo build`. Then run `cargo leptos watch`.
6. Go to https://localhost:3000. ChadGDP should be up if setup went correctly.


![image](https://github.com/marcusbirkeland/chadGDP/assets/36818485/af46278e-343b-4890-bd75-154c5f19f1fd)


## Credits
Check out LLM for more information about how to use llm in rust: https://github.com/rustformers/llm
Source code based on amazing tutorial/repo RustyLlama: https://github.com/MoonKraken/rusty_llama
