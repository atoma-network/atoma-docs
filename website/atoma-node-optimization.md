# Atoma vLLM

It is possible to improve substantially Atoma's nodes performance for AI LLM inference. We are in the process of integrating vLLM and Paged Attention into Atoma's infrastructure, with the goal of having highly optimized memory management of incoming requests and continuously batching such requests for maximum GPU utilization.

The way vLLM works is by scheduling incoming requests depending on the arrival order and of its size. Once a request is ready to be run, Atoma node can batched it with already scheduled previous requests and runs a single forward pass on multiple batched requests, being then able to generate the next prediction token of multiple requests simultaneously. 

Through this sort of optimizations, we can make Atoma a highly performant decentralized AI compute network, as well as reducing cost of inference, reducing latency times and improving nodes GPU utilization, ultimately leading to higher accrued rewards for nodes and a better overall user experience.