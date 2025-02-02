// src/errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Unauthorized: Only the contract owner can perform this action.")]
    Unauthorized,

    #[msg("Insufficient Funds: The user does not have enough tokens to stake or unstake.")]
    InsufficientFunds,

    #[msg("Account Not Initialized: User's staking account is not initialized.")]
    AccountNotInitialized,

    #[msg("Invalid Token: The provided token is not valid for this staking pool.")]
    InvalidToken,

    #[msg("Staking Period Not Reached: The staking lock period has not been completed yet.")]
    StakingPeriodNotReached,

    #[msg("Invalid Amount: The amount provided for staking or unstaking is invalid.")]
    InvalidAmount,

    #[msg("Invalid APR or Locktime: The provided APR or locktime values are not valid.")]
    InvalidPoolParameters,

    #[msg("No Rewards Available: The user has no rewards to claim.")]
    NoRewards,

    #[msg("Invalid Price Feed: The price feed data is invalid or unavailable.")]
    InvalidPriceFeed,

    #[msg("Invalid Pool Name: The provided pool name is not valid.")]
    InvalidPoolName,

    #[msg("Pool Name Too Long: The provided pool name is too long.")]
    PoolNameTooLong,

    #[msg("Invalid APR: The provided APR value is not valid (must be between 1 and 100).")]
    InvalidAPR,

    #[msg("Invalid Locktime: The provided locktime value is not valid (must be greater than 0).")]
    InvalidLocktime,

    #[msg("Math Overflow: An arithmetic operation caused an overflow or underflow condition.")]
    MathOverflow,

    #[msg("Invalid Token Account: The provided token account is not valid for this staking pool.")]
    InvalidTokenAccount,

    #[msg("Invalid price feed account")]
    InvalidPriceAccount,

    #[msg("Invalid price data")]
    InvalidPrice,

    #[msg("Reward amount too small")]
    RewardTooSmall,

    #[msg("Invalid calculation")]
    InvalidCalculation,

    #[msg("No stake")]
    NoStake,

    #[msg("Insufficient contract balance")]
    InsufficientContractBalance,
}