# Cosmos 网络与 Cosmos 生态系统

> Cosmos 将自己定位为“区块链互联网”。

## 一、简介

Cosmos 是`Layer 0`的区块链。简而言之，Cosmos 是一个基础设施，允许在区块链网络中连接多个`Layer 1`的区块链，例如[Osmosis](https://coinmarketcap.com/currencies/osmosis/)、[dYdX](https://coinmarketcap.com/currencies/dydx/)以及[Evmos](https://coinmarketcap.com/currencies/evmos/)。这些区块链通过区块链间通信（IBC）协议进行通信。

因此，Cosmos 不是线性扩展（如Solana等单片链）或模块化扩展（如Ethereum尝试分离执行层和结算层），而是通过允许不同的区块链以信任最小化的方式进行通信来扩展。每个区块链（或`HUB`）可以与区块链群岛中的不同`HUB`连接。此外，每个区块链都可以拥有自己的特定于应用程序的区块链或应用程序链（在 Cosmos 生态系统中称为`Zomes`）来扩展其中心。

![image-20240415131703263](/Users/booker/Library/Application Support/typora-user-images/image-20240415131703263.png)

与单一的第一层链相比，Cosmos 设想了一个以去中心化的方式交换、存储和保护价值的平台。因此，Cosmos 选择使用模块化软件堆栈（Cosmos SDK）和互连区块链网络进行构建。这可以让 Cosmos 生态系统中的中心和区域能够通过自定义执行环境启动新的区块链，同时利用 IBC 进行跨链通信。

这种模块化和自主增长的愿景导致 Cosmos 上的区域数量快速增加。

[Map of zones](https://mapofzones.com/home?columnKey=ibcVolume&period=24h)

## 二、Cosmos技术堆栈

### Cosmos SDK

开源 Cosmos SDK 有助于从头开始构建特定于应用程序的链，使其能够与其他区块链进行互操作。简而言之，就是需要、构建自定义区块链。下一步则是测试并收集和迭代反馈，然后再在主网上启动。最后，区块链可以连接到其他 Cosmos 区块链，通过 IBC 将区块链连接到 Cosmos 网络，有助于提高每个区块链的适应性和链上币的流动性。

### Tendermint

Tendermint 是一种BFT共识机制，这意味着只要超过三分之一的验证者是诚实的，区块链就永远不会创建分叉。Tendermint BFT引擎通过称为应用程序区块链接口（`ABCI`）的套接字协议连接到应用程序。该协议可以包装在任何编程语言中。

## 三、Cosmos Hub 和 ATOM

### ATOM

Cosmos 通常与ATOM代币相关联。 ATOM 是 Cosmos Hub 的原生代币，而 Cosmos Hub 只是 Cosmos 生态系统中的一个区域。

### Current ATOM Tokenomics

由于COSMOS的不同，其他HUB可以拥有自己的 Validators 和 Native tokens，因此 IBC 中不一定需要 Cosmos Hub 和 ATOM 代币，并且代币不会产生价值。这会导致协议费用微乎其微。

此问题已通过 Cosmos 2.0 更新得到解决。

## 四、Cosmos 2.0

Cosmos 于 2022 年 9 月发布了 [Cosmos 2.0 White Paper](https://gateway.pinata.cloud/ipfs/QmWXkzM74FCiERdZ1WrU33cqdStUK9dz1A8oEvYcnBAHeo)，里面提到了Cosmos未来三年变化的路线图，主要是为了解决Cosmos生态系统的缺点。

四项关键创新：

- 流动质押
- 链间安全
- 链间调度器
- 链间分配器

### 流动质押

流动质押是包装质押代币以将其用作区块链生态系统中的抵押品并提高资本效率的过程。

![image-20240415133358655](/Users/booker/Library/Application Support/typora-user-images/image-20240415133358655.png)

### 链间安全

链间安全性将 Cosmos Hub 置于生态系统安全模型的中心。

虽然其他区域保留拥有自己的Validators的选项，但它们也能够租用 Cosmos Hub 验证器。这使得较小的应用程序链能够防御验证者攻击，同时保持去中心化。

Cosmos Hub 通过这些应用链上的代币和费用来收取费用，其中 25% 将支付给 ATOM 质押者。

### 链间调度

主要是为了解决MEV问题应用链将能够以代币化`NFT`的形式交易区块空间，从而允许用户以信任最小化的方式执行区块交易。

### 链间分配

Cosmos Hub 将协调 ATOM 持有者和 DAO 之间的激励措施，并参与激励生态系统新公共产品的融资。目标是使 ATOM 成为 IBC 协议中最理想、最广泛部署的储备资产。

## 五、IBC

是区块链之间的连接协议，利用Tendermint的即时最终确定性，允许其他不同的链（共识层必须具有`fast finality`）实现价值传递和数据传递，当前没有其他区块链可以支持这种级别的互操作性。

### IBC工作原理

1. 链A上的帐户想向链B发送10个代币
2. Tracking：链B接收链A的`headers`，反之亦然。这允许每个链`跟踪`另一个链的验证器集。从本质上讲，每个链都运行另一个链的`轻客户端`。
3. Bonding：启动IBC传输后，ATOM被锁定（绑定）在链A上。
4. Proof Relay： 然后，10ATOM的证明从A链传递到B链。
5. Validation： 在链B上对照链A的`headers`验证该`proof`，如果有效，则在链B上创建10个`ATOM凭证`。
6. 请注意，在B链上产生的ATOM并**不是真正的ATOM**，因为ATOM只存在于A链上。它们是A链上ATOM在B上的表现，同时也证明了这些ATOM是冻结在A链上的。当ATOM回到原来的链时，也可以使用类似的机制来解锁ATOM。

### Hub 和 Zone

`Zone`是常规的异构区块链，而`Hub`则是专门设计用于将`Zone`连接在一起的区块链。`Zone`创建与`Hub`的`IBC`连接时，它可以自动访问与其连接的所有其他`Zone`（即向其发送和从其接收）。结果，每个`Zone`仅需要与有限的Hub建立有限数量的连接。`Hub`还可以防止`Zone`之间的双重支付。这意味着，当`Zone`从`Hub`接收到`token`时，它仅需要信任此`token`和`Hub`的原始`zone`。

### 连接非Tendermint链

两种情况可以区分：

- `快速最终链`： 使用任何`快速最终共识算法`的区块链都可以通过调整`IBC`与`Cosmos`连接。例如，如果以太坊要切换到Casper FFG（ Friendly Finality Gadget ），则可以通过使`IBC`与`Casper`配合使用，在以太坊和Cosmos生态系统之间建立直接连接。
- `概率最终链`： 对于没有`快速最终`的区块链会有些麻烦，比如`工作量证明链`。对于这些链，Cosmos会使用一种特殊的`代理链`，称为`Peg-Zone`。 一个`Peg-Zone`是一个区块链，它**跟踪**另一个区块链的状态。`Peg-Zone`本身具有`快速终结性`，因此与IBC兼容。它的作用是为它所连接的区块链`确定最终结果`。

![image-20240415135732111](/Users/booker/Library/Application Support/typora-user-images/image-20240415135732111.png)