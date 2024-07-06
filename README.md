# up-liquidator-rs
This is an [paradigmxyz/artemis](https://github.com/paradigmxyz/artemis) bot that liquidates UP positions on chain

# Usage
#### Download

```shell
git clone git@github.com:xiaoyu1998/up-liquidator-rs.git --recursive
```
#### copy address from up-contracts
```
cp deployed_addresses.json /home/xiaoyu/work/up-liquidator-rs/deployments
cp underlyAsset_addresses.json  /home/xiaoyu/work/up-liquidator-rs/deployments
cp deployed_addresses.json /home/xiaoyu/work/up-liquidator-rs/crates/liquidator-contract/deployments
cp underlyAsset_addresses.json /home/xiaoyu/work/up-liquidator-rs/crates/liquidator-contract/deployments

```
#### deploy liquidator
```shell
cd crates/liquidator-contract
npx hardhat ignition deploy ignition/modules/liquidator.ts --network localnet
```
#### run liquidator
```
cargo run -- --rpc http://192.168.2.106:8545 --private-key "private-key " --deployment localnet --bid-percentage 10 --liquidator-address 0x5aa3B6d49e2AAC9AD7c687C79A899AA6Db2e3cbf

```
