# happy
A self practicing Rust synchronization service

## Actor 设计

* 游戏逻辑Actor, 负责游戏逻辑处理,可以有多个游戏逻辑actor相互交互
* 网络通讯Actor, 负责收发客户端的接受和发送消息
* 帧同步Actor, 负责定时从游戏Actor中获取游戏状态数据计算帧同步数据,并发送给网络通信Actor

## 容错和一致性保证

* 为了保证容错和一致性，可以使用一些算法和协议，例如基于向量时钟的一致性协议或分布式快照算法。

## 安全和防作弊

* 使用加密算法


