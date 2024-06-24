# Atoma vLLM

It is possible to improve substantially Atoma's nodes performance for AI LLM inference. We are in the process of integrating vLLM and Paged Attention into Atoma's infrastructure, with the goal of having highly optimized memory management of incoming requests and continuously batching such requests for maximum GPU utilization.

vLLM works by scheduling incoming requests depending on the arrival order and size. Once a request is ready to be run, the Atoma node can batch it with previously scheduled requests and run a single forward pass on multiple batched requests. It is then able to generate the next prediction token of multiple requests simultaneously.

Through these optimizations, Atoma can become a highly performant decentralized AI compute network, as well as reducing the cost of inference, reducing latency times, and improving nodes GPU utilization. Ultimately this leads to higher accrued rewards for nodes and a better overall user experience.
