# Why decentralized AI will consume all available block space ?

## Abstract

In this paper, we will analyze a few trends in the growth of generative AI models and compute, following the Situational Awareness paper. Following the growth of the Bitcoin network, in terms of both compute and hardware, we will also explore the potential of a decentralized AI network, such as the Atoma Network, to accommodate the future AI growth.

In a second part of the paper, following the trends in AI previously described, we will justify why decentralized AI networks need to completely decouple the execution layer from a settlement layer. Therefore, nodes in a decentralized AI network should not be responsible for producing blocks, to maintain the state of a global ledger. Instead, we will conclude that decentralized AI networks should rely on existing blockchains as a settlement layer. We will also explore why decentralized AI networks do not require specialized block space, instead they should only care about opaque block space on a set of blockchains, contrary to most dApps in Web3.

Moreover, under the lens of current AI demand, we will justify why in the future, decentralized AI networks will consume all current available block space.

## Introduction

We are inspired by Leopold Aschenbrenner's [Situational Awareness](https://situational-awareness.ai/wp-content/uploads/2024/06/situationalawareness.pdf) paper, which analyzes the trends in the growth of AI and compute in the coming years, based on historical data. In this paper, we will explore how, if Leopold's predictions materialize, decentralized AI networks will be able to accommodate for the future AI growth.

## Current and Future AI trends

### Analysing Current AI Models 

To understand how decentralized AI gain can solve the impending challenges of AI growth, it is important to first understand the landscape of current AI models and the expected trajectory of developements in the field over the coming years. 

### GPT-4

At the time of writing, GPT-4 stands at the forefront of large language models with roughly 1.76 trillion total paramaters. To run GPT-4 at a bf16 precision requires roughly 3.52 trillion bytes of memory. This is achievable with roughly 40 Nvidia H100 80GBs memory GPUs working in coordination. Nvidia H100 80 GBs GPUs cost ~$31,000 per card meaning to run 40 models would cost ~$1.24million dollars on solely acquiring the harwardware.

Sam Altman stated the cost of training GPT-4 was more than $100million. This entailed 90 days of training on 25,000 Nvidia A100 GPUS in 128 A100 GPU clusters. GPT-4 was trained on a dataset of ~13 trillion tokens (web text, books, other) https://klu.ai/blog/gpt-4-llm
It is estimated that GPT-4 serves between 15k requests per second (on average) and can peak to 500,000 requests per second.

The development of the GPT model's capabilities and accuracy has been staggering. Looking back to 2019, we saw the release of GPT-2 which has roughly 1.5B model parameters. GPT-2 to GPT-4 saw a scale up of effective compute of roughly 100,000x, courtesy of improved hardware and algorithmic optimizations. 

This development of GPT-2 to GPT-4 is the equivalent of a primary school student to a smart high school student level of intelligence. It is expected that by 2028, the newest models will scale up 100,000x from the current effective compute of GPT-4 and intelligence will be at the level of PhD researchers and field experts on a broad range of topics. 

(Need to reference data source here) 
The scale up is possible as the current data shows that the efficiency of effective compute doubles roughly every 8 months and the available compute to train large language models increases roughly by 3x a year. The newest release GPT-4o prices have fallen 4-6x compared to GPT-4, mostly due to algorithmic improvements and deploying newer, more efficient GPUs as soon as they are acquired. These newer GPUs result in much greater throughput meaning more tokens in less time using less electricity and therefore the models can be cheaper. 

### LLama3 

Llama3 is the leading open source LLM and currently operates models with 8B and 70B parameters. LLama3 is pretrained on over 15T tokens from publicly available sources with a training dataset seven times larger than what was used for Llama2, including four times more code. Previous generations of Llama were good at identifying high quality data and therefore Llama2 was used for text-qualifying qualifiers in powering Llama3. 

https://ai.meta.com/blog/meta-llama-3/
https://llama.meta.com/llama3/
https://engineering.fb.com/2024/03/12/data-center-engineering/building-metas-genai-infrastructure/

The expectation is that LLama3 400B will reach parity with GPT-4. In order to serve a 400B model with bf16 precision requires ~5 Nvidia H100 80GBs. To reach the level of intelligence that GPT-4 has set the standard for in this case would only cost $155,000 for acquiring hardware. In terms of benchmarking, expectations of Meta Llama3400B+(Instruct) measures at 86.1 MMLU while GPT-4o averaged 88.7. 

### Benchmarking 

A benchmark called MMLU was developed in 2020 to accomodate analysing the rapid evolution of these models. GPT-4 mostly solves all standard US high school and college aptitude tests. https://arxiv.org/pdf/2009.03300

New benchmarking test like GPQA consist of PhD level biology, chemistry, and physics questions. Most of these questions are incomprehensible to people who are not experts in the respective fields and even PhDs in similar or adjacent fields require significant time and research to score above random chance in a test. Claude3 Opus currently scores ~60% compared to in-domain PhDs who achieve 80%. Aschenbrenner expects this benchmark will fall in the next generation. https://arxiv.org/pdf/2311.12022

### Data Wall / Training Data 

Models don't have an unlimited pool of resources to use for training. Simply put, improving models isn't just a case of longer training on more data, as mentioned with Llama3, the datset it was trained on was substantical already. 

Models can become more efficient with training data to reflect on the content they are trained on. This in turn could lead to a demand for inference services to generate high quality synthetic data which also comes with its own security and adversarial risks. 

### Unhobbling

Providing models with retrieved data for augment generation (i.e vector databases, knowledge graphs, highly catered data) improves performance. This will increase the demand of AI inference. 


### Spending Scaleup and Hardware Developments:

The speed of improvement of AI models not only lies in algorithmic effiency and training optimizations but also in the accessible hardware. Companies are investing heavily in high performance hardware such as GPUs and TPUs, to keep up demand of larger and more complex AI models. Leopold Aschenbrenner estimates that in 2024 there will already be $100B-$200B of AI specific hardware investment and envisions a "trillion dollar supercluster" by the end of the decade. Whether this is feasible remains to be seen due to constraints on what big businesses / institutions can afford to invest. 

Moore's law observes that the number of transistors on a computer chip doubles roughly every two years, typically meaning that compute power also doubles. As mentioned previously, the efficiency of effective compute is doubling every 8 months meaning that AI hardware is improving beyond Moore's law due to specializing chips for AI workloads. 

This direction of specialization for AI has included moving from CPUs to GPus, creating adapted chips for transformers, and much lower precision number formats from fp64/fp32 for traditional supercomputing to fp8 on H100s. By end of decade, there will likely be specialized chips solely purposed for AI inference. 

We'll explore bridging this gap of available hardware and efficiently utilizing existing compute by decentralizing AI. 

### Transparency 

A final thought which we'll dive into in more detail in the next section is that key research and development findings are becoming increasingly proprietrary and AI labs could own the space if this reseach remains behind closed doors. It's crucial that research and compute resources are available at cheap prices and are transparent to the public. 




 
