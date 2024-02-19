# Rusty AI Chat

## Overview

**Rusty AI Chat** is a Rust-based project that offers a chat experience by utilizing a local Open Source LLM.

This project allows users to input prompts and engage with a powerful language model, similar to ChatGPT, all within the privacy and control of our local environment.

## How it works?

The project utilizes the weights of a neural network of the local Open Source LLM, using [Rustformers](https://github.com/rustformers/llm) to load and use these weights for prompt responses.

The project is built with [Leptos](https://leptos.dev/), a Fullstack Rust framework, ensuring a comprehensive and efficient development experience.

## Requirements

You need to have the following installed in your system:
- Rust (nightly)
- Node.js
- a model

You can install the following packages using the following command:

```bash
make install
```
> [!IMPORTANT]
> Only works on Linux and MacOS.

This will add the following packages:
- `rustup toolchain install nightly` (to use the nightly version of Rust).
- `rustup target add wasm32-unknown-unknown` (to build the frontend components).
- `cargo install trunk cargo-leptos` ([trunk](https://trunkrs.dev/) to work in _localhost_ (as automatic refresh) and [leptos](https://leptos.dev/) is a fullstack framework).
- `npm install` (to install the frontend dependencies).

## Getting Started

You can use it by running the following command:

```bash
make run
```
> [!IMPORTANT]
> Only works on Linux and MacOS.

This will start the chat on your web browser and you can start typing your prompts.
