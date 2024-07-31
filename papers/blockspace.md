# Why decentralized AI will consume all available block space ?

\maketitle 

\begin{abstract}
Inspired by the recent work of Leopold Aschenbrenner on Situational Awareness, we will analyze the potential for a decentralized AI protocol, such as the Atoma Network, to aggregate enough hardware resources to serve the next generation AI applications. 

Our analysis is inspired by the historical growth of the Bitcoin network, which at the time of writing processes roughly 540 exahashes per second of total aggregated compute. We further explain why the design of Atoma, as an execution layer fully decoupled from a settlement layer, allows for ease of node deployment and naturally facilitates both horizontal and vertical network scaling.

We also explain why we believe that a certain percentage of the current AI demand will be captured by decentralized networks, as the latter provide lower inference costs, high verifiable guarantees and to foster open source development and free flow of ideas. 

Finally, following along the trends for increase in AI capabilities and demand, we will explain explain why in the scenario that decentralized networks capture part of such demand it will likely lead to scarcity of block space, across most blockchains. We will also explain how this might necessarily lead to a bottleneck for decentralized AI services, as the latter only need to rely on free 'opaque' block space. 
<!-- 
In a second part of the paper, following the trends in AI previously described, we will justify why decentralized AI networks need to completely decouple the execution layer from a settlement layer. Therefore, nodes in a decentralized AI network should not be responsible for producing blocks, to maintain the state of a global ledger. Instead, we will conclude that decentralized AI networks should rely on existing blockchains as a settlement layer. We will also explore why decentralized AI networks do not require specialized block space, instead they should only care about opaque block space on a set of blockchains, contrary to most dApps in Web3.

Moreover, under the lens of current AI demand, we will justify why in the future, decentralized AI networks will consume all current available block space. -->
\end{abstract}

\section{Introduction}
We are inspired by Leopold Aschenbrenner's [Situational Awareness](https://situational-awareness.ai/wp-content/uploads/2024/06/situationalawareness.pdf) paper, which analyzes the trends in the growth of AI and compute in the coming years, based on historical data. In this paper, we will explore how, if Leopold's predictions materialize, decentralized AI networks will be able to accommodate for the future AI growth.

\section{Current and Future AI trends}

\section{Bitcoin Network Growth}

\section{Decentralized AI}

As we previously discussed, over the past decade, the Bitcoin network witnessed an exponential compute growth. This growth was mostly driven by the expectation that Bitcoin can serve as a world reserve currency, given its tokenomics. 

In this section, we will explore if the demand for AI services, which includes AI inference, fine tuning and retrieved augmented generation, can drive exponential growth in a decentralized AI network. In order for it to succeed, such a protocol must be able to aggregate enough specialized and collocated compute worldwide, as well as offer reduced prices compared to alternative centralized services and access to private AI inference, etc. If this materializes, we believe that the decentralized AI will be able to compete in the world of AI.

Gather enough compute worldwide is a difficult task, per se. Moreover, the type of compute for AI is highly specialized and is in high demand. Recent GPUs for AI compute have high costs, compared to previous models, a scenario exacerbated by current geo-political factors. Furthermore, AI chip manufacturing is a highly competitive market, mostly dominated by a few selected companies. That said, that is no different than the required hardware for Bitcoin mining, which is highly specialized for SHA-256 hashing. If a decentralized network such as Atoma can offer high demand for such compute, we believe that it will be able to attract enough node participants to see an exponential growth in available compute, similar to that of Bitcoin.

Moreover, we believe that such scenario can be greatly propelled by a combination of factors such as: an architecture highly specialized for AI compute which allows for high hardware utilization rates, a robust tokenomics and a rich AI application ecosystem built on top of it.

