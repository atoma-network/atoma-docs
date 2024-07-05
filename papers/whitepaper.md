---
title: Atoma Network Whitepaper
author: Jorge António and Hisham Khan
---

# Abstract

The Atoma Network is a decentralized and permissionless protocol for verifiable AI inference. Atoma relies on compute providers, referred to as execution nodes (or simply nodes), which are responsible for hosting and running AI models, in order to process user-defined requests.

Atoma will serve a crucial role as an AI execution layer for Web3 protocols. It will integrate AI capabilities into smart contracts, compensating for blockchains' inability to perform heavy computations such as AI inference. In addition to enhancing smart contracts, new applications will be built on Atoma, such as chat applications specific to Web3 protocols, AI-driven market prediction platforms that analyze and forecast market trends using advanced algorithms, Web3 AI-enhanced wallets specialized for user intent interactions, knowledge bases as public goods, social discussion forums, DAO and Network States governance, etc.

In a world where AI will play the role of our caretaker, tutor, personal assistant, and co-worker, it will be crucial that these applications have high computing integrity guarantees.

For this purpose, we have designed a new consensus protocol, which we refer to as *Sampling Consensus*. Our protocol is based on the assumption that the nodes are rational actors in a competitive economic environment. The protocol is optimized to achieve very high compute integrity guarantees whilst having a lower cost compared to other methods (such as zkML or opML).

Using its innovative approach, the network aims to foster a future characterized by increased transparency, innovation, and democratic principles, ensuring that technology fully aligns with the needs and values of users and communities.

