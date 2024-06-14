use super::types::Config;
use crate::collectors::time_collector::NewTick;
use anyhow::{anyhow, Result};
//use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_up::{
    reader::Reader,
    data_store::DataStore,
    // event_emitter::EventEmitter,
    // exchange_router::ExchangeRouter,
    // liquidation_handler::LiquidationHandler,
    // ierc20::IERC20,
    event_emitter::{
        EventEmitter, 
        DepositFilter, 
        BorrowFilter, 
        RepayFilter, 
        RedeemFilter, 
        SwapFilter, 
        LiquidationFilter, 
        ClosePositionFilter, 
        CloseFilter
    },
};
use clap::{Parser, ValueEnum};
use ethers::{
    contract::builders::ContractCall,
    providers::Middleware,
    types::{transaction::eip2718::TypedTransaction, Address, ValueOrArray, H160, I256, U256, U64},
};
// use ethers_contract::Multicall;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
use std::fs::File;
use std::io::Write;
use std::iter::zip;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info};

use super::types::{Action, Event};

#[derive(Debug)]
struct DeploymentConfig {
    data_store: Address,
    reader: Address,
    event_emitter: Address,
    exchange_router: Address,
    liquidation_handler: Address,
    weth: Address,
    multicall: Address,
    creation_block: u64,
}

#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum Deployment {
    TESTNET
}

// admin stuff
pub const LOG_BLOCK_RANGE: u64 = 1024;
pub const MULTICALL_CHUNK_SIZE: usize = 100;
pub const RAY_DECIMALS: u64 = 27;
pub const WETH_DECIMALS: u64 = 18;
pub const STATE_CACHE_FILE: &str = "borrowers.json";