\subsection{Atoma's Architecture}

We have designed the Atoma Network as a decentralized protocol that is fully decoupled from any settlement layer. That is, nodes on Atoma are not responsible for proposing and propagating new blocks and are neither responsible for maintaining a global state of a ledger. Instead, the settlement layer can be fully delegated to existing blockchains, given the high number of free available block space, at the time of writing. 

Instead, nodes on Atoma are fully specialized for AI inference. This means that nodes can acquire specialized hardware for AI acceleration, which can provide a considerable edge in accruing fees by processing more requests over the network. Moreover, the fact that the network does not require nodes to share any state among themselves, and to synchronize such state at given time intervals, it makes it possible for a highly heterogeneous compute layer, meaning that nodes can have completely different types of hardware at their disposal, and leverage whichever type of hardware it can give them a market advantage. 

\begin{remark}
    The fact that the network allows for an heterogeneous compute layer, it means that processing AI requests becomes highly competitive. This observation follows from the fact that smaller models being able to be fully served by less powerful hardware (for example, MacBook Pros or gaming GPUs), whereas highly specialized compute will be dedicated to serve only the largest available AI models.
\end{remark}

Moreover, fully decoupling the settlement layer from the compute layer allows for a great reduction in bandwidth costs that nodes incur in most traditional decentralized protocols. The latter means that nodes can serve AI requests with a low latency, allowing these to process a much higher number of requests in the long term, which leads to a substantial reward accumulation. 

Furthermore, by reducing the communication between nodes on the network, it is possible that each node processes requests independently of other nodes. Therefore, the network can scale horizontally with the number of available nodes on the network. It also follows, as a conclusion of the latter remark, that nodes can leverage the best LLM inference optimization algorithms, such as Flash and Paged Attention, in order to continuously batch a higher number of requests with high efficiency, allowing them to fully utilize their hardware resources. Therefore, the network can scale vertically with better hardware available on the network (that is, it scales linearly with the increase of GPU cores available).

Finally, any decentralized network of nodes competing among themselves to serve customers needs to have a verifiability mechanism, otherwise nodes will invariably cheat the protocol. The Atoma Sampling Consensus was specially crafted for a reduced operational costs, close to that of native inference, through robust game theoretical guarantees. Moreover, once enough specialized hardware becomes available (such as Nvidia Blackwell and Hopper architecture GPUs), it will be possible to have full verifiability at native inference cost. 

The aspects above combined, naturally lead to a decentralized protocol, with high integrity guarantees, offering AI services on pair with most of the most optimized centralized AI inference services. Nodes on the network will be able to better utilize their compute resources, as well as accruing more fees in the long run. This creates a highly lucrative business for node operators, allowing them to acquire better resources over time, enhancing the network throughput capacity.

\subsection{Tokenomics}

It is important to consider that a decentralized network can deploy a token emission rate which can be utilized to subsidize node operators to acquire better compute resources, allowing these to scale their hardware. Through a subsidy mechanism it is possible to aggregate better compute, allowing the network to become increasingly more valuable, which compensates for a high inflation rate of the token, in the initial phase of the network. Subsidizing compute will also lead to a more competitive fair market value for AI inference, as more nodes can participate in the network, establishing free market mechanisms.

Once the network has bootstrap enough compute, it is a highly likely scenario that demand will naturally follow as at this phase the network will be able to offer very competitive pricing for AI services. This is especially relevant, given the high margins that both hardware manufacturers and cloud providers charge for use of their specialized hardware, such as high caliber GPUs.

We will fully detail the Atoma Network tokenomics in a latter document. 

\subsection{Application Layer}

In order to create enough demand for aggregated compute, we plan to foster a rich open source ecosystem of AI development. Through a community treasury fund, it is possible to incentivize builders and developers to create innovative applications on top of Atoma. Moreover, developers can leverage from lower costs and high integrity guarantees that their applications can leverage the best open source AI models. 

Moreover, we expect that with the likely scenario of broad available confidential compute, privacy will become high available on the Atoma Network, through the use of TEEs. The latter can unlock the full potential of AI applications, leading developers to leverage the high demand for data privacy. The latter might be specially relevant, if cloud providers cannot compete within the market for data privacy.

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




 
