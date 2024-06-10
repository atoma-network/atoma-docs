# Atoma's Contracts

Currently the Atoma Network protocol is supported on the Sui blockchain. We will expand Atoma's reach to both EVM chains and Solana.

## Sui

### Atoma Contract Features

We have a current implementation of the Atoma contract on Sui. This contract is responsible for the following features:

1. `Node registration` - Nodes operating on the Atoma Network should first register on the Atoma contract. Once registered on the Atoma contract, nodes can receive newly submitted requests and run the required computation to resolve the request. 
2. `Submit collateral` - Upon registration, nodes should deposit a given amount of collateral. The collateral is indexed in Atoma's native token, the `TOMA` token. 
3. `Accrue fees` - Nodes accrue fees, indexed in `TOMA` token, based on the number and the type of requests they process. Accrued fees can only be withdrawn two epochs later.
4. `Subscribe to AI models and other forms of compute types` - Upon registration, nodes should specify which AI models these nodes subscribe to (i.e., which models do the nodes currently host). Once a node registers for a given set of models, it can't change these, unless it deregisters and registers a second type, specifying the new set of models.
5. `Node deregistration` - Once a node decides to stop providing compute to the Atoma Network, it can deregister itself directly on the smart contract.
6. `Specifying node hardware features` - The Atoma Network protocol runs on a `sampling consensus` mechanism. This mechanism requires multiple different nodes to reach consensus on the execution of a given output state. To achieve this, it is required that nodes, on a selected `quorum`, generate outputs in a deterministic fashion. However, most AI requests are `non-deterministic in nature`. It is possible to achieve determinism, if the selected nodes for a given request, have the same GPU hardware with the same congifuration. For this reason, nodes must submit the type of GPU card(s) these can hold.
7. `Echelon specification` - Upon 5., the Atoma contract specifies compute `echelons`. These can be thought of as shards of the network. Compute across echelons should be as homogeneous as possible in compute and memory requirements, process time, and determinism. 
8. `Request submission` - Every request to the Atoma Network (processed by Atoma tokens) is submitted via the Atoma contract. Requests are paid in `TOMA` token.
9. `Load balancing` - Based on fine grained echelon performance, the Atoma contract is responsible for balancing request total volume across suitable echelons (based on their total available compute) and each echelon total `amount of work` at each time.
10. `Random sampling` - Each request submitted into the Atoma Network is processed across a finite number of nodes within the same suitable echelon. The Atoma contract is responsible for randomly selecting the requested number of nodes. We currently use Sui's on-chain random generation features.
11. `Timeouts` - The Atoma contract keeps a registry of the time it takes to process each request. If a node does not submit a request on time, a time out is triggered and a percentage of the node's deposited collateral is slashed automatically.
12. `Output commitment submission` - Upon generating a new output for a given request, a node must submit a cryptographic commitment back to the Atoma contract. This commitment is used by the Atoma contract to check if there is `consensus on the state of the output`. Once consensus is reached, all nodes that generated a commitment are entitled to accrue fees (paid by the user on request submission).
13. `Dispute` - If consensus is not reached on the state of the output (that is, different nodes submit different commitments), a dispute mechanism is put forth by the Atoma contract, by selecting additional high reputation (running trusted hardware) to resolve the dispute.
14. `Staking` - Registered nodes are entitled for staking rewards based on their average node performance, in each echelon (future feature).
15. `Governance` - Will allow `TOMA` holders to vote and decide which models to operate on the Atoma Network, as well as other types of compute (future feature).

### Future Features
We plan to add other features to the Atoma contract. These include

1. `Staking`;
2. `Governance`;
3. `Dispute` - we are in the process of establishing different types of dispute resolving (i.e BFT dispute resolution and trusted hardware oracle nodes)
4. `General compute tasks`, this will include general WASM applications that can be run on Atoma nodes. Due to potential security issues for both the user and the node, we will require such applications to run in trusted execution environments (TEEs).