fn get_deployment_config(deployment: Deployment) -> DeploymentConfig {
    match deployment {
        Deployment::TESTNET => DeploymentConfig {
            data_store: Address::from_str("0x4B962Bc43951528A2bA7713E836B9EEB0528B791").unwrap(),
            reader: Address::from_str("0x1D380146EB9216751fE453854ed23544Af04baE2").unwrap(),
            event_emitter: Address::from_str("0x7C9d309C2D87d7Af8b1D5060D78A9e000e0aD4b4").unwrap(),
            exchange_router: Address::from_str("0xA238Dd80C259a72e81d7e4664a9801593F98d1c5").unwrap(),
            liquidation_handler: Address::from_str("0x95401dc811bb5740090279Ba06cfA8fcF6113778").unwrap(),
            weth: Address::from_str("0x2Cc0Fc26eD4563A5ce5e8bdcfe1A2878676Ae156").unwrap(),
            multicall: Address::from_str("0x2Cc0Fc26eD4563A5ce5e8bdcfe1A2878676Ae156").unwrap(),
            creation_block: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
    pools: HashMap<Address, Pool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pool: Address,
    collateral: U256,
    debt_scaled: U256,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Borrower {
    address: Address,
    positions: HashMap<Address, Position>,
    //health: u64
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pool {
    underlying_asset: Address,
    symbol: String,
    price: U256,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct UpStrategy<M> {
    client: Arc<M>,
    bid_percentage: u64,
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
    pools: HashMap<Address, Pool>,
    chain_id: u64,
    config: DeploymentConfig,
    //liquidator: Address,
    //liquidation_threshold:u64
}

impl<M: Middleware + 'static> UpStrategy<M> {
    pub fn new(
        client: Arc<M>,
        config: Config,
        deployment: Deployment,
        liquidator_address: String,
    ) -> Self {
        Self {
            client,
            bid_percentage: config.bid_percentage,
            last_block_number: 0,
            borrowers: HashMap::new(),
            pools: HashMap::new(),
            chain_id: config.chain_id,
            config: get_deployment_config(deployment),
            //liquidator: Address::from_str(&liquidator_address).expect("invalid liquidator address"),
        }
    }
}

// struct LiquidationProfit {
//     borrower: Address,
//     profit: I256,
// }

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for UpStrategy<M> {
    // In order to sync this strategy, we need to get the current bid for all Sudo pools.
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        self.update_pools().await?;
        // self.approve_tokens().await?;
        self.load_cache()?;
        self.update_state().await?;

        info!("done syncing state");
        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::NewTick(block) => self
                .process_new_tick_event(block)
                .await
                .map_or(vec![], |a| vec![a]),
        }
    }
}

impl<M: Middleware + 'static> UpStrategy<M> {

    /// Process new block events, updating the internal state.
    async fn process_new_tick_event(&mut self, event: NewTick) -> Option<Action> {
        info!("received new tick: {:?}", event);

        self.update_pools()
            .await
            .map_err(|e| error!("Update Pools error: {}", e))
            .ok()?;

        self.update_state()
            .await
            .map_err(|e| error!("Update State error: {}", e))
            .ok()?;

        // info!("Total borrower count: {}", self.borrowers.len());
        // let op = self
        //     .get_best_liquidation_opportunity()
        //     .await
        //     .map_err(|e| error!("Error finding liq ops: {}", e))
        //     .ok()??;

        // info!("Best op - profit: {}", op.profit);

        // if op.profit < I256::from(0) {
        //     info!("No profitable ops, passing");
        //     return None;
        // }

        // return Some(Action::SubmitTx(SubmitTxToMempool {
        //     tx: self
        //         .build_liquidation_tx(&op)
        //         .await
        //         .map_err(|e| error!("Error building liquidation: {}", e))
        //         .ok()?,
        //     gas_bid_info: Some(GasBidInfo {
        //         bid_percentage: self.bid_percentage,
        //         total_profit: U256::from_dec_str(&op.profit.to_string())
        //             .map_err(|e| error!("Failed to bid: {}", e))
        //             .ok()?,
        //     }),
        // }));
        return None;
    }

    // // for all known borrowers, return a sorted set of those with health factor < 1
    // async fn get_underwater_borrowers(&mut self) -> Result<Vec<(Address, U256, U256, U256)>> {
    //     let reader = Reader::<M>::new(self.config.reader, self.client.clone());

    //     let mut underwater_borrowers = Vec::new();

    //     // call pool.getUserAccountData(user) for each borrower
    //     let mut multicall = Multicall::new(self.client.clone(), self.config.multicall).await?;
    //     let borrowers: Vec<&Borrower> = self
    //         .borrowers
    //         .values()
    //         .filter(|b| b.debt.len() > 0)
    //         .collect();

    //     for chunk in borrowers.chunks(MULTICALL_CHUNK_SIZE) {
    //         multicall.clear_calls();

    //         for borrower in chunk {
    //             multicall.add_call(reader.get_liquidation_health_factor(borrower.address), false);
    //         }

    //         let result: Vec<(U256, U256, bool, U256, U256)> = multicall.call_array().await?;
    //         for (borrower, (health_factor, is_health_factor_higher_than_liquidation_threshold, user_total_collateral_usd, user_total_debt_usd,_) in zip(chunk, result) {
    //             if (!is_health_factor_higher_than_liquidation_threshold) {
    //                 info!(
    //                     "Found underwater borrower {:?} -  healthFactor: {} - user_total_collateral_usd: {} - user_total_debt_usd: {}",
    //                     borrower, health_factor, user_total_collateral_usd, user_total_debt_usd
    //                 );
    //                 underwater_borrowers.push((borrower.address, health_factor, user_total_collateral_usd, user_total_debt_usd));
    //             }
    //         }
    //     }

    //     // sort borrowers by health factor
    //     underwater_borrowers.sort_by(|a, b| a.1.cmp(&b.1));
    //     Ok(underwater_borrowers)
    // }

    // load borrower state cache from file if exists
    fn load_cache(&mut self) -> Result<()> {
        match File::open(STATE_CACHE_FILE) {
            Ok(file) => {
                let cache: StateCache = serde_json::from_reader(file)?;
                info!("read state cache from file");
                self.last_block_number = cache.last_block_number;
                self.borrowers = cache.borrowers;
                self.pools = cache.pools;
            }
            Err(_) => {
                info!("no state cache file found, creating new one");
                self.last_block_number = self.config.creation_block;
            }
        };

        Ok(())
    }


    // update known borrower state from last block to latest block
    async fn update_state(&mut self) -> Result<()> {
        let latest_block = self.client.get_block_number().await?;
        let mut start_block = self.last_block_number;
        if start_block > latest_block.as_u64() {
            start_block = latest_block.as_u64();
        }
        info!(
            "Updating state from block {} to {}",
            start_block, latest_block
        );

    
        self.get_deposit_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("deposit {:?} {} {} {}", log.depositer, self.pools.get(&log.pool).unwrap().symbol, log.collateral, log.debt_scaled);   
                let user = log.depositer;      
                if self.borrowers.contains_key(&user) {
                    let borrower = self.borrowers.get_mut(&user).unwrap();
                    borrower.positions.insert(
                        log.pool, 
                        Position {
                            pool:log.pool,
                            collateral:log.collateral,
                            debt_scaled:log.debt_scaled,
                        }
                    );
                } else {
                    self.borrowers.insert(
                        user,
                        Borrower {
                            address: user,
                            positions: HashMap::from([(
                                log.pool, 
                                Position {
                                    pool:log.pool,
                                    collateral:log.collateral,
                                    debt_scaled:log.debt_scaled,                               
                                }
                            )])
                        },
                    );
                }
                return;
            });

        self.get_borrow_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("borrow {:?} {} {} {}", log.borrower, self.pools.get(&log.pool).unwrap().symbol, log.collateral, log.debt_scaled);   
                let user = log.borrower;
                if self.borrowers.contains_key(&user) {
                    let borrower = self.borrowers.get_mut(&user).unwrap();
                    borrower.positions.insert(
                        log.pool, 
                        Position {
                            pool:log.pool,
                            collateral:log.collateral,
                            debt_scaled:log.debt_scaled,
                        }
                    );
                } else {
                    self.borrowers.insert(
                        user,
                        Borrower {
                            address: user,
                            positions: HashMap::from([(
                                log.pool, 
                                Position {
                                    pool:log.pool,
                                    collateral:log.collateral,
                                    debt_scaled:log.debt_scaled
                                }
                            )])
                        },
                    );
                }
                return;
            });

        self.get_repay_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("repay {:?} {} {} {}", log.repayer, self.pools.get(&log.pool).unwrap().symbol, log.collateral, log.debt_scaled);   
                let user = log.repayer;
                let borrower = self.borrowers.get_mut(&user).unwrap();
                borrower.positions.insert(
                    log.pool, 
                    Position {
                        pool:log.pool,
                        collateral:log.collateral,
                        debt_scaled:log.debt_scaled
                    }
                ); 
                if log.collateral == U256::from(0) && log.debt_scaled == U256::from(0)  {
                    self.borrowers.remove(&user);
                }          
                return;
            });

        self.get_redeem_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("redeem {:?} {} {} {}", log.redeemer, self.pools.get(&log.pool).unwrap().symbol, log.collateral, log.debt_scaled); 
                let user = log.redeemer;
                let borrower = self.borrowers.get_mut(&user).unwrap();
                borrower.positions.insert(
                    log.pool,
                    Position {
                        pool:log.pool,
                        collateral:log.collateral,
                        debt_scaled:log.debt_scaled,
                    }
                );
                if log.collateral == U256::from(0)  && log.debt_scaled == U256::from(0)  {
                    self.borrowers.remove(&user);
                }          
                return;
            });

        self.get_swap_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("swapIn {:?} {} {} {}", log.account, self.pools.get(&log.pool_in).unwrap().symbol, log.collateral_in, log.debt_scaled_in); 
                info!("swapOut {:?} {} {} {}", log.account, self.pools.get(&log.pool_out).unwrap().symbol, log.collateral_out, log.debt_scaled_out); 
                let user = log.account;
                //info!("swap {}", log);
                let borrower = self.borrowers.get_mut(&user).unwrap();
                borrower.positions.insert(
                    log.pool_in, 
                    Position {
                        pool:log.pool_in,
                        collateral:log.collateral_in,
                        debt_scaled:log.debt_scaled_in
                    }
                );
                borrower.positions.insert(
                    log.pool_out,
                    Position {
                        pool:log.pool_out,
                        collateral:log.collateral_out,
                        debt_scaled:log.debt_scaled_out
                    }
                );
                return;
            });

        self.get_liquidation_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("liquidation {}", log.account);
                let user = log.account;
                self.borrowers.remove(&user);
                return;
            });

        self.get_close_position_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("close_position {:?} {} {} {} {} ", log.account, self.pools.get(&log.pool).unwrap().symbol, self.pools.get(&log.pool_usd).unwrap().symbol, log.collateral_usd, log.debt_scaled_usd); 
                let user = log.account;
                let borrower = self.borrowers.get_mut(&user).unwrap();
                borrower.positions.remove(&log.pool);
                borrower.positions.insert(
                    log.pool_usd, 
                    Position {
                        pool:log.pool_usd,
                        collateral:log.collateral_usd,
                        debt_scaled:log.debt_scaled_usd,
                    }
                );
                return;
            });

        self.get_close_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("close {}", log.account);
                let user = log.account;
                self.borrowers.remove(&user);
                return;
            });

        // write state cache to file
        let cache = StateCache {
            last_block_number: latest_block.as_u64(),
            borrowers: self.borrowers.clone(),
            pools: self.pools.clone(),
        };
        self.last_block_number = latest_block.as_u64();
        let mut file = File::create(STATE_CACHE_FILE)?;
        file.write_all(serde_json::to_string(&cache)?.as_bytes())?;

        Ok(())
    }

    // fetch all deposit events from the from_block to to_block
    async fn get_deposit_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<DepositFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.deposit_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.event_emitter))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // fetch all borrow events from the from_block to to_block
    async fn get_borrow_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<BorrowFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.borrow_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.event_emitter))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // fetch all repay events from the from_block to to_block
    async fn get_repay_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<RepayFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.repay_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.event_emitter))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // fetch all redeem events from the from_block to to_block
    async fn get_redeem_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<RedeemFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.redeem_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.event_emitter))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // fetch all swap events from the from_block to to_block
    async fn get_swap_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<SwapFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.swap_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.event_emitter))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // fetch all liquidation events from the from_block to to_block
    async fn get_liquidation_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<LiquidationFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.liquidation_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.event_emitter))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // fetch all close position events from the from_block to to_block
    async fn get_close_position_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<ClosePositionFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.close_position_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.event_emitter))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // fetch all close events from the from_block to to_block
    async fn get_close_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<CloseFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.close_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.event_emitter))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // async fn approve_tokens(&mut self) -> Result<()> {
    //     let mut nonce = self
    //         .client
    //         .get_transaction_count(
    //             self.client
    //                 .default_sender()
    //                 .ok_or(anyhow!("No connected sender"))?,
    //             None,
    //         )
    //         .await?;
    //     for pool in self.pools.keys() {
    //         let underlying_asset = IERC20::new(pool.underlying_asset.clone(), self.client.clone());
    //         let allowance = underlying_asset
    //             .allowance(self.liquidator, self.config.liquidation_handler)
    //             .call()
    //             .await?;
    //         if allowance == U256::zero() {
    //             underlying_asset
    //                 .approve(self.config.liquidation_handler, U256::MAX()
    //                 .nonce(nonce)
    //                 .send()
    //                 .await
    //                 .map_err(|e| {
    //                     error!("approve failed: {:?}", e);
    //                     e
    //                 })?;
    //             nonce = nonce + 1;
    //         }
    //     }

    //     Ok(())
    // }

    async fn update_pools(&mut self) -> Result<()> {
        let reader = Reader::<M>::new(self.config.reader, self.client.clone());
        let all_pools = reader.get_pools_price(self.config.data_store).await?;
        //info!("all_pools: {:?}", all_pools);
        for pool in all_pools {
            info!("pool {:?} {} {} ", pool.underlying_asset, pool.symbol, pool.price);
            self.pools.insert(
                pool.underlying_asset,
                Pool {
                    underlying_asset: pool.underlying_asset,
                    symbol: pool.symbol,
                    price: pool.price,
                },
            );           
        }

        Ok(())
    }

    // async fn get_best_liquidation_opportunity(&mut self) -> Result<Option<LiquidationOpportunity>> {
    //     let underwaters = self.get_underwater_borrowers().await?;

    //     if underwaters.len() == 0 {
    //         return Err(anyhow!("No underwater borrowers found"));
    //     }

    //     info!("Found {} underwaters", underwaters.len());
    //     let mut best_bonus: I256 = I256::MIN;
    //     let mut best_op: Option<LiquidationOpportunity> = None;

    //     for (borrower, health_factor, user_total_collateral_usd, user_total_debt_usd) in underwaters {
    //         if let Some(op) = self
    //             .get_liquidation_profit(
    //                 self.borrowers
    //                     .get(&borrower)
    //                     .ok_or(anyhow!("Borrower not found"))?,
    //                 &health_factor,
    //                 &user_total_collateral_usd,
    //                 &user_total_debt_usd,
    //             )
    //             .await
    //             .map_err(|e| info!("Liquidation op failed {}", e))
    //             .ok()
    //         {
    //             if op.profit > best_bonus {
    //                 best_bonus = op.profit;
    //                 best_op = Some(op);
    //             }
    //         }
    //     }

    //     Ok(best_op)
    // }

    // async fn get_liquidation_profit(
    //     &self,
    //     borrower: &Borrower,
    //     health_factor: &U256,
    //     health_factor: &user_total_collateral_usd,
    //     health_factor: &user_total_debt_usd,
    // ) -> Result<LiquidationOpportunity> {
    //     let reader = Reader::<M>::new(self.config.reader, self.client.clone());
    //     let eth_price = reader.get_price(self.config.data_store, self.config.weth).await?;

    //     let mut op = LiquidationOpportunity {
    //         borrower: borrower.address,
    //         profit: (user_total_collateral_usd - user_total_debt_usd)*10_U256.pow(WETH_DECIMALS)/eth_price ,
    //     };

    //     Ok(op)
    // }

    // async fn build_liquidation_tx(&self, op: &LiquidationOpportunity) -> Result<TypedTransaction> {
    //     let exchange_router = ExchangeRouter::new(self.config.exchange_router, self.client.clone());
    //     let mut call = exchange_router.execute_liquidation({account:op.borrower});
    //     Ok(call.tx.set_chain_id(self.chain_id).clone())
    // }

}