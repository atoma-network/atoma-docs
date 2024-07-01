![Atoma Logo](https://github.com/atoma-network/atoma-paged-attention/blob/main/assets/atoma-symbol.jpg)

# Atoma Whitepaper

## Authored by Jorge Antonio and Hisham Khan

### Abstract

The Atoma Network is a decentralized and permissionless protocol for verifiable AI inference. Atoma relies on compute providers, referred to as execution nodes (or simply nodes), which are responsible for hosting and running AI models, in order to process requests on specific user defined inputs, in a fully decentralized manner. 

<!-- 
AI inference requests are processed through nodes that run Large Language models (LLMs) on different types of inputs (text, images, video, audio) to produce some form of output (which can be either text, or other formats for multi-modality capabilities). The Atoma Network will support AI model architectures beyond just transformer architectures (such as LLMs). These include time series forecasting models, AI models for recommendations systems, and more.

Due to the recent advances in the field of generative AI, the demand for AI compute is expected to grow exponentially in the coming years. Inevitably this will lead to a rise in the demand for compute and energy resources.

More precisely, the rapid growth in AI model performance has been accompanied by an ever increasing number of model weight parameters that require higher amounts of memory to host the models. Since the leading LLMs require several hundred GBs, it is likely that this trend will only accelerate and we will see much larger models becoming available in the future. For this reason, to run the best AI models at scale, it is crucial to have specialized powerful hardware, such as (multiple) high caliber GPUs (such as the Hopper and Blackwell Nvidia GPU series). 

To account for the above phenomena, we have deliberately designed Atoma to provide the right financial incentives to compute providers to participate in the network and be able to monetize their hardware. This incentivizes compute providers to acquire better hardware which will allow nodes to accrue higher value in the long run.

These financial incentives combined with a series of optimizations around AI inference will lead to nearly optimal node GPU utilization, allowing the Atoma Network to scale horizontally with the number of nodes, and also to scale vertically with better available hardware per node. -->

Moreover, the Atoma Network leverages existing blockchains as compute coordination layers. This means that the financial incentives for nodes to participate in the network are managed by Atoma's own on-chain smart contracts. The data and compute resources are entirely decoupled from the blockchains, as the latter inherently have limited compute and storage capabilities. Consequently, the Atoma Network functions as both an AI coprocessor and a heavy compute execution layer that operates on top of existing blockchains.

New applications will be built through the Atoma Network that can leverage the immense potential of generative AI models in order to provide a new intelligence layer for Web3 protocols.

The Atoma Network will be a natural framework to build innovative applications such as protocol based AI generated content, AI market predictions, Web3 AI enhanced wallets specialized for user intent interactions, knowledge bases as public goods, social discussion forums, DAO and Network States governance, etc.

In a world where AI will play the role of our caretaker, tutor, personal assistant, and coworker, it will be crucial that these applications will provide the high trust guarantees around AI inference.

That said, in order for the Atoma Network to become a full intelligence layer powering Web3 protocols, it is required to establish high integrity guarantees around AI inference compute. In practice, this means that one needs high certainties that AI generated outputs have not been tampered with in any possible form; in other words, no malicious actor could have interfered with the correct execution of an AI model.

We have designed a novel consensus protocol, which we refer to as `Sampling Consensus`, exactly for this purpose. Our protocol is based on the assumption that nodes are rational actors in a competitive economic environment. The protocol is optimized to achieve very high security guarantees whilst having lower cost compared to other methods (such as zkML or opML).

For the above reasons, the Atoma Network is well positioned to lead the space of decentralized AI compute. We see a future where the Atoma Network will open the doors for a more bright, transparent, innovative, and democratic future where technology is fully aligned with users and communities.

### Introduction

The past few years have seen the emergence of new generative AI capabilities powered by large language models (LLMs), more powerful hardware allowing for orders of magnitude more computing throughput, and a rise of highly scalable blockchains with lower transaction fees that utilize a combination of novel consensus, cryptography, and modularity techniques.

Especially noticeable is the rapid growth in the field of AI and it is expected that this growth will just accelerate from now on. For example, McKinsey estimates that generative AI could generate a revenue between $2.6 trillion to $4.4 trillion annually across a wide variety of use cases.By comparison, the United Kingdom’s entire GDP in 2021 was $3.1 trillion.
 
This growth will be accompanied by a rise in the demand for compute and energy resources. Through the first half of 2024, we have seen experts in the field highlighting the need for governments and big tech companies to cooperate in order to build the next generation trillion dollar compute clusters, as in Leopold Aschenbrenner most recent work on [Situational Awareness](https://situational-awareness.ai/wp-content/uploads/2024/06/situationalawareness.pdf).

Whether this trillion dollar compute cluster will materialize in the future is yet to be seen. What is clear, however, is there will be rising demand for compute and energy resources over the coming years. AI pipelines require specialized hardware, such as GPUs and TPUs, and while GPUs are widely accessible to the public, (mostly for gaming purposes), the type of GPUs required for AI usually are of a higher caliber, and are less widely available to retail consumers and considerably more expensive. There are only a few GPU manufacturers (such as NVIDIA and AMD), and there are geopolitical tensions that directly affect the supply chains of producing these devices. This results in difficulty making high caliber GPUs for AI accessible to retail or small to medium size data center businesses. 

If this trend continues, we will step into a future in which governments and big tech companies retain most of the world's available compute resources, in which case, these institutions will have a complete monopoly over most of AI development from model training to inference.

Moreover, these institutions will want to preserve their margin of profit and their intellectual property which will inevitably push AI towards becoming a closed source technology (meaning model weights are not available to the general public).

Combine this with the fact that big tech companies have access to virtually all of the real world data being generated, we could see extreme centralization of the most revolutionary technology of our time, generative AI. Furthermore, such high centralization risks might pose a threat to AIs becoming less aligned with users needs and values (AI misalignment). Such misalignment has already been witnessed after the release of the highly [biased](https://www.cnbc.com/2024/03/27/how-to-drive-bias-out-of-ai-without-making-mistakes-of-google-gemini.html) Google's Gemini image generation model. The only way to avoid this dangerous scenario is to build more fair and transparent technologies that can help to mitigate these risks.

This is where decentralized protocols play an important role in mitigating some of the risk factors. For example, we have witnessed financial incentives alignment through blockchains such as Bitcoin and Ethereum with massive network growth effects.

These technologies provide the right framework to align and coordinate computational resources, financial incentives, application development and communities, overall.

Through the Atoma Network, nodes will be able to monetize their compute resources in a permissionless and transparent environment, while being exposed to fair market economic incentives. In order to achieve this, we are building Atoma's node infrastructure with a focus on the ease of software deployment. Atoma node infrastructure will allow nodes to easily execute AI compute requests. Our infrastructure is highly optimized for efficient GPU utilization resources, incorporating the most recent advances in LLM memory management techniques (such as Paged Attention, quantization techniques, etc).

On the other hand, the Atoma Network will provide a platform that creates demand for utilize the available compute on the network, working effectively as an AI market place. We plan to achieve this by building decentralized infrastructure that provides verifiable AI compute where any AI request can be followed with an attestation asserting that the AI generated output has not been tampered with in any form.

Web3 protocols will gain access to AI functionality through the Atoma Network allowing them to integrate a true intelligence layer into Web3 dApps. In order to provide verifiability, we have developed a novel sampling consensus protocol specialized in output state verifiability.

The consensus protocol is elastic in the sense that the end user is able to decide the output integrity guarantees their use case requires and pay a fee that is aligned with their desired level of verifiability. This provides utility to both Web3 applications and naturally centralized Web2 applications.

Additionally there will be demand for Atoma Network functionality beyond smart contracts such as UI chat applications, AI Web3 wallet integrations, online data scraping, next generation browsers and browser extensions, community based knowledge bases, social media platforms, and more. In a decentralized environment, even such applications can’t rely being operated entirely by a single party without further integrity guarantees, due to the potential risk of misbehaving nodes. For this reason, such applications still need to be subject to a decentralized consensus protocol.

Whereas, a DeFi protocol may require almost 100% certainty that a given AI market liquidity prediction was computed correctly, as even a small deviation from the original prediction can have a huge impact on the protocol's future funds management, a user interacting with an online community based chat application is mostly interested in knowing that it paid the right fee to have access to the right model for output generation, and not a less capable model. 

In the last scenario, the risk for the user and the potential reward for malicious behavior for the serving node are much lower than in the previous example. The system can work securely with a Nash equilibrium over the long run with little replication overhead costs. In practical terms this means most requests are being operated at native cost while still maintaining a high level of security guarantees.

Our Sampling Consensus algorithm is well adapted for each of the above use cases, being highly flexible and optimized for different scenarios. We are also actively exploring different verifiability techniques such as Trusted Execution Environments (TEEs) that allow for even higher levels of verifiability while also reducing the cost of verifiable execution close to native cost.

<!-- Web2 style applications will most likely drive the demand for the Atoma Network in the short to medium term. This derives from the fact that Web3 applications are still in their infancy in terms of AI adoption. In numbers, the total market for AI (comprising not only generative AI) is estimated to be worth in the tens of trillions of dollars worldwide. -->

We believe that in order to drive mass adoption of Web3, it is crucial to reach parity with the type of UX and UI that Web2 applications offer. With AI being so pivotal for the overall functionality and features of Web2 applications, it is likely that it will play a similar role in allowing Web3 protocols to reach mass adoption.

In order to empower the next generation of applications in the intersection of Web3 and AI, both data and model privacy is of the utmost importance. Through the use of TEEs, we will not only be able to provide verifiable AI compute, but also provide full data privacy to the end user.

This means that Atoma nodes will not be able to access any sensitive data, such as personal information, while also not being able to access the output generated by the AI model. Only the end user will be able to decrypt the generated output, and can have high assurance that the input to the model was not shared with any third party.

Through this, users can have full control of their data, allowing for AI models to operate on sensitive data. Privacy will unlock the full potential of AI, allowing for the creation of new data centric economic revenue models as well as applications that can operate on the user's behalf without revealing anything about the user.

The Atoma Network can be used by Web2 companies and builders, especially software companies that are built from the ground up to be AI-centric, i.e fast growing AI providers and software companies committed to integrating AI into their products (think of Canvas, etc). The reason for these companies to operate on the Atoma Network comes from the fact that Atoma is positioned to drive down the costs of GPU utilization.

Due to the current GPU chips shortage, together with a waste of GPU utilization, (less GPU cycles being used for AI inference), hardware providers face sunk costs by not using their infrastructure efficiently. Therefore, these are able to offer those fractional GPU cycles at a much lower cost compared to other larger cloud providers.

As an example, a customer that wants to rent an H100 from AWS today needs to commit to a 1-year lease, due to market supply constraints, while being highly unlikely that a customer needs to use a GPU at full capacity during the 1-year period. Moreover, there is a high number of idle GPUs in the market that were used before as mining hardware (e.g., when ETH transitioned from PoW to PoS) which could be repurposed for AI inference.

The development of new AI models will be crucial for the future of the Atoma Network. We believe that open source AI development will democratize and broaden the reach of AI technologies across societies. We are committed to incentivize the open source AI community to develop and deploy new AI models, while being able to monetize their work. This allows experts in the community to earn a continuous stream of revenue through their model utilization.

This is especially important once we factor in possible risks of AI censorship. For example, Italy [banned](https://www.bbc.com/news/technology-65139406) the use of OpenAI's ChatGPT shortly after its release, due to user privacy concerns (even though it later lifted the ban). On the other hand, companies like OpenAI are self-censoring due to societal and political reasons. For example, OpenAI will not build models that can help predict the next presidential election results, etc. These use cases will likely create large predictions/content markets that big technology companies will not want to be involved in. The Atoma Network, on the other hand, can leverage open-source software AI models, at a lower-cost GPU cluster to perform inference for these types of use cases.

The Atoma Network will be powered by its TOMA token. Through robust tokenomics, compute providers, protocol participants, and developers will be rewarded by their contribution to maintaining the network longevity. The TOMA token will be required to pay for AI inference fees, for nodes to deposit collateral to be able to participate on the network, and also for paying developers who build new applications on top of the Atoma Network.

<!-- The token distribution will follow an initial period of inflation in order to subsidize the network's growth. This growth will be used to:

1. Subsidize future node operators to acquire powerful AI hardware,
2. Incentivize model creators to develop new AI models, specialized for applications of interest to the Atoma Network (such as financial market predictions)
3. Incentivize developers to build applications that drive demand for the available compute resources available on the Atoma Network. -->

After the network's inflationary period (which will last a few years), the token emission distribution will follow a slightly deflationary pattern through a combination of tail emissions and burning mechanisms. Together, both strategies will lead to a gradual decrease in the total supply of the TOMA token, contributing for its overall value increase (provided the demand for Atoma Network increases), while also allowing for the allocation of the very small portion of newly minted tokens to the protocol's treasury. These newly minted tokens can then be used for different purposes including bringing new generation hardware to the network or social incentives through UBI.

As a decentralized platform, Atoma Network will put forth combined efforts that will ultimately lead to a new era of use cases within decentralized applications. This is done so by driving financial incentives for computing power to be partially decoupled from centralized cloud providers. Smart contracts will not only deploy new financial products, but also be able to deploy and orchestrate compute resources, powering new decentralized protocols.

The focus of the current text is to explore at a deeper level the Atoma's unique design and architecture. More concretely, we will provide detailed descriptions on the following topics:

* Atoma Network as a decentralized permissionless protocol for AI inference.
* Verifiable AI compute methods, including our novel sampling consensus protocol and TEEs.
* Data and privacy protection for AI inference.
* Governance and reputation mechanisms for Atoma Network participants.
* Node infrastructure and optimizations.
* Applications and use cases for Atoma Network.
* Tokenomics and economic incentives for Atoma Network participants.

### Atoma Network as a decentralized permissionless protocol for AI inference

In this section, we will explore the architecture behind the Atoma Network, including how nodes can register on the network, how they can subscribe to AI models, how requests are processed, and how rewards are accrued by nodes operating on the network. At the heart of the Atoma Network lies the `Atoma on-chain logic`. We refer to the Atoma on-chain logic as a collection of blockchain-agnostic smart contracts that together are responsible for managing the state of the Atoma Network. This Atoma on-chain logic is responsible for managing the state of the Atoma Network. This includes:

- Keeping a registry of which nodes are currently registered and operating on the network. 
- Which AI models are currently subscribed to by nodes.
- Manage node deposited collateral and accrued rewards.
- Select which node(s) to process a given request.
- Settle any unresolved request once every selected node has committed and agreed on the output state. In case of disagreement, a dispute is solicited by the Atoma smart contract.
- Manage the TOMA token emission schedule.
- AI model whitelisting and Atoma Network protocol governance.

**Atoma Network node registry:** In order to ensure the modus operandi of the network, it is crucial to keep a registry of which nodes are providing compute on behalf of the Atoma Network, which fees have these nodes accrued over time by processing requests, and which AI models are being utilized at a given time. In order for the Atoma Network to be fully decentralized, it is crucial that this registry is not maintained by a single entity. Instead, the natural choice is to deploy this logic has a series of smart contracts on a blockchain.

Whenever an entity (being a single user, a company, a DAO, etc) wishes to delegate compute resources to the Atoma Network in order to accrue fees, it should first register on the Atoma Network, through the Atoma on-chain logic. Registration should be handled by a simple transaction call onto the Atoma on-chain smart contract. Upon registration, the smart contract will emit a TicketId for that entity. Owning a `TicketId` will allow the entity to later withdraw the rewards accrued over time. In order to mitigate any possible malicious acts, it is required that upon registration, nodes submit a fixed amount of collateral to the smart contract that is indexed in `TOMA` tokens. Nodes can only withdraw the deposited collateral upon de-registration, in which case the `TicketId` is burned and the node is no longer eligible to operate on the Atoma Network.

The Atoma Network progresses in a series of linear epochs. Epochs can be defined as a fixed time period (e.g. one day, one week, etc). After a node submits a registration transaction onto Atoma, it is only available for the next epoch. Similarly, upon de-registration, the node is still entitled to process any request within the current epoch. Once the current epoch has expired, the node can then withdraw its collateral.

Once a node is registered, the Atoma on-chain logic will be able to assign it to any incoming request and the node should process that request within a certain time period, otherwise a timeout is triggered and the node gets a percentage of its deposited collateral slashed. 

**AI models subscription:** The Atoma Network is specialized for AI inference compute. This means that nodes on the Atoma Network should be able to handle various AI requests over time. For this reason, once the node is registered, it must subscribe to a given AI model.This should be done within the same epoch of registration, otherwise the node will only be able to process requests for the given AI model for the next epoch. Subscribing to a given AI model requires the node to specify its hardware specifications (that is, the GPU cards it hosts, how many of these, etc). Hardware specifications are uniquely identified by an `EchelonId`. In order to facilitate the model subscription, we will have a simple UI with the list of all the EchelonIds, to which node operators can easily use to find the right `EchelonId` for their hardware. Specifying the hardware type is crucial for the network to be able to load balance the requests effectively. Different hardware types will have different pricing, and users can choose how much they want to pay for the processing of a given request.

It is assumed that once the node is subscribed to a given AI model, it will receive incoming requests for AI inference using that model. To run inference efficiently, it is essential that the node has the model weights available locally which avoids needing to download the model weights for each incoming request. Nodes will be able to download their model weights either from a decentralized storage (such as IPFS) or from an external API call (such as Hugging Face).

The node can unsubscribe from a given AI model at any time, immediately halting the node from receiving further requests for that model. However, in case a node is subscribed to a given AI model, and it stops processing requests for that model for a long period of time, it will be penalized with some deposited collateral being slashed.

If a node de-registers from the Atoma Network, it will be automatically unsubscribed from all AI models that it was subscribed to.

**Requests serving:** In order to fully understand the full scope of the Atoma Network, it is important to understand which types of requests can be processed by the network nodes, and how these are processed by the nodes. We will first examine independently two (three?) different types of scenarios in which the Atoma Network can be used at scale:

1. *Externally owned requests:* These consist of any request that is submitted to the Atoma Network by a user's wallet. This request can be generated by the user interacting with a frontend UI, a desktop application, a mobile application, etc. Examples of this correspond to users interacting with applications built by developers such as chat-bots, AI-enhanced Web3 wallets, collama certain shared knowledge chat applications, content generation applications on the web, and online data scraping. In this case, in order to submit a request to the Atoma Network, a transaction to the Atoma on-chain logic contract needs to be signed and submitted to the blockchain.

This request contains only the hash of the input data, and metadata responsible to evaluate the cost to run the AI inference (denominated in TOMA tokens). This includes the number of input tokens and the maximum number of output tokens. The data of the request, that is the prompt, is not submitted directly to the blockchain (to reduce the gas cost of the transaction, as this input data might be very large). Instead, the hash of the data is submitted alongside the transaction. The input data can be shared with the Atoma nodes either through some external API call from the application the user is interacting with, through a decentralized storage (such as IPFS), or through a direct communication between user and nodes (such as WebRTC).

2. *Smart contract requests:* These consist of any requests that are submitted to the Atoma Network by another smart contract, or that are authenticated by a smart contract. In the first case, a smart contract on the blockchain can make calls directly to the Atoma on-chain logic contract, through external smart contract calls. For example, an on-chain funds manager smart contract can call the Atoma on-chain logic contract to request to have access to an AI prediction of the a given token emission rate for the next 24 hours, based on on-chain market data. In this case, the input request is submitted by the smart contract directly to the Atoma on-chain logic contract. This means that the data is fully public and must fit into the blockchain's execution runtime calldata memory limit. It is also likely that the smart contract requesting AI inference will need the output to be submitted back on-chain (think of the prediction market case above). There are situations in which this might not be the case, for example an on-chain NFT AI art generation protocol. 

3. *Smart contract authenticated requests:* The second case, is where a request is not started by a smart contract, but rather by a wallet, and requires a smart contract to authenticate the request. One example is DAO governance decision making through AI in which a transaction might be submitted through a multi-sig, but requires approval directly from the majority of the DAO community. Another interesting case is using account abstract 'wallet's to interact with the Atoma Network, in which one can leverage user-defined permission logic with decision making through AI, i.e an AI is required to parse the web to obtain data information, and if that data satisfies specific well defined logical criteria, blockchain transactions can be submitted. The latter case is especially interesting in the context of the autonomous Smart Agents in Web3 protocols.



Nodes on the Atoma Network listen to blockchain events that are emitted once new request transactions are submitted to the Atoma on-chain logic contract. Once it listens to a new event, it can check if it has been selected to process the request, or not. If it has been selected, it should start processing the request right away. If the event contains all the available input data (in case, the transaction was submitted another smart contract in a `Smart contract authenticated request` case), or it must then fetch the input data from an external API call (in case, the transaction was submitted through a `Externally owned request` case) to the application the user is interacted with, so that it can start processing the request, right away.

**Node collateral and rewards:** As previously mentioned nodes on the Atoma Network are required to deposit a given amount of collateral upon registration. This collateral is subject to slashing in the cases of malicious behavior or when the node becomes unresponsive (it stops processing requests for a long period of time). Note once more that nodes are only able to withdraw that collateral in the epoch following de-registration.

If a node timeouts for a given request, a fixed percentage of its deposited collateral is slashed. Part of the slashed collateral goes to the user that initiated the request, to compensate them for a poor service. Another part of the slashed collateral goes to the Atoma community treasury fund, and the remaining part is burned. This incentivizes nodes to be always responsive, otherwise they lose part of their stake in the network while compensating both the user (with a refund), the network through treasury fund allocation, and also contributing to the TOMA token appreciation through deflation.

If a node acts maliciously, it gets penalized with all of its deposited collateral being slashed. Similarly to the timeout case above, a percentage of the slashed collateral goes to the user, compensating for any possible malicious behavior. There is also a percentage of the slashed collateral that is allocated to the honest nodes involved in processing the request, the Atoma nodes that participated in the dispute event, and the Atoma community treasury fund, with the rest of the remaining collateral being burned.

On the other hand, for each processed request, a node is entitled to part of the fee paid by the user that initiated the request. Request fees are paid in `TOMA` tokens and their price is set by a fair free market mechanism. At the end of each epoch, the Atoma nodes bid the TOMA token fee value, per token, for the next epoch. Therefore, fees are computed in terms of the total number of tokens a given request requires to be processed. So for example, if a prompt text request has 100 input tokens, and its output response contains 200 tokens, then the total fee amount paid to a node to process request will be calculated as:

$(100 + 200) * \mathrm{token\_fee} = 300 * \mathrm{token\_fee}$.

Nodes accrue fees per epoch. Moreover, nodes can only withdraw their accrued fees for a given epoch, once that epoch has ended.

**Node selection:** For every request submitted into the Atoma Network, the Atoma on-chain logic is responsible for selecting one or more nodes to process that request. A node is selected uniformly at random from the set of all the nodes that are subscribed to the AI model that the request requires. This random sampling mechanism helps balancing request load and ensures a fair
distribution of the fees being accrued by nodes. 

**Sampling Consensus:** A request might require multiple nodes to process it in order to provide high assurance guarantees that the generated response was not tampered with. In this case, the request is only settled by the Atoma on-chain logic once all the nodes have submitted a commitment to the output state and these commitments all agree. More precisely, for a given request and a selected node to process it, that node must compute the output generated by running the AI model inference on the request's input. Once the output `O` is generated, the node must then submit a short cryptographic commitment to the output `O`, in the form of:

$[H(H(O || 1), H(O || 2), ..., H(O || n)), \ \ H(O || i)]$

where `||` denotes byte concatenation, `n` the number of selected nodes to process the request and `i` the node's index in the array of all the selected nodes. Here `H` denotes a cryptographically secure hash function.

The fact that node provides both hash `H(O || i)` and the `n`-ary Merkle hash $H(H(O || 1), H(O || 2), ..., H(O || n))$ mitigates risks of a malicious node behavior by lazily wait until one node commits first to the output state, and then simply copy that same commitment, without having to perform the actual computation. Since, each selected node has a unique index in the array `[1, n]`, to be able to compute `H(O || i)`, the node must have access to the actual value of the output `O`. Moreover, if two distinct compute outputs `O` and `O'`, then

$H(H(O || 1), H(O || 2), ..., H(O || n)) == H(H(O' || 1), H(O' || 2), ..., H(O' || n))$,  

if and only if $O == O'$, if very high likelihood. The above observations follow from the key properties of a secure hash function. 

Once all the nodes have submitted all their commitments, the Atoma on-chain logic can compare that all the submitted values

(1) $H(H(O || 1), H(O || 2), ..., H(O || n))$

agree, and it can also verify that the hash of submitted leaves $H(O || 1)$, $H(O || 2)$, ..., $H(O || n)$ equals (1). This ensures that all nodes have submitted to the same output state. If one the previous two conditions are not met, then a dispute mechanism is put forth by the Atoma on-chain logic contract.  If we let $0 < p < 1$ be the maximum percentage of nodes controlled by a single authority, then if for a given request `n` nodes are selected to process it, the probability that a tampered output is settled is $M = p^n$. In other words, such probability decays exponentially with the number of selected nodes. For example, if $p = 0.5$ and $n = 10$, then $M = 0.5^{10} = 0.0009765625$. Moreover, the Atoma on-chain logic contract allows the user and/or the protocol to choose the number of nodes `n` to be selected for a given request, providing elastic verifiability optionality depending on the task at hand.

This method of node sampling is particularly relevant for Web3 applications, that require high assurance guarantees on the output state generated by AI models. These encompass mostly the `Smart contract requests` scenario above. For `Externally Owned` and `Smart contract authenticated` requests, such as UI chat applications, AI Web3 wallet integrations, online data scraping, etc, we can adapt the original Sampling Consensus protocol to reduce replication costs (that is costs of having multiple nodes processing the same request). 

**Cross validation sampling consensus:** In order to address the replication costs within the original `Sampling Consensus` method, we have put forth a slightly different protocol algorithm which we refer to `Cross Validation Sampling Consensus` which allows for considerably lower costs, with a slight impact on verifiability. This method relies on the assumption that nodes are rational actors in a competitive economic environment. In such case, our protocol can be shown to reach Nash equilibrium over the long run. Moreover, we recognize that such protocol was independently discovered by the Hyperbolic Labs team, in their research [paper](https://arxiv.org/html/2405.00295v2).

Instead of the Atoma on-chain logic selecting `n` nodes at random for a given request, the contract simply select a single node at random. Once the node commits to the output state, the contract then select a quorum o `n > 0` nodes with a protocol-defined probability `p`, also referred to as the `repliation rate`, to recompute the output and attest for the first node output commitment integrity. If the Atoma on-chain logic chooses to select `n` nodes, with probability `p`, then the output is settled once all the `n + 1` (the `n` nodes plus the original node) nodes have committed to the output state and agree on it. If there is a disagreement, a dispute mechanism is put forth by the Atoma on-chain logic contract, similarly to the previous case. On the other hand, if the Atoma on-chain logic does not select anymore nodes, with probability `1 - p`, then the output is settled once the original node has committed to the output state, without further verification.

We will show, in the appendix, that in a rational competitive node environment, such protocol leads to a Nash equilibrium game theoretical analysis. Moreover, under the assumptions that the maximum of nodes controlled by a single authority is `50%` of the total available compute resources on the network, that the total collateral at stake per node is $250$ times than the actual native cost of running AI inference, and that the only reward of being malicious on the network is simply not running expensive AI inference, then one can show mathematically that the replication rate is inferior to `0.01`. That is:

*Conclusion:* In such scenario, in order for the network to reach Nash equilibrium, the protocol only needs to select more than a single node for executing a request at most 1 in every 100 submitted requests, on average. 

Therefore, Cross Validation Sampling Consensus is especially relevant for applications in which the nodes have little incentives to collude with the request intent and sender, except for the fact that they might try to cheat the protocol by running smaller and less capable AI models, instead of those they have subscribed to, in order to reduce their cost of running the AI inference. As referred these include both `Externally Owned` and `Smart contract authenticated` requests.

As already observed, Cross Validation Sampling Consensus is not indicated for applications that provide smart contracts with additional AI inference features, as in that case the incentive to collude with the protocol might be orders of magnitude higher than the cost of running native AI inference. Such a scenario pertains to the `Smart contract requests` case.

**Node obfuscastion:** In the Cross Validation Sampling Consensus protocol, the protocol needs to process a two-round inference every $p * 100$ requests. One can optimize the protocol to rely on a single round of inference, through node obfuscation. In this way, the protocol simply does not disclose which nodes have been selected to process a given request, neither the number of nodes selected. In this way, a given node can verify if it was selected to process a given request, but does not learn any information if there is another node selected for the same request. In this way, the same analysis as in the previous section can be applied to the single-round protocol. 

**Non-determinism nature of AI inference:** Verifiable AI inference is only possible within a deterministic context. Within our sampling consensus protocol (including Cross Validation Sampling Consensus), the protocol relies on the assumption that nodes can reach to the same output given the same inputs and the same AI model weights. However, running AI inference is not necessarily non-deterministic, for different reasons. 

First, generative AI inference often times relies on next token sampling, in which the AI model forward pass is on a given input token series to generate a logits tensor of probabilities for the next token. Once we have the logits tensor available, the algorithm goes by sampling the next token among those with highest probability available. Sampling tokens in this way is highly non-deterministic. However, the source of non-determinism can be controlled through the random seed. Once we fix the random seed (and other metadata values such as temperature, top_k, top_, etc), it is shown that given the same logits tensor, token sampling generation becomes deterministic across any number of different machines.

Secondly, other sources of non-determinism are derived from the fact that AI inference relies heavily on floating point arithmetic and floating point arithmetic is not transitive under assummption. This means that if we have three floating point number $x, \ y, \ z$ then 

\[(x + y) + z != x + (y + z)\].

One way to mitigate this issue is to use fixed point arithmetic possibly with quantized weights, instead of floating point arithmetic. However, even quantized model weights are usually combined with floating point arithmetic inputs. Once we quantize these to fixed points, it is shown that one sees a model performance degradation, which for a wide range of applications is unacceptable.

Another possible way to mitigate this issue is to run requests across nodes with the same hardware specifications, such as the same GPU card model. This ensures that floating point arithmetic operations become deterministic across different nodes (provided they are all process the requests across the same GPU cards satisfying the [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) floating point standard, which is the case for Nvidia GPU cards). This observation can be mathematically proven through a thread volume analysis on the number of GPU cores, plus writing deterministic GPU kernels (which is the case for most of the available kernels used in AI inference). Moreover, we have run hundreds of thousands of tests internally on this issue, running different requests across multiple nodes (all with the same GPU cards) with the same random seed, and other input metadata values. Through all our testing we have observed full determinism across all the generated outputs (across all machines).

Moreover, it is possible to adapt the work by M. Srivastava, S. Arora, D. Boneh on [Optimistic Verifiable Training by Controlling Hardware Nondeterminism](https://arxiv.org/abs/2403.09603)
to our protocol, within the case of Cross Validation Sampling Consensus. In this case, the initial node runs AI model inference committing to a lot state of all the floating point approximations it made within given critical thresholds. Once other nodes are selected, they must run the same AI inference, with same input parameters, following the first node committed log state, if that does not lead to a disagreement outside the critical threshold. Through this mechanism, the Cross Validation Sampling Consensus protocol can be made fully verifiable across nodes with different hardware specifications.

**Disputes:** In the case a request is not settled within the protocol, that is, sampled nodes do not agree on the output state, a dispute is put forth by the Atoma on-chain logic contract. Disputes can be resolved in different ways. First, the Atoma on-chain logic is responsible for sampling a given number of nodes to resolve the dispute. In that case, the nodes must run the exact same AI inference on the given request. These nodes are required to submit an actual proof of the computation, either through a fraud proof in which a bisection algorithm is used to find the initial step of disagreement between the initial selected nodes and the segment of computation can be run within a zk proving system. Another possibility, is to run the whole AI inference within a single or multiple trusted execution environments (TEE) providing TEE attestations. Once such proof/attestation is generated, it can be verified by the Atoma on-chain logic contract and the contract can resolve the dispute by rewarding the malicious nodes and rewarding both the honest nodes, the selected disputed nodes, the Atoma community treasury fund and the user that initiated the request. 

Notice that, from the fact that the deposited collateral is orders of magnitude higher than the actual native cost of running AI inference, even though that the dispute resolution might be orders of magnitude more expensive than the native cost (if the dispute is to be resolved through a fraud proof), it is still economically feasible, as the collateral is then distributed among all parties involved. Moreover, in case of a dispute, the user might right away submit a new request, with the same contents, with the guarantee that it will be rewarded monetarily for the initial failed request.

We will write a detailed analysis on the security guarantees of TEEs on the Atoma Network, in the future.

**AI model whitelisting and governance:** At launch, the Atoma Network will support the major open source AI models, such as Llama3-8B, LLama3-70B, Qwen2, Phi3-mini, Stable Diffusion, etc. That said, it is crucial that the Atoma Network protocol allows for new, more capable, AI models to be added to the network, over time. Being committed to the standards of open source AI, we plan to build incentive systems to reward developers for creating and deploying new AI models into the network. However, this question comes with a few questions, namely:

1. *How to ensure tha the deployed AI model does lead to potential malicious behavior?* This is a very hard task to solve by formal methods. General generative AI models, such as Llama, GPT, Claude, Qwen2 etc have been training on enormous corpus of text data. This means it is very difficult and expensive to access the quality of that data. Moreover, even if this is attainable, these models being general purpose, can be prompted engineered  to produce some undesired answer. For these reasons, and from the fact we believe the protocol should have the least amount of trust assumptions possible, we prefer to leave the choice of the deployment of new AI models into the network to the protocol developers, communities and users. 

In order to register a new AI model onto the Atoma Network, any externally owned or a contract account can submit a transaction into the Atoma on-chain logic contract, whose goal is to register a new AI model architecture and weights. In order to do so, it must specify a unique AI model id, and a model weight hash identifier (generated as a Merkle tree root hash of all the model weights). Once this registration is submitted, Atoma nodes can subscribe to that AI model. 

If a node subscribes to a given AI model, it must download its weights either through an external API call service (such as Hugging Face) or from a decentralized storage (such as IPFS). Through Atoma's own UI interface, it is possible to have access to the list of all the models being orchestrated onto the network, including additional metadata that allows nodes to fetch the model weights. Once the weights are downloaded, the node must ensure that the Merkle tree root hash of the model weights is the same as the one specified in the registration transaction. If this is not the case, the node will submit a failed subscription request into the Atoma on-chain logic contract. If enough such failed subscription requests are submitted, the AI model id will be removed from the internal registry of the Atoma on-chain logic contract.

Once there are enough nodes subscribed to that model, whose number depends on the total available number of nodes on the Atoma Network, the AI model becomes available to be used to be called by  protocols and users of the Atoma Network.

Importantly, the Atoma on-chain logic contract can be used to accrue fees for AI model creators. The AI model creators should 'own' the external owned or contract account that initiated the model registration transaction. That wallet can be entitled to accrue fees for the deployed AI model, for every request that is executed through that AI model, together with it being entitled to have a portion of the token emission rate, as well. In order to avoid possible model weights leaks and theft, we will put forth a mechanism to ensure that the AI model creator can be properly identified as such. It is possible to develop Sybil resistance algorithms for this purpose, following similar ideas like those in the [zkLogin](https://arxiv.org/abs/2401.11735) protocol. 

In the future, we plan to incorporate a whitelisting mechanism into Atoma's on-chain logic, that allows model contributors, protocols and users (namely, `TOMA` token holders) to be able to vote for a general deployment of a new given AI model for their protocol specifically. For example, if either a `externally owned` or `smart contract authenticated` request based protocol builds a new application on top of the Atoma Network, the protocol might be interested in deploying a new AI model architecture solely for its own purposes. Similar to the above case, the protocol submits a transaction for model registration, but it provides a permissioned access to the model on the Atoma Network. From this point on, only the protocol can use the model on the network. This is especially important as mechanism to protect applications/protocols IP on the Atoma Network. This method is particularly interesting when adopted with full privacy of model weights, which we will explore below.

That said, the opposite might be relevant for protocols built on top of the Atoma Network. Namely, a protocol might have developed its own set of (system) prompts through which request inputs are passed to the prompt, in the form of 'prompt keys'. After making its due diligence to ensure that if a given AI model provides reasonable answers when run on the given prompts, with different input values, it is found that the AI model might be of danger to the integrity of the protocol, it can black-list the AI model from being publicly used the protocol. 

### Verifiable AI

In this section we will explore the different methods for verifiable AI inference, and compare them with respect to our Sampling Consensus protocol, in terms of performance, cost and security guarantees. We are mostly interested in a formal comparison between zkML and OpML techniques, for verifiable AI.

#### zkML

The field of zero knowledge Machine Learning (zkML) has emerged after Zero Knowledge proofs have gained a prominent role in the cryptocurrency space, especially for scaling blockchains. To put it simply, zkML is a set of techniques based on Zero Knowledge cryptography applied to providing cryptographic proofs that attest any Machine Learning model inference computation. Over the past few years, some promising research has been published on the topic, such as [TensorPlonk](https://medium.com/@danieldkang/tensorplonk-a-gpu-for-zkml-delivering-1-000x-speedups-d1ab0ad27e1c), [trusted DNN](https://arxiv.org/abs/2210.08674), [zkCNN](https://eprint.iacr.org/2021/673), Modulus [Remainder](https://github.com/Modulus-Labs/Papers/blob/master/remainder-paper.pdf) protocol based on [GKR](https://github.com/Modulus-Labs/Papers/blob/master/remainder-paper.pdf), [SpaGKR](https://github.com/Modulus-Labs/Papers/blob/master/remainder-paper.pdf), [zkLLM](https://arxiv.org/pdf/2404.16109) to name a few.

Especially relevant to our discussion of verifiable AI inference for LLMs, is the work in zkLLM, which, at the time of writing, is considered the state of the art zkML work applied to LLMs. Such work leveraged GPU acceleration, leveraging the CUDA library [ec-gpu](https://github.com/filecoin-project/ec-gpu) for BLS12-381 curve operations on GPUs.
Even though achieving an astonishing 15 minute proof generation time for a single 13 billion parameter LLM inference forward pass, it is important to notice that such forward pass allows to generate the next token, in the sequence. If we consider an output sequence of 100 tokens in total (which is considered a short answer, for practical purposes), then aggregating all the 100 forward passes zk proofs together would need at least 100 times more time. That is, one would need to wait 1500 minutes (that is, 25 hours) in total to generate a zk proof for entire LLM inference to generate 100 tokens. That said, in practice we would expect such zk proof generation time to be considerably higher, as attention layers in LLMs have quadratically growing Key and Value (KV) caches, with the number of both prompt and generated tokens. Given that LLM inference on GPUs is bounded by GPU memory and not compute, we expect that quadratic growing KV caches will lead to higher proof generation times, in practice.

Moreover, the zkLLM proving times were achieved using a setup of 124.5GB of memory, 12 CPU cores of an AMD EPYC 7413 (2.65 GHz with 128M cache L3), and an NVIDIA A100SMX4 GPU with 40GB of memory, which is a setup orders of magnitude more powerful than the actual hardware requirements to run native LLM inference for models up to 13 billion parameters.

For larger models, such as 70 billion parameter Llama3, we are still yet to see any research work to provide zk proofs for such models inference. With the advent of larger LLMs, such as the 400 billion model parameter Llama3, zkML might not be practical for such large LLMs. 

To add to that, LLM inference does not pertain solely to proving multiple forward passes. Often times, LLM inference uses techniques such as next token sampling, out of a list of highly likely logits, to actually generate the next token. This is especially relevant, if one needs to ensure some creativity of the LLM responses. If the full LLM inference is to be proved, it is necessary to provide zk proofs for the sampling process, as well. Each of these steps are to considered if one aspires to have a fully zk proving system for LLM inference, adding to its complexity and cost. 

It is important to notice that zero knowledge proofs provide a very high level of security guarantees for LLM inference. Combined with the fact that zkML will continue to be a few times more expensive than native LLM inference, different verifiable requirements situations will not benefit from a more elastic verifiability strategy, contrary to techniques offering Nash equilibrium in the long run with considerably lower costs, such as our Cross Validation Sampling Consensus protocol.

Even though considerable technical challenges remain to make zero knowledge proofs practical for LLM inference, we consider these results to be highly promising, especially if we factor in a combination of various likely scenarios for improving zk proving generation for LLM inference. We are especially interested in the use of zk-STARK based proving systems requiring smaller bit-field sizes, such as Plonky3 over [Mersennes31](https://eprint.iacr.org/2024/278) and [Binius](https://eprint.iacr.org/2023/1784), and the future production of zk-specific acceleration hardware. It is also relevant to notice that zkML provides the most secure method for verifiable AI, while also having the advantage of allowing for model weight to be kept private by nodes (however, zkML does not provide a privacy preserving model for input and output user data).

Given that zkML is still in its infancy and it has already seen incredible improvements in the past few years, we are looking very closely to the developments of the field, providing a potential candidate to be integrated into the Atoma Network, in the future, for verifiable LLM inference.

#### opML

Contrary to the heavy cryptographic reliance of zkML, optimistic Machine Learning (opML) adopts a fundamentally different strategy based on dispute resolution mechanisms. The optimistic approach presupposes that participants will act honestly, given the economic disincentives for fraudulent behavior. In the rare event of disputes, opML provides mechanisms for challenge and resolving fraudulent claims, ideally without necessitating heavy computational verification for every transaction. Nevertheless, the reliance on economic incentives and dispute resolution may introduce vulnerabilities for network security. 

In particular, for each AI inference request, a challenge period is required in order for verifier nodes to potentially challenge the validity of original AI inference output. This challenge period is a fixed period of time, that might correspond to a few minutes to a few hours. This characteristic of the protocol has the disadvantage of introducing a delay, or increase in latency, for settling requests on the network. Moreover, verifier nodes are typically paid if they put forth a dispute and the resolution favor them. Therefore, for expensive computations such as AI inference, it becomes less economically viable to incentivize verifiers to continuously attest the validity of AI inference outputs being generated by the network. The latter might invariable lead to security vulnerabilities for the network.

#### Atoma's Sampling Consensus

The Atoma's Sampling Consensus (both in its simpler form and its Cross validation variant) provide verifiable AI inference, with the advantage of allowing for elastic verifiability. At a single node level, the Atoma's Sampling Consensus requires only the node to run native AI inference, without any additional overhead compared to zkML. Contrary to opML, our Sampling Consensus protocol does not require waiting for any challenge periods in order to reach settlement, instead settlement is reached once the slowest node requested to process a request commits to the output state (in the happy path of no disputes).

Even though our simple form of Sampling Consensus can lead to high verifiability guarantees, at the expense of higher replication costs,
our Cross Validation Sampling Consensus protocol requires minimal replication, allowing for close to native cost of running verifiable AI inference, without challenge periods, and better security guarantees than opML. 

#### Comparison between zkML and Sampling Consensus

In this section, we will compare our Sampling Consensus protocol with both zkML and opML methods.

Compared to zkML, our Sampling Consensus protocol requires considerably less computational resources, overall, with a much lower cost and overhead time. Moreover, Sampling Consensus allows node on the network to run native AI inference, with no additional hardware requirements, other than those for native AI inference. This implies that nodes on the network can implement the latest optimizations around LLM inference, etc, without having to recur to an extensive research and development efforts to integrate the latest state of the art in AI inference. 

Another advantage is that our Sampling Consensus protocol allows for elastic verifiability. That said, if an application requires very high levels of verifiability, the user and/or protocol might require multiple nodes to be sampled in order to reach consensus on the output state. Similarly, applications requiring applications requiring low levels of verifiability might prefer to select a very little number of nodes to be sampled, in order to reduce the cost of running the AI inference. Moreover, using Cross Validation Sampling Consensus, the majority of decentralized AI applications will be able to process requests with long term verifiability guarantees, with minimal replication costs. 

A drawback of our Sampling Consensus protocol faces compared to zkML is the fact that Sampling Consensus does not provide full privacy for model weights (indeed, a wide number of nodes on the network need to be able to access the model weights in order to run LLM inference on a given model). This can be widely mitigated by using TEEs, which then can enable full privacy for model weights and user data privacy. We believe, the use of TEEs combined with our Sampling Consensus protocol will lead to both full privacy and very high verifiability (even with Cross Validation Sampling Consensus). We will explore these synergies in a future paper.

#### Comparison between opML and Sampling Consensus

Contrary to zkML, OpML claims low computational overhead, with the caveat that opML may incur higher overhead during disputes. The zkML’s approach results in high computational overhead due to cryptographic processes. 

Sampling consensus also requires low computational overhead. Moreover, even when multiple nodes are required to process a request (both for simple and Cross Validation Sampling Consensus) the overhead is very low. This is because multi-node runs only require the Atoma on-chain logic to compare a few hashes. Only when the hashes do not match, we need to resolve a dispute, which may incur high overhead during arbitration (even though, in an environment with rational agents, arbitration never occurs).

Moreover, following the ideas in the [work](https://arxiv.org/pdf/2308.02880) for Rollup validators, we can derive a Nash equilibrium for opML, however, our approach reflects a lower probability of undetected fraud lower than that of opML, assuming that a malicious actor can control at most 10% of the network total compute power.

### Atoma's node optimizations

### Application layer on top of the Atoma Network

### Output management

### Tokenomics

### Governance
