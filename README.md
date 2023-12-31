# ChadGDP

This is not a lanuguage model! This is a joke/meme webapp that can host Open-Source LLMs with Rust and Leptos.

# How to use

1. Install Rust: https://www.rust-lang.org/learn/get-started
2. Install nvidia CUDA toolkit: https://developer.nvidia.com/cuda-downloads. This is to enable GPU-acceleration for Nvidia GPUs. Alternatively, remove ` features=["cublas"]` from Cargo.toml to only use CPU to process prompts.  
3. Setup WASM build with `rustup target add wasm32-unknown-unknown`.
4. Install global dependencies: `cargo install trunk cargo-leptos`
5. Add the path of the LLM model to the .env file as `MODEL_PATH=<PATH HERE>`. Wizard Vicuna (LLAMA) is currently used: https://huggingface.co/TheBloke/Wizard-Vicuna-7B-Uncensored-GGML. LLAMA2 also works: https://huggingface.co/TheBloke/Llama-2-13B-chat-GGML/blob/main/llama-2-13b-chat.ggmlv3.q4_K_S.bin
6. Run `cargo build`. Then run `cargo leptos watch`.
7. Go to https://localhost:3000. ChadGDP should be up if setup went correctly.


![image](https://github.com/marcusbirkeland/chadGDP/assets/36818485/af46278e-343b-4890-bd75-154c5f19f1fd)


## Credits
Check out LLM for more information about how to use llm in rust: https://github.com/rustformers/llm
Source code based on amazing tutorial/repo RustyLlama: https://github.com/MoonKraken/rusty_llama
