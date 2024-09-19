# Atoma's Contracts

The Atoma Network is supported by an on-chain smart contract on the Sui blockchain. That said, the Atoma protocol is chain agnostic, in particular, we have future plans to expand Atoma's functionality to other chains, such as EVM compatible chains, Solana, Near, etc. We will also explore the possibility of integrating as an EigenLayer AVS, or building our own L1/L2 for native payments.

This document outlines the key features, upcoming developments, and usage instructions for interacting with Atoma's smart contracts on Sui.
## Atoma Contract Features

The Atoma contract on Sui implements the following key features:

1. **Node Registration**: Nodes must register to participate in the Atoma Network and process requests.

2. **Collateral Management**: Nodes deposit `TOMA` tokens as collateral upon registration.

3. **Fee Accrual**: Nodes earn fees in `TOMA` tokens based on processed requests, withdrawable after two epochs.

4. **Model Subscription**: Nodes specify which AI models they host and can process.

5. **Node Deregistration**: Allows nodes to exit the network and withdraw collateral.

6. **Hardware Specification**: Nodes declare their GPU configurations to ensure deterministic outputs within quorums.

7. **Echelon System**: Organizes nodes into compute shards (echelons) based on hardware capabilities.

8. **Request Handling**: Manages submission and payment (in `TOMA`) for network requests.

9. **Load Balancing**: Distributes requests across suitable echelons based on performance and workload.

10. **Random Node Sampling**: Selects a subset of nodes within an echelon to process each request.

11. **Timeout Enforcement**: Monitors request processing times and slashes collateral for late responses.

12. **Output Consensus**: Nodes submit cryptographic commitments of outputs to reach consensus.

13. **Dispute Resolution**: Handles disagreements on output state using high-reputation nodes.

### Upcoming Features

1. **Staking**: Reward system for nodes based on performance within echelons.
2. **Governance**: Voting mechanism for `TOMA` holders to influence network decisions.
3. **Enhanced Dispute Resolution**: Implementing BFT and trusted hardware oracle solutions.
4. **General Compute Tasks**: Support for WASM applications running inside Trusted Execution Environments (TEEs).

This contract design ensures a robust, scalable, and secure decentralized compute network for AI and other intensive tasks.
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

#### Create a Sui Wallet

To interact with the Atoma contract on the Sui blockchain, you'll need a Sui wallet. If you already have one, you can skip to the next section. Otherwise, follow these steps to create a new wallet:

1. Choose a Sui wallet:
   - For browser extensions: [Sui Wallet](https://chrome.google.com/webstore/detail/sui-wallet/opcgpfmipidbgpenhmajoajpbobppdil) or [Ethos Wallet](https://chrome.google.com/webstore/detail/ethos-sui-wallet/mcbigmjiafegjnnogedioegffbooigli)
   - For mobile: [Suiet](https://suiet.app/) or [Morphis Wallet](https://morphiswallet.com/)

2. Install your chosen wallet and follow the setup instructions.

3. Securely store your recovery phrase (seed words) in a safe place.

4. Fund your wallet:
   - For testnet: Use the [Sui Faucet](https://discord.com/channels/916379725201563759/971488439931392130) in the official Sui Discord.
   - For mainnet: Purchase SUI tokens from a supported exchange.

5. Verify your wallet balance using the Sui Explorer or your wallet interface.

For more detailed instructions and additional wallet options, refer to the [official Sui documentation on wallets](https://docs.sui.io/learn/about-sui/sui-wallets).



#### How to use the Atoma protocol

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
  - `pre_prompt_tokens`: For in-context applications, this is the number of tokens already generated before the user's current prompt.
  - `prompt`: input text prompt.
    There's no limit to the prompt length at the protocol level, but a Sui transaction can be at most 128KB.
  - `random_seed`: any random number to seed the random generator for consistent output across nodes.
    Before Sui stabilizes random generator, you can use `atoma::utils::random_u64`.
  - `repeat_last_n`: instructs the model to avoid reusing tokens within the last `n` tokens.
  - `repeat_penalty`: a float number determining token repetition avoidance.
  - `should_stream_output`: a boolean indicating whether the output should be streamed or not
  to a suitable output destination.
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

The last parameter is `nodes_to_sample`, as an optional parameter. If specified, a
higher number of nodes means higher confidence in the generated output, overall.
However, the price is also higher as nodes multiply the prompt price. This behavior
is part of our standard `Sampling Consensus` protocol.
If the value of `nodes_to_sample` is not specified, then the protocol will advance
with the Cross-Validation Sampling Consensus mechanism. That is, a single node will
be sampled by the contract and once the node generates the response, the contract
will sample more nodes to attest to the response's correctness, with some probability `p`,
specified at the protocol level. This approach reduces the cost of verifiable inference,
while guaranteeing that the protocol converges to game-theoretical Nash equilibrium, where
honest nodes are incentivized to act honestly.

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

| ID | GPU Configuration |
|----|-------------------|
| 0 | 1 x NVIDIA RTX3090 |
| 1 | 2 x NVIDIA RTX3090 |
| 2 | 4 x NVIDIA RTX3090 |
| 3 | 8 x NVIDIA RTX3090 |
| 4 | 1 x NVIDIA RTX4080 |
| 5 | 2 x NVIDIA RTX4080 |
| 6 | 4 x NVIDIA RTX4080 |
| 7 | 8 x NVIDIA RTX4080 |
| 9 | 1 x NVIDIA RTX4090 |
| 10 | 2 x NVIDIA RTX4090 |
| 11 | 4 x NVIDIA RTX4090 |
| 12 | 8 x NVIDIA RTX4090 |
| 13 | 1 x NVIDIA L40 |
| 14 | 2 x NVIDIA L40 |
| 15 | 4 x NVIDIA L40 |
| 16 | 8 x NVIDIA L40 |
| 17 | 1 x NVIDIA A6000 |
| 18 | 2 x NVIDIA A6000 |
| 19 | 4 x NVIDIA A6000 |
| 20 | 8 x NVIDIA A6000 |
| 21 | 1 x NVIDIA A100 (40GB) |
| 22 | 2 x NVIDIA A100 (40GB) |
| 23 | 4 x NVIDIA A100 (40GB) |
| 24 | 8 x NVIDIA A100 (40GB) |
| 25 | 1 x NVIDIA A100 (80GB) |
| 26 | 2 x NVIDIA A100 (80GB) |
| 27 | 4 x NVIDIA A100 (80GB) |
| 28 | 8 x NVIDIA A100 (80GB) |
| 29 | 1 x NVIDIA H100 (40GB) |
| 30 | 2 x NVIDIA H100 (40GB) |
| 31 | 4 x NVIDIA H100 (40GB) |
| 32 | 8 x NVIDIA H100 (40GB) |
| 33 | 1 x NVIDIA H100 (80GB) |
| 34 | 2 x NVIDIA H100 (80GB) |
| 35 | 4 x NVIDIA H100 (80GB) |
| 36 | 8 x NVIDIA H100 (80GB) |
| 37 | 1 x NVIDIA RTX 2060 |
| 38 | 2 x NVIDIA RTX 2060 |
| 39 | 4 x NVIDIA RTX 2060 |
| 40 | 1 x NVIDIA RTX 2070 |
| 41 | 2 x NVIDIA RTX 2070 |
| 42 | 4 x NVIDIA RTX 2070 |
| 43 | 1 x NVIDIA RTX 2080 |
| 44 | 2 x NVIDIA RTX 2080 |
| 45 | 4 x NVIDIA RTX 2080 |
| 46 | 1 x NVIDIA RTX 2080 Ti |
| 47 | 2 x NVIDIA RTX 2080 Ti |
| 48 | 4 x NVIDIA RTX 2080 Ti |
| 49 | 1 x NVIDIA RTX 3060 |
| 50 | 2 x NVIDIA RTX 3060 |
| 51 | 4 x NVIDIA RTX 3060 |
| 52 | 1 x NVIDIA RTX 3070 |
| 53 | 2 x NVIDIA RTX 3070 |
| 54 | 4 x NVIDIA RTX 3070 |
| 55 | 1 x NVIDIA RTX 3080 |
| 56 | 2 x NVIDIA RTX 3080 |
| 57 | 4 x NVIDIA RTX 3080 |
| 58 | 1 x NVIDIA Titan V (Volta) |
| 59 | 2 x NVIDIA Titan V (Volta) |
| 60 | 4 x NVIDIA Titan V (Volta) |
| 61 | 1 x NVIDIA Quadro RTX 8000 (Turing) |
| 62 | 2 x NVIDIA Quadro RTX 8000 (Turing) |
| 63 | 4 x NVIDIA Quadro RTX 8000 (Turing) |
| 64 | 1 x NVIDIA RTX 4060 |
| 65 | 2 x NVIDIA RTX 4060 |
| 66 | 4 x NVIDIA RTX 4060 |
| 67 | 1 x NVIDIA RTX 4070 |
| 68 | 2 x NVIDIA RTX 4070 |
| 69 | 4 x NVIDIA RTX 4070 |
| 70 | 1 x NVIDIA RTX 4070 Ti |
| 71 | 2 x NVIDIA RTX 4070 Ti |
| 72 | 4 x NVIDIA RTX 4070 Ti |
| 1000 | 1 x AMD Radeon RX 6600 |
| 1001 | 2 x AMD Radeon RX 6600 |
| 1002 | 4 x AMD Radeon RX 6600 |
| 1003 | 1 x AMD Radeon RX 6700 XT |
| 1004 | 2 x AMD Radeon RX 6700 XT |
| 1005 | 4 x AMD Radeon RX 6700 XT |
| 1006 | 1 x AMD Radeon RX 6800 XT |
| 1007 | 2 x AMD Radeon RX 6800 XT |
| 1008 | 4 x AMD Radeon RX 6800 XT |
| 1009 | 1 x AMD Radeon RX 6900 XT |
| 1010 | 2 x AMD Radeon RX 6900 XT |
| 1011 | 4 x AMD Radeon RX 6900 XT |
| 1012 | 1 x AMD Radeon RX 7600 |
| 1013 | 2 x AMD Radeon RX 7600 |
| 1014 | 4 x AMD Radeon RX 7600 |
| 1015 | 1 x AMD Radeon RX 7700 XT |
| 1016 | 2 x AMD Radeon RX 7700 XT |
| 1017 | 4 x AMD Radeon RX 7700 XT |
| 1018 | 1 x AMD Radeon RX 7800 XT |
| 1019 | 2 x AMD Radeon RX 7800 XT |
| 1020 | 4 x AMD Radeon RX 7800 XT |
| 1021 | 1 x AMD Radeon RX 7900 XT |
| 1022 | 2 x AMD Radeon RX 7900 XT |
| 1023 | 4 x AMD Radeon RX 7900 XT |
| 1024 | 1 x AMD Radeon RX 7900 XTX |
| 1025 | 2 x AMD Radeon RX 7900 XTX |
| 1026 | 4 x AMD Radeon RX 7900 XTX |
| 1027 | 1 x AMD Instinct MI100 |
| 1028 | 2 x AMD Instinct MI100 |
| 1029 | 4 x AMD Instinct MI100 |
| 1030 | 1 x AMD Instinct MI200 |
| 1031 | 2 x AMD Instinct MI200 |
| 1032 | 4 x AMD Instinct MI200 |
| 2000 | MACBOOK PRO M2 (Metal) |
| 2001 | MACBOOK PRO M3 (Metal) |


#### Node model subscription

In order to subscribe to a given model, the node operator can run the following command

```sh
./cli db add-node-to-model \
    --package "0x8fc663315a07208e86473b808d902c9b97a496a3d2c3779aa6839bd9d26272b8" \
    --model "MODEL" \
```

Notice that once a node subscribes to a given model, it is entitled to execute requests for that specific model.
It the node doesn't host the model, node submission will timeout. This means that part of the node's submitted collateral 
will be slashed for timeout. In order to avoid this, the node operator must be sure to subscribe to only host to models
it currently hosts.

The available list of supported models is:

| Model Type                         | Hugging Face model name                  |
|------------------------------------|------------------------------------------|
| falcon_7b_f16                      | tiiuae/falcon-7b                         |
| falcon_7b_bf16                     | tiiuae/falcon-7b                         |
| falcon_40b_f16                     | tiiuae/falcon-40b                        |
| falcon_40b_bf16                    | tiiuae/falcon-40b                        |
| falcon_180b_f16                    | tiiuae/falcon-180b                       |
| falcon_180b_bf16                   | tiiuae/falcon-180b                       |
| flux_dev_f16                       | black-forest-labs/FLUX.1-dev             |
| flux_dev_bf16                      | black-forest-labs/FLUX.1-dev             |
| flux_schnell_f16                   | black-forest-labs/FLUX.1-schnell         |
| flux_schnell_bf16                  | black-forest-labs/FLUX.1-schnell         |
| llama_v1_f16                       | Narsil/amall-7b                          |
| llama_v1_bf16                      | Narsil/amall-7b                          |
| llama_v2_f16                       | meta-llama/Llama-2-7b-hf                 |
| llama_v2_bf16                      | meta-llama/Llama-2-7b-hf                 |
| llama_solar_10_7b_f16              | upstage/SOLAR-10.7B-v1.0                 |
| llama_solar_10_7b_bf16             | upstage/SOLAR-10.7B-v1.0                 |
| llama_tiny_llama_1_1b_chat_f16     | TinyLlama/TinyLlama-1.1B-Chat-v1.0       |
| llama_tiny_llama_1_1b_chat_bf16    | TinyLlama/TinyLlama-1.1B-Chat-v1.0       |
| llama3_8b_f16                      | meta-llama/Meta-Llama-3-8B               |
| llama3_8b_bf16                     | meta-llama/Meta-Llama-3-8B               |
| llama3_instruct_8b_f16             | meta-llama/Meta-Llama-3-8B-Instruct      |
| llama3_instruct_8b_bf16            | meta-llama/Meta-Llama-3-8B-Instruct      |
| llama3_70b_f16                     | meta-llama/Meta-Llama-3-70B              |
| llama3_70b_bf16                    | meta-llama/Meta-Llama-3-70B              |
| mamba_130m_f16                     | state-spaces/mamba-130m                  |
| mamba_130m_bf16                    | state-spaces/mamba-130m                  |
| mamba_370m_f16                     | state-spaces/mamba-370m                  |
| mamba_370m_bf16                    | state-spaces/mamba-370m                  |
| mamba_790m_f16                     | state-spaces/mamba-790m                  |
| mamba_790m_bf16                    | state-spaces/mamba-790m                  |
| mamba_1-4b_f16                     | state-spaces/mamba-1.4b                  |
| mamba_1-4b_bf16                    | state-spaces/mamba-1.4b                  |
| mamba_2-8b_f16                     | state-spaces/mamba-2.8b                  |
| mamba_2-8b_bf16                    | state-spaces/mamba-2.8b                  |
| mistral_7bv01_f16                  | mistralai/Mistral-7B-v0.1                |
| mistral_7bv01_bf16                 | mistralai/Mistral-7B-v0.1                |
| mistral_7bv02_f16                  | mistralai/Mistral-7B-v0.2                |
| mistral_7bv02_bf16                 | mistralai/Mistral-7B-v0.2                |
| mistral_7b-instruct-v01_f16        | mistralai/Mistral-7B-Instruct-v0.1       |
| mistral_7b-instruct-v01_bf16       | mistralai/Mistral-7B-Instruct-v0.1       |
| mistral_7b-instruct-v02_f16        | mistralai/Mistral-7B-Instruct-v0.2       |
| mistral_7b-instruct-v02_bf16       | mistralai/Mistral-7B-Instruct-v0.2       |
| mixtral_8x7b_f16                   | mistralai/Mixtral-8x7B-v0.1              |
| mixtral_8x7b_bf16                  | mistralai/Mixtral-8x7B-v0.1              |
| phi_3-mini_f16                     | microsoft/Phi-3-mini-4k-instruct         |
| phi_3-mini_bf16                    | microsoft/Phi-3-mini-4k-instruct         |

#### Atoma's request submission


##### Text Prompt Request

To submit a text prompt request to the Atoma network, say on Llama3.18b instruct model, while sampling 3 nodes for verifiability, a user can run the following command:

```sh
./cli gate send-prompt-to-ipfs \
    --package "your package id can be found when publishing" \
    --model "llama3_8b_instruct" \
    --prompt "YOUR_PROMPT" \
    --max-tokens 512 \
    --max-fee-per-token 1 \
    --nodes-to-sample 3
```

The above command will submit a text prompt request to the Atoma network and print the corresponding transaction digest, the output text will be stored on IPFS and the user can retrieve it with the correct IPFS `cid`. We also
support storage on Gateway. To do so, the user can run the following command:

```sh
./cli gate send-prompt-to-gateway \
    --package "your package id can be found when publishing" \
    --model "llama3_8b_instruct" \
    --prompt "YOUR_PROMPT" \
    --max-tokens 512 \
    --max-fee-per-token 1 \
    --gateway-user-id "YOUR_GATEWAY_USER_ID" \
    --nodes-to-sample 3
```

Where you need to provide your Gateway user ID, which you have set once registering to Atoma Gateway portal.

##### Image Prompt Request

###### Image Prompt Request to IPFS

To submit an image prompt request to the Atoma network, say on Flux-dev model, while sampling 3 nodes for verifiability, a user can run the following command:

```sh
./cli gate send-image-prompt-to-ipfs \
    --package "your package id can be found when publishing" \
    --model "flux_dev" \
    --prompt "YOUR_PROMPT" \
    --height 512 \
    --width 512 \
    --max_fee_per_input_token 1 \
    --max_fee_per_output_token 1 \
    --nodes-to-sample 3
```

where `max_fee_per_input_token` and `max_fee_per_output_token` are the maximum fees to be paid to nodes per text input token and output image pixel, respectively.

###### Image Prompt Request to Gateway

To submit an image prompt request to the Atoma network, say on Flux-dev model, while sampling 3 nodes for verifiability, a user can run the following command:

```sh
./cli gate send-image-prompt-to-gateway \
    --package "your package id can be found when publishing" \
    --model "flux_dev" \
    --prompt "YOUR_PROMPT" \
    --height 512 \
    --width 512 \
    --max_fee_per_input_token 1 \
    --max_fee_per_output_token 1 \
    --gateway-user-id "YOUR_GATEWAY_USER_ID" \
    --nodes-to-sample 3
```

Where you need to provide your Gateway user ID, which you have set once registering to Atoma Gateway portal.


<!-- List of References -->

[github-sui-std]: https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/packages/sui-framework/sources
[sui-install]: https://docs.sui.io/guides/developer/getting-started/sui-install
[sui-analyzer]: https://marketplace.visualstudio.com/items?itemName=MoveBit.sui-move-analyzer
[sui-explorer]: https://explorer.sui.io