# Table of Contents
1. [Introduction](#introduction)
2. [Atoma as a decentralized permissionless protocol for AI inference](#atoma-as-a-decentralized-permissionless-protocol-for-ai-inference)
   - [Atoma's node registry](#atomas-node-registry)
   - [Node collateral, slashing and rewards](#node-collateral-slashing-and-rewards)
   - [AI model subscription](#ai-model-subscription)
   - [Serving requests on Atoma](#serving-requests-on-atoma)

# Introduction

**Current and future trends in AI:** The past few years have seen the emergence of new generative AI capabilities powered by large language models (LLMs), followed by an exponential increase in compute throughput at lower costs, through better hardware.

Following OpenAI's release of ChatGPT, we have seen better, more capable models being released at a tremendously fast pace. These include both proprietary models (the likes of GPT, Anthropic, etc.) and open-source models (such as Llama, Mixtral, Qwen, etc.).

This growth trend is expected to continue over the coming years, leading to a fast-growing market. For example, McKinsey estimates that generative AI could lead to worldwide economic benefits between \$2.6 trillion and \$4.4 trillion annually in a wide variety of use cases. By comparison, the total GDP of the United Kingdom in 2021 was \$3.1 trillion.

This expected growth will be accompanied by an increase in the demand for computing and energy resources. Through the first half of 2024, we have seen experts in the field highlighting the need for governments and big tech companies to cooperate in order to build the next generation trillion dollar compute clusters, as in Leopold Aschenbrenner's most recent work *Situational Awareness*, see [Situational Awareness](https://example.com).

Whether this trillion-dollar compute cluster will materialize in the future remains to be seen. However, it is clear that there will be an increasing demand for compute and energy resources over the coming years. AI pipelines require specialized hardware, such as GPUs and TPUs, and while GPUs are widely accessible to the public (mostly for gaming purposes), the type of GPUs required for AI tasks are of a higher caliber and considerably more expensive. The fact that there are only a few GPU manufacturers (such as Nvidia and AMD) combined with the current geopolitical tensions leads to serious disruptions in the supply chains of chip manufacturing. Consequently, this results in difficulties making high-caliber GPUs for AI accessible to retail and small- to medium-sized data center businesses.

If this trend continues, we will step into a future in which big tech giants retain most of the world's available compute resources, in which case, these companies will have a complete monopoly over most of AI development from model training to AI model deployment and inference. Moreover, in order to preserve their margin of profit and/or their intellectual property, these will push AI towards becoming a closed-source technology (meaning model weights are not available to the general public).

Combining the fact that a few companies have access to virtually all of the real-world data being generated, we could see extreme centralization of the most revolutionary technology of our time, generative AI. Furthermore, such a high centralization risks posing a threat to AI alignment with user needs and values (AI misalignment). This misalignment has already been observed following the release of the highly biased Google Gemini image generation model [Google Gemini](https://cnbc.com).

**Democratizing AI through Atoma:** Decentralized protocols provide the right framework to mitigate the above risk factors.

Using cryptoeconomic principles and decentralization, decentralized networks have demonstrated their ability to aggregate vast amounts of computational power. For instance, as of the time of writing, the Bitcoin network is capable of processing approximately 500 requests per second. According to the conversion rate between hashes and FLOPS described in [Wikipedia FLOPS](https://en.wikipedia.org/wiki/Talk%3AFLOPS#Bitcoin_.22FLOPS.22_computation_on_bitcoinwatch), this translates to approximately $6.35 \times 10^9$ TFLOPs per second. In contrast, training GPT-4, one of the most extensive models released to date, required about $4 \times 10^{13}$ TFLOPs in total, as detailed in [Situational Awareness](https://example.com). This illustrates that decentralized networks can effectively coordinate rational agents to aggregate sufficient computational resources to tackle the most demanding AI tasks currently available. In fact, in roughly 14 days, Bitcoin can generate as many TFLOPs as are required to train GPT-4.

Blockchains are unsuitable for running AI inference on the largest language models (LLMs) due to their limited execution runtime environments. Executing AI inference on each node of a blockchain, or simply on a supramajority of these, incurs prohibitive costs and significant bandwidth usage. Furthermore, most blockchain execution environments are highly constrained and typically only capable of processing transactions of a few hundred kilobytes. Even networks with more capable execution layers, such as ICP or AO, cannot handle LLMs as large as a 70b Llama3 model, [Internet Computer](https://internetcomputer.org/whitepaper.pdf) and [AO Protocol](https://5z7leszqicjtb6bjtij34ipnwjcwk3owtp7szjirboxmwudpd2tq.arweave.net/7n6ySzBAkzD4KZoTviHtskVlbdab_yylEQuuy1BvHqc), let alone future models, which may be orders of magnitude larger.

In order to resolve the issues with blockchain technologies for AI inference, we have designed the Atoma Network, a decentralized protocol specialized in verifiable AI inference. Following the philosophy of modular blockchains, Atoma decouples the execution layer from settlement. Through Atoma, it is possible to aggregate nodes to run any sort of AI inference.

![Blockchains lack execution environments capable of handling heavy computations such as AI inference.](image6.png)

Atoma nodes will be able to monetize their compute resources in a permissionless and transparent environment while being exposed to fair market economic incentives. This will allow for a more fair and transparent access to compute resources around the world. Moreover, Atoma's infrastructure is highly optimized for efficient GPU utilization, incorporating the most advanced LLM memory management techniques (such as FlashAttention, PagedAttention, quantization techniques, etc.). These optimizations will allow the network to scale both horizontally with the number of available nodes on the network and vertically, with better hardware becoming available on the network.

This is especially relevant given the current GPU chip shortage, as hardware providers face sunk costs by not using their infrastructure efficiently.

On the demand side, Atoma will provide a platform to build next-generation applications that are at the center of AI and Web3. These will include UI chat applications, Web3 wallet enhancements through LLMs (enabling user wallet interactions through declarative intents), online data scraping, next-generation browsers and browser extensions, content generation, community-based knowledge bases, social media platforms, and more.

Additionally, Web3 protocols will gain access to AI functionality through the network, providing a full intelligence layer to dApps.

In a decentralized environment, applications cannot rely solely on a single party for operation without additional integrity guarantees, due to the risk of misbehaving nodes. Consequently, these applications must undergo rigorous integrity checks. We use the term *verifiable AI inference* to refer to any AI inference computation that offers high integrity guarantees.

There are a few approaches to verifiable AI, including zkML and opML. However, we have developed a novel consensus protocol, referred to as *Sampling Consensus*, specialized for heavy computations (which includes AI inference). We believe that our consensus protocol is better suited for verifiable AI than the alternatives.

The Sampling Consensus protocol is elastic, allowing users and protocols to decide the output integrity guarantees that their use case requires. The advantage is that end-customers pay fees proportional to the integrity level they choose for what their application requires.

Whereas a DeFi protocol may require almost 100% certainty that a given AI market liquidity prediction was computed correctly, as even a small deviation from the original prediction can have