### Atoma Contract Documentation

The following instructions provide a detailed description on how to interact with the Atoma contract, on the Sui blockchain.

#### Atoma on Sui

Useful links:

- [Install Sui][sui-install]
- [Sui Move Analyzer][sui-analyzer]
- [Sui standard lib on Github][github-sui-std]

The packages and CLI is pointed to the currently released Sui _mainnet_ version tag.

Upgrade your CLI to the appropriate mainnet version that matches the [`Move.toml`](packages/atoma/Move.toml):

```sh
cargo install --locked --git https://github.com/MystenLabs/sui.git --tag mainnet-vX.Y.Z sui
```

When upgrading, the version needs to be changed in

- [`atoma` package](packages/atoma/Move.toml)
- [`toma` package](packages/toma/Move.toml)
- [`cli` binary](cli/Cargo.toml)

#### Events

The Atoma contract emits various types of events:

- `db::NodeRegisteredEvent` is emitted when a new node puts up collateral to register.
- `db::NodeSubscribedToModelEvent` is emitted when a node subscribes to a model echelon and is ready to receive prompts.
- `gate::Text2TextPromptEvent` is emitted when a user submits a text to text prompt.
- `gate::Text2ImagePromptEvent` is emitted when a user submits a text to image prompt.
- `settlement::FirstSubmissionEvent` is emitted when a node submits the _first_ response to a prompt.
- `settlement::DisputeEvent` is emitted when a node disputes a submission.
  Now, we want for an oracle to resolve the dispute.
- `settlement::SettledEvent` is emitted when a ticket is settled and fee is distributed.
- `settlement::NewlySampledNodesEvent` is emitted when a new set of nodes is sampled for a prompt because of timeout.

#### Create a Sui wallet

