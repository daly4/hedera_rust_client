use hedera_derive::{QueryExecuteAsync, QueryGetCost};
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services::crypto_get_account_balance_query::BalanceSource;
use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;
use crate::AccountBalance;
use crate::AccountId;
use crate::ContractId;

#[derive(QueryGetCost, QueryExecuteAsync, Debug, Clone)]
#[hedera_derive(
    proto(
        proto_enum = "CryptogetAccountBalance",
        response_enum = "CryptogetAccountBalance",
    ),
    service(
        method_service_name = "crypto",
        method_service_fn = "crypto_get_balance",
    )
)]
pub struct AccountBalanceQuery {
    query: Query,
    header: QueryHeader,
    services: services::CryptoGetAccountBalanceQuery,
}

impl AccountBalanceQuery {
    pub fn new() -> AccountBalanceQuery {
        let header = QueryHeader::new();
        let query = Query::new(false);
        let services = services::CryptoGetAccountBalanceQuery {
            header: None,
            balance_source: None,
        };
        AccountBalanceQuery { query, header, services }
    }

    // SetAccountID sets the AccountID for which you wish to query the balance.
    //
    // Note: you can only query an Account or Contract but not both -- if a Contract ID or Account ID has already been set,
    // it will be overwritten by this method.
    // set_account_id
    gen_query_set_failable_fn!(
        id,
        AccountId,
        balance_source,
        set_account_id,
        (|id: AccountId| {
            let id = match id.to_proto() {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            Ok(Some(BalanceSource::AccountId(id)))
        })
    );

    // account_id
    gen_get_failable_fn!(
        balance_source,
        AccountId,
        account_id,
        (|v: BalanceSource| match v {
            BalanceSource::AccountId(id) => Ok(AccountId::try_from(id.clone())?),
            _ => Err(HederaError::InvalidSetType),
        })
    );

    // SetContractID sets the ContractID for which you wish to query the balance.
    //
    // Note: you can only query an Account or Contract but not both -- if a Contract ID or Account ID has already been set,
    // it will be overwritten by this method.
    gen_query_set_failable_fn!(
        id,
        ContractId,
        balance_source,
        set_contract_id,
        (|id: ContractId| {
            let id = match id.to_proto() {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            Ok(Some(BalanceSource::ContractId(id)))
        })
    );

    // contract_id
    gen_get_failable_fn!(
        balance_source,
        ContractId,
        contract_id,
        (|v: BalanceSource| match v {
            BalanceSource::ContractId(id) => Ok(ContractId::from(id.clone())),
            _ => Err(HederaError::InvalidSetType),
        })
    );

    gen_query_execute!(
        AccountBalance,
        CryptogetAccountBalance,
        (|res: services::CryptoGetAccountBalanceResponse| AccountBalance::try_from(res))
    );
}
