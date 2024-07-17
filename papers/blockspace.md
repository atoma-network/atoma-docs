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