As a first step, in order to interact with the Atoma contract, a user must have a wallet on the Sui blockchain. If the reader already has one, it can skip to the next section, otherwise we recommend following the official Sui [docs](https://blog.sui.io/sui-wallets/).


#### How to use the atoma protocol

To interact with the Atoma protocol, utilize the `gate` module within the `atoma` package, responsible for prompt submission.

A crucial parameter is the shared object ID for `AtomaDb`.
These, along with the package ID, should be configured once and remain unchanged.
The `AtomaDb` object ID can be derived from the package ID by querying the first transaction of the package and locating the shared object with the type name `AtomaDb` _if necessary_.

Before we list all the parameters, here are some general rules:

- Floats are stored on-chain as `u32` integers.
  To convert from float to `u32`, convert the float to little-endian bytes and then interpret those bytes as a little-endian `u32`: `u32::from_le_bytes(xxx_f32.to_le_bytes())`
  Conversely, to convert from `u32` to float, use the reverse process.

As of now, the supported modalities are:

- `submit_text2text_prompt` with params `Text2TextPromptParams`:

  - `max_tokens`: determines the maximum output to be generated and also the amount of `TOMA` tokens charged.
    Unused tokens are refunded upon response generation.
    We discuss pricing below.
  - `model`: a string identifier of the model for text-to-text generation.
    Refer to our website for supported models.
  - `prompt`: input text prompt.
    There's no limit to the prompt length at the protocol level, but a Sui transaction can be at most 128KB.
  - `random_seed`: any random number to seed the random generator for consistent output across nodes.
    Before Sui stabilizes random generator, you can use `atoma::utils::random_u64`.
  - `repeat_last_n`: instructs the model to avoid reusing tokens within the last `n` tokens.
  - `repeat_penalty`: a float number determining token repetition avoidance.
  - `temperature`: a float number determining randomness in the output.
  - `top_k`: an integer determining token consideration for the next generation.
  - `top_p`: a float number determining token consideration for the next generation.

- `submit_text2image_prompt` with params `Text2ImagePromptParams`:

  - `guidance_scale`: a float number determining the consideration of the guidance image.
  - `height`: height of the image.
    See pricing below.
  - `img2img`: an optional string indicating the image to start generating with stable diffusion.
  - `img2img_strength`: a float number indicating the consideration of the `img2img` image.
  - `model`: same as above.
  - `n_steps`: an integer indicating how many steps the model should take to generate the image.
  - `num_samples`: an integer indicating how many samples the model should generate.
  - `prompt`: same as above.
  - `random_seed`: same as above.
  - `uncond_prompt`: negative word prompt.
  - `width`: width of the image.

A wallet with `TOMA` tokens is required for prompt payment, with charges varying based on prompt type.
Pricing for input and output tokens differs for each model.
Each model has a pricing for input and output tokens as two separate parameters.
For text to text models, these two parameters are likely to be the same.

The parameter `nodes_to_sample` is optional and defaults to a sensible value.
Higher number of nodes means higher confidence in the generated output.
However, the price is also higher as nodes multiply the prompt price.

- `Text2TextPromptParams` charges `nodes_to_sample * (prompt_len * input_token_price + max_tokens * output_token_price)` upon prompt submission.
- `Text2ImagePromptParams` charges `nodes_to_sample * (prompt_len * input_token_price + num_samples * output_token_price)` upon submission.

Unused tokens are reimbursed upon response generation by sending a `Coin<TOMA>` object to the prompt submitter.

`submit_text2text_prompt` function has a `max_fee_per_token` parameter.
This applies to both input and output token prices.
If no nodes can generate the prompt within the budget, the transaction fails.

`submit_text2image_prompt` has a `max_fee_per_input_token` and `max_fee_per_output_token` parameters.
These apply to input and output token prices, respectively.

The last parameter is `nodes_to_sample`.
It's optional and defaults to a sensible value.
Higher number of nodes means higher confidence in the generated output.
However, the price is also higher as nodes multiply the prompt price.

Refer to the `atoma::prompts` module for sample implementations.
If you are developing a custom smart contract for prompt submission, this module is a great starting point.

Since these functions are `public` but not `entry`, they must be used in Sui's programmable transactions from the client's perspective.

#### Dev Environment

There's a [`check` shell script](dev/check) that builds all packages.

As of right now we don't use `localnet` for testing because the Sui CLI support for faucet is broken.

#### CLI

##### Env

The CLI loads values from environment variables.
You can set these in your shell or in a `.env` file in the root of the repository.

If any value is not provided, the CLI does best effort to figure it out from the context.
For example, if you provide package ID but not atoma DB object ID, the CLI will query Sui to find it.

```text
WALLET_PATH=
PACKAGE_ID=
ATOMA_DB_ID=
MANAGER_BADGE_ID=
NODE_BADGE_ID=
NODE_ID=
TOMA_WALLET_ID=
GAS_BUDGET=
```

You can also generate these values by running the following command:

```sh
./cli db print-env --package "YOUR PACKAGE ID"
```

The following commands should get you started once you have the Sui binary installed.

```sh
# check what environment (devnet, testnet, mainnet) you're in
sui client active-env
# check what is your active address
sui client active-address
# get some coins into the active address from the faucet
sui client faucet
```

#### `TOMA` token

The `TOMA` token is used as collateral that nodes must lock up to participate.
It's defined in the [`toma` package](./packages/toma).


#### Node registration

In order to register a node, it is required to deposit a given amount of collateral onto the Atoma contract, indexed in `TOMA` tokens. Therefore, a node must acquire enough `TOMA` tokens before registration. Currently, the required amount of `TOMA` tokens for collateral is `10_000`. 

Once the node operator has enough `TOMA` tokens, it can register itself as a node, through the cli command:

```sh
./cli db register-node \
    --package "TODO(add package id here)"
    --echelon NODE_ECHELON
```

Current node echelons are the following (based on the node's type of GPU):

0 - 1 x NVIDIA RTX3090
1 - 2 x NVIDIA RTX3090
2 - 4 x NVIDIA RTX3090
3 - 8 x NVIDIA RTX3090
4 - 1 x NVIDIA RTX4080
5 - 2 x NVIDIA RTX4080
6 - 4 x NVIDIA RTX4080
7 - 8 x NVIDIA RTX4080
9 - 1 x NVIDIA RTX4090
10 - 2 x NVIDIA RTX4090
11 - 4 x NVIDIA RTX4090
12 - 8 x NVIDIA RTX4090
13 - 1 x NVIDIA L40
14 - 2 x NVIDIA L40
15 - 4 x NVIDIA L40
16 - 8 x NVIDIA L40
17 - 1 x NVIDIA A6000
18 - 2 x NVIDIA A6000
19 - 4 x NVIDIA A6000
20 - 8 x NVIDIA A6000
21 - 1 x NVIDIA A100 (40GB)
22 - 2 x NVIDIA A100 (40GB)
23 - 4 x NVIDIA A100 (40GB)
24 - 8 x NVIDIA A100 (40GB)
25 - 1 x NVIDIA A100 (80GB)
26 - 2 x NVIDIA A100 (80GB)
27 - 4 x NVIDIA A100 (80GB)
28 - 8 x NVIDIA A100 (80GB)
29 - 1 x NVIDIA H100 (40GB)
30 - 2 x NVIDIA H100 (40GB)
31 - 4 x NVIDIA H100 (40GB)
32 - 8 x NVIDIA H100 (40GB)
33 - 1 x NVIDIA H100 (80GB)
34 - 2 x NVIDIA H100 (80GB)
35 - 4 x NVIDIA H100 (80GB)
36 - 8 x NVIDIA H100 (80GB)
100 - MACBOOK PRO M2 (Metal)
101 - MACBOOK PRO M3 (Metal)
200 - AMD

#### Node model subscription

In order to subscribe to a given model, the node operator can run the following command

```sh
./cli db add-node-to-model \
    --package "TODO(add package id here)" \
    --model "MODEL" \
```

Notice that once a node subscribes to a given model, it is entitled to execute requests for that specific model.
It the node doesn't host the model, node submission will timeout. This means that part of the node's submitted collateral 
will be slashed for timeout. In order to avoid this, the node operator must be sure to subscribe to only host to models
it currently hosts.

The available list of supported models is:

| Model Type                         | Hugging Face model name                  |
|------------------------------------|------------------------------------------|
| falcon_7b                          | tiiuae/falcon-7b                         |
| falcon_40b                         | tiiuae/falcon-40b                        |
| falcon_180b                        | tiiuae/falcon-180b                       |
| llama_v1                           | Narsil/amall-7b                          |
| llama_v2                           | meta-llama/Llama-2-7b-hf                 |
| llama_solar_10_7b                  | upstage/SOLAR-10.7B-v1.0                 |
| llama_tiny_llama_1_1b_chat         | TinyLlama/TinyLlama-1.1B-Chat-v1.0       |
| llama3_8b                          | meta-llama/Meta-Llama-3-8B               |
| llama3_instruct_8b                 | meta-llama/Meta-Llama-3-8B-Instruct      |
| llama3_70b                         | meta-llama/Meta-Llama-3-70B              |
| mamba_130m                         | state-spaces/mamba-130m                  |
| mamba_370m                         | state-spaces/mamba-370m                  |
| mamba_790m                         | state-spaces/mamba-790m                  |
| mamba_1-4b                         | state-spaces/mamba-1.4b                  |
| mamba_2-8b                         | state-spaces/mamba-2.8b                  |
| mistral_7bv01                      | mistralai/Mistral-7B-v0.1                |
| mistral_7bv02                      | mistralai/Mistral-7B-v0.2                |
| mistral_7b-instruct-v01            | mistralai/Mistral-7B-Instruct-v0.1       |
| mistral_7b-instruct-v02            | mistralai/Mistral-7B-Instruct-v0.2       |
| mixtral_8x7b                       | mistralai/Mixtral-8x7B-v0.1              |
| phi_3-mini                         | microsoft/Phi-3-mini-4k-instruct         |
| stable_diffusion_v1-5              | runwayml/stable-diffusion-v1-5           |
| stable_diffusion_v2-1              | stabilityai/stable-diffusion-2-1         |
| stable_diffusion_xl                | stabilityai/stable-diffusion-xl-base-1.0 |
| stable_diffusion_turbo             | stabilityai/sdxl-turbo                   |
| quantized_7b                       | TheBloke/Llama-2-7B-GGML                 |
| quantized_13b                      | TheBloke/Llama-2-13B-GGML                |
| quantized_70b                      | TheBloke/Llama-2-70B-GGML                |
| quantized_7b-chat                  | TheBloke/Llama-2-7B-Chat-GGML            |
| quantized_13b-chat                 | TheBloke/Llama-2-13B-Chat-GGML           |
| quantized_70b-chat                 | TheBloke/Llama-2-70B-Chat-GGML           |
| quantized_7b-code                  | TheBloke/CodeLlama-7B-GGUF               |
| quantized_13b-code                 | TheBloke/CodeLlama-13B-GGUF              |
| quantized_32b-code                 | TheBloke/CodeLlama-34B-GGUF              |
| quantized_7b-leo                   | TheBloke/leo-hessianai-7B-GGUF           |
| quantized_13b-leo                  | TheBloke/leo-hessianai-13B-GGUF          |
| quantized_7b-mistral               | TheBloke/Mistral-7B-v0.1-GGUF            |
| quantized_7b-mistral-instruct      | TheBloke/Mixtral-8x7B-Instruct-v0.1-GGUF |
| quantized_7b-mistral-instruct-v0.2 | TheBloke/Mistral-7B-Instruct-v0.2-GGUF   |
| quantized_7b-zephyr-a              | TheBloke/zephyr-7B-alpha-GGUF            |
| quantized_7b-zephyr-b              | TheBloke/zephyr-7B-beta-GGUF             |
| quantized_7b-open-chat-3.5         | TheBloke/openchat_3.5-GGUF               |
| quantized_7b-starling-a            | TheBloke/Starling-LM-7B-alpha-GGUF       |
| quantized_mixtral                  | TheBloke/Mixtral-8x7B-v0.1-GGUF          |
| quantized_mixtral-instruct         | TheBloke/Mistral-7B-Instruct-v0.1-GGUF   |
| quantized_llama3-8b                | QuantFactory/Meta-Llama-3-8B-GGUF        |
| qwen_w0.5b                         | Qwen/Qwen1.5-0.5B                        |
| qwen_w1.8b                         | Qwen/Qwen1.5-1.8B                        |
| qwen_w4b                           | Qwen/Qwen1.5-4B                          |
| qwen_w7b                           | qwen/Qwen1.5-7B                          |
| qwen_w14b                          | qwen/Qwen1.5-14B                         |
| qwen_w72b                          | qwen/Qwen1.5-72B                         |
| qwen_moe_a2.7b                     | qwen/Qwen1.5-MoE-A2.7B                   |


#### Atoma's request submission

To submit a request to the Atoma network, a user can run the following command:

TODO: replace with `text` or `image` requests.

```sh
./cli gate submit-tell-me-a-joke-prompt \
    --package "your package id can be found when publishing" \
    --model "llama"
```


<!-- List of References -->

[github-sui-std]: https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/packages/sui-framework/sources
[sui-install]: https://docs.sui.io/guides/developer/getting-started/sui-install
[sui-analyzer]: https://marketplace.visualstudio.com/items?itemName=MoveBit.sui-move-analyzer
[sui-explorer]: https://explorer.sui.io
