# Atoma Node optimizations

## Atoma vLLM

It is possible to substantially improve Atoma's nodes performance for AI LLM inference. We are in the process of integrating vLLM and Paged Attention into Atoma's infrastructure, with the goal of having highly optimized memory management of incoming requests and continuously batching such requests for maximum GPU utilization.

vLLM works by scheduling incoming requests depending on the arrival order and size. Once a request is ready to be run, the Atoma node can batch it with previously scheduled requests and run a single forward pass on multiple batched requests. It is then able to generate the next prediction token of multiple requests simultaneously.

Through these optimizations, Atoma can become a highly performant decentralized AI compute network, as well as reducing the cost of inference, reducing latency times, and improving nodes GPU utilization. Ultimately this leads to higher accrued rewards for nodes and a better overall user experience.

## Quantization techniques

Through the use of quantization techniques, Atoma can reduce the memory footprint of LLM inference by reducing the number of bits used to represent the model weights and activations. This allows for more efficient memory utilization and faster inference times, leading to improved performance and reduced costs. It is important to note that the accuracy of the model may be impacted as the reduced precision may result in a loss of information.

We have introduced quantization support into Atoma using the llama.cpp quantized types. We are also exploring other quantization techniques such as 1.58-bit quantization. From empirical observations, it seems that lower-bit quantization techniques do not have such a high impact for larger models. This is especially relevant as larger models usually require clustering of multiple GPUs to be able to load memory. Reducing the memory footprint of LLM inference is a key factor allowing for nodes to be able to host larger models at a lower cost and at scale.
to be able to host larger models, at a lower cost and at scale.

## Multi-GPU support through Fully Sharded Data Parallelism

We have introduced fully sharded data parallelism into Atoma, which allows for the efficient utilization of multiple GPUs. This is achieved by sharding the model weights and activations across multiple GPUs, allowing for each GPU to process a portion of the model. This approach allows running models as large as the total GPU aggregated memory. We are in the process of integrating multi-GPU support into Atoma's vLLM implementation, which allows for highly efficient utilization of multiple GPUs for LLM inference.
