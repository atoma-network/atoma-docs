# Atoma Node

Atoma Nodes are the backbone of the Atoma's decentralized compute layer. They are responsible for executing AI workloads and providing compute resources to the network. Nodes are rewarded with native TOMA tokens for their contributions to the network.

In this section, we will explain how to set up a node and connect it to the Atoma, so anyone with available computing resources can participate in the network.

**Note:** The Atoma Node is currently under development and the following documentation is for Atoma's alpha release sole purpose.

## Requirements

- Have Rust and Cargo installed, for more details please consult [here](https://www.rust-lang.org/tools/install).
- Have a machine with either one or more Nvidia GPUs, or any MacBook Pro that supports Metal.
- For Nvidia GPUs, it is recommended to have a at least CUDA 12.1 installed.
- In order to use optimized CUDA kernels, with Flash Attention2 support, it is recommended to have a Nvidia Ampere or newer GPU architecture (see more details below).
- It is possible to run the Atoma Node relying solely on a CPU, even though the performance will likely be far from optimal.
- Clone the Atoma's node [repository](https://github.com/atoma-network/atoma-node).
- An Hugging Face API key, to download the models used for inference.
- For IPFS compatibility, it is recommended to have a local IPFS node running (see more details below).
- For Gateway's compatibility, it is recommended to have created a Gateway account and have access to a valid API key (see more details [here](https://docs.mygateway.xyz/developer-guide/api-reference/authentication)).
- To support the current Atoma native UI application, it is recommended to have a Supabase account and access to a valid API key (see more details [here](https://supabase.com/docs/reference/javascript/auth-signup)).
- Have a Sui wallet, and some `SUI` tokens in it. Please visit the Sui official CLI [website](https://docs.sui.io/references/cli/client) to install the Sui CLI and follow the instructions to set up a wallet.
- Have Atoma's native faucet `TOMA` token available on your Sui wallet. You can request faucet tokens here [here]().

## Configuration 

After you have installed Rust and Cargo, and have cloned the Atoma's node repository, you are required to specify a set of configuration parameters that will allow the node to connect to the Atoma Network. We recommend you to create a `config.toml` file in the root of the Atoma node's repository with the following parameters:

```toml
[client]
config_path = "YOUR_LOCAL_PATH_TO_SUI_CONFIG_FILE"
atoma_db_id = "0x0ee50a4ef345ffec5c58906e7f6a7f569fddbf6696c3d7b5b305b72e2683f304"
node_badge_id = "0xbc093a0daf2d5ba7ed287a7e1cf4fac6973523beca462531174647588bfcc4ec"
package_id = "0x8fc663315a07208e86473b808d902c9b97a496a3d2c3779aa6839bd9d26272b8"
small_id = YOUR_SMALL_ID

max_concurrent_requests = 1000

[client.request_timeout]
secs = 300
nanos = 0

[inference]
api_key = "HUGGING_FACE_API_KEY"
cache_dir="./models"
flush_storage=true
tracing=true
jrpc_port=INFERENCE_JRPC_PORT

[[inference.models]]
device_ids = [YOUR_DEVICE_IDS]
dtype="DTYPE"
model_id="HOST_MODEL_ID"
revision="HOST_MODEL_REVISION"
use_flash_attention=BOOLEAN

[input_manager]
firebase_url = "https://atoma-testing-default-rtdb.europe-west1.firebasedatabase.app"
firebase_email = "testing@atoma.network"
firebase_password = "testing"
firebase_api_key = "YOUR_FIREBASE_API_KEY   "
ipfs_api_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9."
ipfs_port = IPFS_DAEMON_PORT
small_id = YOUR_SMALL_ID

[output_manager]
firebase_url = "https://atoma-testing-default-rtdb.europe-west1.firebasedatabase.app"
firebase_email = "testing@atoma.network"
firebase_password = "testing"
firebase_api_key = "YOUR_FIREBASE_API_KEY"
gateway_api_key = "YOUR_GATEWAY_API_KEY"
gateway_bearer_token = "YOUR_GATEWAY_BEARER_TOKEN"
ipfs_port = IPFS_DAEMON_PORT
small_id = YOUR_SMALL_ID

[event_subscriber]
http_url = "SUI_RPC_PROVIDER_HTTP_URL"
ws_url = "SUI_RPC_PROVIDER_WS_URL"
package_id = "0x8fc663315a07208e86473b808d902c9b97a496a3d2c3779aa6839bd9d26272b8"
small_id = YOUR_SMALL_ID

[event_subscriber.request_timeout]
secs = 300
nanos = 0

[streamer]
firebase_url = "https://atoma-testing-default-rtdb.europe-west1.firebasedatabase.app"
firebase_email = "testing@atoma.network"
firebase_password = "testing"
firebase_api_key = "YOUR_FIREBASE_API_KEY"
small_id = YOUR_SMALL_ID
```

In order to fill the above configuration file, you will need to:

1. Fill the Sui `client` parameters with the path to your Sui config file, together with your node registration small id. In order to register your node on the Atoma contract, please follow the instruction below. As an example, if my Sui config file is located at `~/.sui/sui_config/client.yaml` (standard file path), and your registration node small id is 1234, your `config.toml` file should look like this:

```toml
[client]
config_path = "~/.sui/sui_config/client.yaml"
atoma_db_id = "0x0ee50a4ef345ffec5c58906e7f6a7f569fddbf6696c3d7b5b305b72e2683f304"
node_badge_id = "0xbc093a0daf2d5ba7ed287a7e1cf4fac6973523beca462531174647588bfcc4ec"
package_id = "0x8fc663315a07208e86473b808d902c9b97a496a3d2c3779aa6839bd9d26272b8"
small_id = 1234
```

2. Fill the `inference` parameters with your Hugging Face API key, and the model you want to use for inference. Your node must download a specific model weight from Hugging Face. Your node will download the model weights to the `./models` directory, and will use the `DTYPE` and `USE_FLASH_ATTENTION` parameters to select the best inference configuration to serve the specific model inference. Please refer to our [supported model page]() for more information about the supported models, for the current alpha release.

If you have a machine with an Nvidia GPU with CUDA 12.1 or newer and an Ampere or newer GPU architecture, you can use the optimized CUDA kernels and Flash Attention2 by setting `USE_FLASH_ATTENTION` to `true`, otherwise you should set it to `false`. 

You can create a free Hugging Face's account page, [here](https://huggingface.co/join). You will be able to find your API key on your account page, under the `Settings` tab. 

If you wish to deploy a Llama3.1 8b instruct model for inference, with `bf16` precision (we suggest either `bf16` or `fp16` precision types for most of the models, or else some quantized precision types), you should set the `DTYPE` parameter to `bf16`. In this case, your `config.toml` file should look like this:

```toml
[inference]
api_key = "HUGGING_FACE_API_KEY"
cache_dir="./models"
flush_storage=true
tracing=true
jrpc_port=INFERENCE_JRPC_PORT

[[inference.models]]
device_ids = [0]
dtype="bf16"
model_id="llama31_instruct8b"
revision=""
use_flash_attention=true
```

The model id for a specific AI model can be found in the model's page on Hugging Face's website. For example, the model id for Llama3.1 8b instruct can be found [here](https://huggingface.co/meta-llama/Meta-Llama-3.1-8B-Instruct). If you support multiple GPU devices, you can specify which GPU device ids to use by setting the `device_ids` parameter to a list of integers, e.g. `device_ids = [0, 1]` to use the first and second GPU devices. 

With more than one GPU device, the model weights will be automatically split across the available GPUs, with tensor weight parallelism applied. For example with 1 NVIDIA RTX 3090/4090 it is possible to run a Llama3.1 8b model with precision `bf16` or `fp16`. With a NVIDIA A100 is is possible to run a Llama3.1 8b model with precision `fp32`, or a Llama3.1 70b quantized `int4` or `int8` model. With 2 NVIDIA A100 GPUs it is possible to run a Llama3.1 70b model with precision `bf16` or `fp16`, across both 2 GPUs. 

With 8 NVIDIA A100 or H100 GPUs it is possible to run a Llama3.1 405b model with precision `fp8` precision, across all 8 GPUs, whereas with 16 NVIDIA A100 or H100 GPUs it is possible to run a Llama3.1 405b model with precision `bf16` or `fp16` precision, across all 16 GPUs.

2. Fill the `input_manager`, `output_manager` and `streamer` parameters with your Firebase project credentials, and the URL to your local IPFS daemon. To run a local IPFS daemon, you can follow the instructions [here](https://docs.ipfs.tech/install/), we can also find more details below. The Supabase credentials are required for the node to support the current Atoma's alpha native UI application. This is not strictly necessary if you are only interested in contributing to the Atoma's compute layer at the smart contract level.

4. Fill the `event_subscriber` parameters with the URL to your Sui RPC provider. We suggest using a Sui RPC provider that is geographically close to your node, to reduce latency and improve performance. We recommend a few providers including [BlockVision](https://blockvision.org/), [Shinami](https://www.shinami.com/), etc.

## Run the Atoma Node

In order to run the Atoma Node, you need to run the following commands, at the root of the Atoma node's repository: 

```bash
$ cd atoma-node/
$ RUST_LOG=info cargo run --release --featurs FEATURE -- --config-path "YOUR_CONFIG_TOML_FILE_PATH"`
```

The `FEATURE` flag can be one of the following:

- `cuda`: to run the Atoma Node with CUDA support.
- `metal`: to run the Atoma Node with Metal support.
- `flash-attn`: to run the Atoma Node with Flash Attention2 support (for Ampere or newer GPU architecture).
- `cpu`: to run the Atoma Node with CPU support.

The `YOUR_CONFIG_TOML_FILE_PATH` should be the path to the `config.toml` file you created in the previous step. If you created it at the root of the Atoma node's repository, it should be `../config.toml`.

Once your node is running, you should be able to see the node's logs in the terminal. If you have set up the node correctly, you should see the node's logs starting with `INFO` within a few seconds. You can also check how long it takes for the node to load the model weights into the GPU device. 

Your node will start listening to incoming inference requests, once you have registered your node on the Atoma smart contract on the Sui network. 

## Node registration

In order to register your node on the Atoma smart contract, you need to first have some SUI tokens in your Sui wallet (whose keypair client information is specified in the `config.toml` file above). You will also need to have some faucet TOMA tokens in your wallet. You can request TOMA tokens from the Atoma faucet, [here]().

The first step is to clone the Atoma's smart contract repository, and build the Atoma's smart contract package.

```bash
$ cd ~
$ git clone https://github.com/atoma-network/atoma-contracts
$ cd atoma-contracts/sui/cli
```

To register your node on the Atoma smart contract, you need to run the following command:

```bash
$ ./cli db register-node \
    --package "0x8fc663315a07208e86473b808d902c9b97a496a3d2c3779aa6839bd9d26272b8"
```

Then you need to subscribe your node to the currrent AI model you are hosting, as follows:

```bash
$ ./cli db add-node-to-model \
    --package "0x8fc663315a07208e86473b808d902c9b97a496a3d2c3779aa6839bd9d26272b8" \
    --model "YOUR_MODEL_ID" \
    --echelon ECHELON
```

In order to replace the flag `YOUR_MODEL_ID` you can consult the right value in [here](). You can further find your `ECHELON` value by consulting [here](), depending on your GPU hardware specs.

## Flash attention requirements

It is required holding one or more a Nvidia Ampere or newer GPU architecture to use the optimized CUDA kernels and Flash Attention2. 

## CUDA requirements

We support any NVIDIA series 20xx or newer. It is recommended to have a CUDA 12.1 or newer installed driver installed. For more details about how to update your NVIDIA driver, please refer to the [NVIDIA website](https://www.nvidia.com/Download/index.aspx).

## Metal requirements

We support any Apple Silicon series M or newer. It is recommended to have a Metal version 3.0 or newer compatible machine. 

## IPFS compatibility

It is recommended to have a local IPFS node running, to store AI generated model outputs on behalf of the user, if this has requested it. It is possible to run a local IPFS node by following the steps:

1. Install the IPFS daemon by following the instructions [here](https://docs.ipfs.tech/install/command-line/#install-official-binary-distributions).
2. On a different terminal command line, run `ipfs init` to initialize the IPFS node.
3. On that same terminal command line, run `ipfs daemon` to start the IPFS daemon.

## Gateway compatibility

It is recommended to have a Gateway account, to store AI generated model outputs on behalf of the user, if this has requested it. It is possible to create a free Gateway account [here](https://www.mygateway.xyz/). Once logged in, you will be able to create a new API key by navigating to the `API Keys` section and clicking on the `Create API Key` button. We will be using the API key to authenticate requests to the Gateway API.

## Supabase compatibility

