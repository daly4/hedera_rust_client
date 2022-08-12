macro_rules! optional_try_from {
    ($fn_name:ident, $input_val_type:ty, $result_val_type:ty) => {
        pub fn $fn_name(
            val: Option<$input_val_type>,
        ) -> Result<Option<$result_val_type>, crate::error::HederaError> {
            let r = match val {
                Some(v) => Some(<$result_val_type>::try_from(v)?),
                None => None,
            };
            Ok(r)
        }
    };
}

macro_rules! non_optional_try_from {
    ($fn_name:ident, $input_val_type:ty, $result_val_type:ty) => {
        pub fn $fn_name(
            val: Option<$input_val_type>,
        ) -> Result<$result_val_type, crate::error::HederaError> {
            match val {
                Some(v) => Ok(<$result_val_type>::try_from(v)?),
                None => Err(crate::error::HederaError::MissingInProto(
                    stringify!($val_name).to_string(),
                )),
            }
        }
    };
}

macro_rules! non_optional_from {
    ($fn_name:ident, $val_name:ident, $input_val_type:ty, $result_val_type:ty) => {
        pub fn $fn_name(
            val: Option<$input_val_type>,
        ) -> Result<$result_val_type, crate::error::HederaError> {
            match val {
                Some(v) => Ok(<$result_val_type>::from(v)),
                None => Err(crate::error::HederaError::MissingInProto(
                    stringify!($val_name).to_string(),
                )),
            }
        }
    };
}

// macro_rules! optional_from {
//     ($fn_name:ident, $val_name:ident, $input_val_type:ty, $result_val_type:ty) => {
//         pub fn $fn_name(val: Option<$input_val_type>) -> Option<$result_val_type> {
//             match val {
//                 Some(v) => Ok(<$result_val_type>::from(v)),
//                 None => None,
//             }
//         }
//     };
// }

////////////////////////////////////////////////////////////////////////////////
// Common Get-Set
////////////////////////////////////////////////////////////////////////////////
macro_rules! gen_get_failable_fn {
    ($val:ident, $val_type:ty, $fn_name:ident, $fn:tt) => {
        pub fn $fn_name(&self) -> Result<$val_type, crate::error::HederaError> {
            match &self.services.$val {
                Some(v) => {
                    let res = ($fn)(v.clone())?;
                    Ok(res)
                }
                None => Err(crate::error::HederaError::ValueNotSet(
                    stringify!($val_name).to_string(),
                )),
            }
        }
    };
}

macro_rules! gen_query_set_failable_fn {
    ($val:ident, $val_type:ty, $target:ident, $fn_name:ident, $fn:tt) => {
        pub fn $fn_name(
            &mut self,
            $val: $val_type,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.services.$target = ($fn)($val)?;
            Ok(self)
        }
    };
}

macro_rules! gen_transaction_set_failable_fn {
    ($val:ident, $val_type:ty, $target:ident, $fn_name:ident, $fn:tt) => {
        pub fn $fn_name(
            &mut self,
            $val: $val_type,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.services.$target = ($fn)($val)?;
            Ok(self)
        }
    };
}

macro_rules! gen_transaction_set_failable_option_fn {
    ($val:ident, $val_type:ty, $target_type:ty, $target:ident, $fn_name:ident, $fn:tt) => {
        pub fn $fn_name(
            &mut self,
            $val: $val_type,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            let val: std::result::Result<$target_type, crate::error::HederaError> = ($fn)($val);
            self.services.$target = Some(val?);
            Ok(self)
        }
    };
}

macro_rules! gen_get_try_from_fn {
    ($val:ident, $val_type:ty, $try_from:path, $get_fn_name:ident) => {
        pub fn $get_fn_name(&self) -> Result<$val_type, crate::error::HederaError> {
            match &self.services.$val {
                Some(v) => {
                    let res = $try_from(v.clone())?;
                    Ok(res)
                }
                None => Err(crate::error::HederaError::ValueNotSet(
                    stringify!($val_name).to_string(),
                )),
            }
        }
    };
}

macro_rules! gen_get_proto_option_fn {
    ($val:ident, $val_type:ty, $get_fn_name:ident) => {
        pub fn $get_fn_name(&self) -> Result<$val_type, crate::error::HederaError> {
            match &self.services.$val {
                Some(v) => Ok(v.clone()),
                None => Err(crate::error::HederaError::ValueNotSet(
                    stringify!($val_name).to_string(),
                )),
            }
        }
    };
}

macro_rules! gen_get_proto_from_fn {
    ($val:ident, $val_type:ty, $from:path, $get_fn_name:ident) => {
        pub fn $get_fn_name(&self) -> Result<$val_type, crate::error::HederaError> {
            match &self.services.$val {
                Some(v) => {
                    let res = $from(v.clone());
                    Ok(res)
                }
                None => Err(crate::error::HederaError::ValueNotSet(
                    stringify!($val_name).to_string(),
                )),
            }
        }
    };
}

macro_rules! gen_get_proto_fn {
    ($val:ident, $val_type:ty, $get_fn_name:ident) => {
        pub fn $get_fn_name(&self) -> Result<$val_type, crate::error::HederaError> {
            Ok(self.services.$val.clone())
        }
    };
}

macro_rules! gen_u64_from_i64 {
    ($val:ident, $get_fn_name:ident) => {
        pub fn $get_fn_name(&self) -> u64 {
            u64::try_from(self.services.$val).expect("unable to convert to u64")
        }
    };
}

////////////////////////////////////////////////////////////////////////////////
// Transaction
////////////////////////////////////////////////////////////////////////////////
macro_rules! gen_transaction_set_proto_fn {
    ($val:ident, $val_type:ty, $set_fn_name:ident) => {
        pub fn $set_fn_name(
            &mut self,
            $val: $val_type,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.services.$val = $val;
            Ok(self)
        }
    };
}

macro_rules! gen_transaction_set_option_proto_fn {
    ($val:ident, $val_type:ty, $set_fn_name:ident) => {
        pub fn $set_fn_name(
            &mut self,
            $val: $val_type,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.services.$val = Some($val);
            Ok(self)
        }
    };
}

macro_rules! gen_transaction_get_set_pb_option_fns {
    ($val:ident, $val_type:ty, $get_fn_name:ident, $set_fn_name:ident) => {
        gen_get_proto_option_fn!($val, $val_type, $get_fn_name);
        gen_transaction_set_option_proto_fn!($val, $val_type, $set_fn_name);
    };
}

macro_rules! gen_transaction_get_set_pb_fns {
    ($val:ident, $val_type:ty, $get_fn_name:ident, $set_fn_name:ident) => {
        gen_get_proto_fn!($val, $val_type, $get_fn_name);
        gen_transaction_set_proto_fn!($val, $val_type, $set_fn_name);
    };
}

macro_rules! gen_transaction_get_set_fns {
    ($val:ident, $val_type:ty, $get_fn_name:ident, $set_fn_name:ident) => {
        pub fn $get_fn_name(&self) -> Result<$val_type, crate::error::HederaError> {
            Ok(self.$val.clone())
        }
        pub fn $set_fn_name(
            &mut self,
            $val: $val_type,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.$val = $val;
            Ok(self)
        }
    };
}

// u64 -> i64 util
macro_rules! gen_transaction_u64_to_i64 {
    ($val:ident, $target:ident, $fn_name:ident) => {
        pub fn $fn_name(&mut self, $val: u64) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.services.$target = i64::try_from($val)?;
            Ok(self)
        }
    };
}

// Auto renew period
macro_rules! gen_transaction_auto_renew_period_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            auto_renew_period,
            chrono::Duration,
            auto_renew_period,
            set_auto_renew_period
        );
    };
}

macro_rules! gen_transaction_expiration_time_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            expiration_time,
            chrono::DateTime<chrono::Utc>,
            expiration_time,
            set_expiration_time
        );
    };
}

macro_rules! gen_transaction_expiry_auto_renew_fns {
    () => {
        gen_transaction_auto_renew_period_fns!();

        gen_get_proto_option_fn!(expiry, chrono::DateTime<chrono::Utc>, expiration_time);

        pub fn set_expiration_time(
            &mut self,
            expiration_time: chrono::DateTime<chrono::Utc>,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.services.auto_renew_period = None;
            self.services.expiry = Some(expiration_time);
            Ok(self)
        }
    };
}

// token_type
macro_rules! gen_transaction_token_type_fns {
    () => {
        gen_transaction_get_set_pb_fns!(
            token_type,
            crate::token_type::TokenType,
            token_type,
            set_token_type
        );
    };
}

// supply_type
macro_rules! gen_transaction_supply_type_fns {
    () => {
        gen_transaction_get_set_pb_fns!(
            supply_type,
            crate::token_supply_type::TokenSupplyType,
            supply_type,
            set_supply_type
        );
    };
}

// max_supply
macro_rules! gen_transaction_max_supply_fns {
    () => {
        gen_transaction_get_set_pb_fns!(max_supply, i64, max_supply, set_max_supply);
    };
}

// custom_fees
macro_rules! gen_transaction_custom_fees_fns {
    () => {
        gen_transaction_get_set_pb_fns!(
            custom_fees,
            Vec<crate::custom_fee::CustomFee>,
            custom_fees,
            set_custom_fees
        );
    };
}

// admin key
macro_rules! gen_transaction_admin_key_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            admin_key,
            crate::key::Key,
            admin_key,
            set_admin_key
        );
    };
}

// kyc_key
macro_rules! gen_transaction_kyc_key_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(kyc_key, crate::key::Key, kyc_key, set_kyc_key);
    };
}

// submit_key
macro_rules! gen_transaction_submit_key_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            submit_key,
            crate::key::Key,
            submit_key,
            set_submit_key
        );
    };
}

// freeze_key
macro_rules! gen_transaction_freeze_key_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            freeze_key,
            crate::key::Key,
            freeze_key,
            set_freeze_key
        );
    };
}

// wipe_key
macro_rules! gen_transaction_wipe_key_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(wipe_key, crate::key::Key, wipe_key, set_wipe_key);
    };
}

// supply_key
macro_rules! gen_transaction_supply_key_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            supply_key,
            crate::key::Key,
            supply_key,
            set_supply_key
        );
    };
}

// fee_schedule_key
macro_rules! gen_transaction_fee_schedule_key_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            fee_schedule_key,
            crate::key::Key,
            fee_schedule_key,
            set_fee_schedule_key
        );
    };
}

// pause key
macro_rules! gen_transaction_pause_key_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            pause_key,
            crate::key::Key,
            pause_key,
            set_pause_key
        );
    };
}

macro_rules! gen_transaction_receiver_sig_required_fns {
    () => {
        gen_transaction_get_set_pb_fns!(
            receiver_sig_required,
            bool,
            fee_receiver_sig_required,
            set_receiver_sig_required
        );
    };
}

macro_rules! gen_transaction_max_automatic_token_associations_fns {
    () => {
        gen_transaction_get_set_pb_fns!(
            max_automatic_token_associations,
            i32,
            max_automatic_token_associations,
            set_max_automatic_token_associations
        );
    };
}

macro_rules! gen_transaction_max_automatic_token_associations_option_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            max_automatic_token_associations,
            i32,
            max_automatic_token_associations,
            set_max_automatic_token_associations
        );
    };
}

// memo
macro_rules! gen_transaction_memo_fns {
    () => {
        pub fn memo(&self) -> Result<String, crate::error::HederaError> {
            Ok(self.services.memo.clone())
        }

        pub fn set_memo(&mut self, memo: String) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            crate::memo::check_memo_length(&memo)?;
            self.services.memo = memo;
            Ok(self)
        }
    };
}

// memo
macro_rules! gen_transaction_optional_memo_fns {
    () => {
        pub fn memo(&self) -> Result<String, crate::error::HederaError> {
            match &self.services.memo {
                Some(v) => Ok(v.clone()),
                None => Err(crate::error::HederaError::ValueNotSet("memo".to_string())),
            }
        }

        pub fn set_memo(&mut self, memo: String) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            crate::memo::check_memo_length(&memo)?;
            self.services.memo = Some(memo);
            Ok(self)
        }
    };
}

// bytecode_file_id
macro_rules! gen_transaction_bytecode_file_id_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            file_id,
            crate::file_id::FileId,
            bytecode_file_id,
            set_bytecode_file_id
        );
    };
}

// file_id
macro_rules! gen_transaction_file_id_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            file_id,
            crate::file_id::FileId,
            file_id,
            set_file_id
        );
    };
}

// account_id
macro_rules! gen_transaction_account_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            account,
            crate::account_id::AccountId,
            account_id,
            set_account_id
        );
    };
}

macro_rules! gen_transaction_delete_account_id_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            delete_account_id,
            crate::account_id::AccountId,
            delete_account_id,
            set_delete_account_id
        );
    };
}

macro_rules! gen_transaction_account_id_to_update_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            account_id_to_update,
            crate::account_id::AccountId,
            account_id_to_update,
            set_account_id_to_update
        );
    };
}

// payer_account_id
macro_rules! gen_transaction_payer_account_id_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            payer_account_id,
            crate::account_id::AccountId,
            payer_account_id,
            set_payer_account_id
        );
    };
}

// treasury
macro_rules! gen_transaction_treasury_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            treasury,
            crate::account_id::AccountId,
            treasury,
            set_treasury
        );
    };
}

// proxy_account_id
macro_rules! gen_transaction_proxy_account_id_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            proxy_account_id,
            crate::account_id::AccountId,
            proxy_account_id,
            set_proxy_account_id
        );
    };
}

// decline award
macro_rules! gen_transaction_decline_award_fns {
    () => {
        gen_transaction_get_set_pb_fns!(decline_reward, bool, decline_reward, set_decline_reward);
    };
}

macro_rules! gen_transaction_decline_award_option_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            decline_reward,
            bool,
            decline_reward,
            set_decline_reward
        );
    };
}

// staked_id
macro_rules! gen_transaction_staked_id_option_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            staked_id,
            crate::staked_id::StakedId,
            staked_id,
            set_staked_id
        );
    };
}

// auto_renew_account
macro_rules! gen_transaction_auto_renew_account_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            auto_renew_account,
            crate::account_id::AccountId,
            auto_renew_account,
            set_auto_renew_account
        );
    };
}

macro_rules! gen_transaction_auto_renew_account_id_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            auto_renew_account_id,
            crate::account_id::AccountId,
            auto_renew_account_id,
            set_auto_renew_account_id
        );
    };
}

// initcode_source
macro_rules! gen_transaction_initcode_source_option_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            initcode_source,
            crate::initcode_source::InitcodeSource,
            initcode_source,
            set_initcode_source
        );
    };
}

// key
macro_rules! gen_transaction_key_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(key, crate::key::Key, key, set_key);
    };
}

// key_list
macro_rules! gen_transaction_keys_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(keys, crate::key_list::KeyList, keys, set_keys);
    };
}

// schedule_id
macro_rules! gen_transaction_schedule_id_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            schedule_id,
            crate::schedule_id::ScheduleId,
            schedule_id,
            set_schedule_id
        );
    };
}

// transfer_account_id
macro_rules! gen_transaction_transfer_account_id_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            transfer_account_id,
            crate::account_id::AccountId,
            transfer_account_id,
            set_transfer_account_id
        );
    };
}

// account_of_live_hash_fns
macro_rules! gen_transaction_account_of_live_hash_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            account_of_live_hash,
            crate::account_id::AccountId,
            account_of_live_hash,
            set_account_of_live_hash
        );
    };
}

macro_rules! gen_transaction_live_hash_to_delete_fns {
    () => {
        gen_transaction_get_set_pb_fns!(live_hash_to_delete, Vec<u8>, hash, set_hash);
    };
}

// contract_id
macro_rules! gen_transaction_contract_id_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            contract_id,
            crate::contract_id::ContractId,
            contract_id,
            set_contract_id
        );
    };
}

// token
macro_rules! gen_transaction_token_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            token,
            crate::token_id::TokenId,
            token_id,
            set_token_id
        );
    };
}

// token
macro_rules! gen_transaction_token_id_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            token_id,
            crate::token_id::TokenId,
            token_id,
            set_token_id
        );
    };
}

// topic
macro_rules! gen_transaction_topic_id_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            topic_id,
            crate::topic_id::TopicId,
            topic_id,
            set_topic_id
        );
    };
}

// contents
macro_rules! gen_transaction_contents_fns {
    () => {
        gen_transaction_get_set_pb_fns!(contents, Vec<u8>, contents, set_contents);
    };
}

// token name
macro_rules! gen_transaction_name_fns {
    () => {
        gen_transaction_get_set_pb_fns!(name, String, name, set_name);
    };
}

// token symbol
macro_rules! gen_transaction_symbol_fns {
    () => {
        gen_transaction_get_set_pb_fns!(symbol, String, symbol, set_symbol);
    };
}

// token decimals
macro_rules! gen_transaction_decimals_fns {
    () => {
        gen_transaction_get_set_pb_fns!(decimals, u32, decimals, set_decimals);
    };
}

// token initial_supply
macro_rules! gen_transaction_initial_supply_fns {
    () => {
        gen_transaction_get_set_pb_fns!(initial_supply, u64, initial_supply, set_initial_supply);
    };
}

// token amount
macro_rules! gen_transaction_amount_fns {
    () => {
        gen_transaction_get_set_pb_fns!(amount, u64, amount, set_amount);
    };
}

// freeze_default
macro_rules! gen_transaction_freeze_default_fns {
    () => {
        gen_transaction_get_set_pb_fns!(freeze_default, crate::freeze_default::FreezeDefault, freeze_default, set_freeze_default);
    };
}

macro_rules! gen_transaction_live_hash_fns {
    () => {
        gen_transaction_get_set_pb_option_fns!(
            live_hash,
            crate::live_hash::LiveHash,
            live_hash,
            set_live_hash
        );
    };
}

macro_rules! gen_transaction_metadatas_fns {
    () => {
        gen_get_proto_fn!(metadata, Vec<Vec<u8>>, metadata);

        pub fn set_metadata(
            &mut self,
            metadata: Vec<u8>,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.services.metadata = vec![metadata];
            Ok(self)
        }

        pub fn add_metadata(
            &mut self,
            metadata: Vec<u8>,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.services.metadata.push(metadata);
            Ok(self)
        }
    };
}

macro_rules! gen_transaction_serial_numbers_fns {
    () => {
        gen_transaction_get_set_pb_fns!(
            serial_numbers,
            Vec<i64>,
            serial_numbers,
            set_serial_numbers
        );

        pub fn set_serial_number(
            &mut self,
            serial_number: i64,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.services.serial_numbers.push(serial_number);
            Ok(self)
        }
    };
}

// gas
macro_rules! gen_transaction_gas_fns {
    () => {
        gen_u64_from_i64!(gas, gas);
        gen_transaction_u64_to_i64!(gas, gas, set_gas);
    };
}

macro_rules! gen_transaction_contract_params {
    ($val:ident, $get_fn_name:ident, $set_raw_fn_name:ident, $set_from_params_fn_name:ident) => {
        gen_transaction_get_set_pb_fns!($val, Vec<u8>, $get_fn_name, $set_raw_fn_name);

        pub fn $set_from_params_fn_name(
            &mut self,
            mut params: crate::contract_function_parameters::ContractFunctionParameters,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.services.$val = params.build(None)?;
            Ok(self)
        }
    };
}

// gen_transaction_get_set_with_hbar_i64
macro_rules! gen_transaction_get_set_with_hbar_i64 {
    ($val:ident, $get_fn_name:ident, $set_fn_name:ident) => {
        pub fn $get_fn_name(&self) -> Hbar {
            Hbar::from(self.services.$val)
        }

        pub fn $set_fn_name(&mut self, $val: Hbar) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.services.$val = $val.as_tinybar();
            Ok(self)
        }
    };
}

// gen_transaction_get_set_with_hbar_i64
macro_rules! gen_transaction_get_set_with_hbar_u64 {
    ($val:ident, $get_fn_name:ident, $set_fn_name:ident) => {
        pub fn $get_fn_name(&self) -> Hbar {
            Hbar::try_from(self.services.$val).expect("unable to convert to Hbar")
        }

        pub fn $set_fn_name(&mut self, $val: Hbar) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.services.$val = $val.as_tinybar_u64()?;
            Ok(self)
        }
    };
}

// gen_transaction_initial_balance_i64
macro_rules! gen_transaction_initial_balance_i64 {
    () => {
        gen_transaction_get_set_with_hbar_i64!(
            initial_balance,
            initial_balance,
            set_initial_balance
        );
    };
}

// gen_transaction_initial_balance_u64
macro_rules! gen_transaction_initial_balance_u64 {
    () => {
        gen_transaction_get_set_with_hbar_u64!(
            initial_balance,
            initial_balance,
            set_initial_balance
        );
    };
}

// tokens
// gen_transaction_initial_balance_u64
macro_rules! gen_transaction_tokens_fns {
    () => {
        gen_transaction_get_set_pb_fns!(tokens, Vec<crate::token_id::TokenId>, tokens, set_tokens);

        pub fn add_token(
            &mut self,
            val: crate::token_id::TokenId,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.require_not_frozen()?;
            self.services.tokens.push(val);
            Ok(self)
        }
    };
}

////////////////////////////////////////////////////////////////////////////////
// Query
////////////////////////////////////////////////////////////////////////////////
macro_rules! gen_query_execute {
    ($val_type:ty, $response_enum:ident, $fn:tt) => {
        pub async fn execute(
            &mut self,
            client: &crate::client::Client,
        ) -> Result<$val_type, crate::error::HederaError> {
            let res = self.execute_async(client).await?;
            let proto_response = res.to_proto_query()?;
            if let crate::proto::services::response::Response::$response_enum(res) = proto_response
            {
                let response = ($fn)(res)?;
                return Ok(response);
            }
            Err(crate::error::HederaError::UnexpectedProtoResponseType(
                format!("{:?}", proto_response),
            ))
        }
    };
}

macro_rules! gen_query_execute_with_cost_check {
    ($val_type:ty, $response_enum:ident, $fn:tt) => {
        pub async fn execute(
            &mut self,
            client: &crate::client::Client,
        ) -> Result<$val_type, crate::error::HederaError> {
            let res = self.execute_async_with_cost_check(client).await?;
            let proto_response = res.to_proto_query()?;
            if let crate::proto::services::response::Response::$response_enum(res) = proto_response
            {
                let response = ($fn)(res)?;
                return Ok(response);
            }
            Err(crate::error::HederaError::UnexpectedProtoResponseType(
                format!("{:?}", proto_response),
            ))
        }
    };
}

macro_rules! gen_query_execute_non_failable_with_cost_check {
    ($val_type:ty, $response_enum:ident, $fn:tt) => {
        pub async fn execute(
            &mut self,
            client: &crate::client::Client,
        ) -> Result<$val_type, crate::error::HederaError> {
            let res = self.execute_async_with_cost_check(client).await?;
            let proto_response = res.to_proto_query()?;
            if let crate::proto::services::response::Response::$response_enum(res) = proto_response
            {
                let response = ($fn)(res);
                return Ok(response);
            }
            Err(crate::error::HederaError::UnexpectedProtoResponseType(
                format!("{:?}", proto_response),
            ))
        }
    };
}

macro_rules! gen_query_set_to_proto_fn {
    ($val:ident, $val_type:ty, $set_fn_name:ident) => {
        pub fn $set_fn_name(
            &mut self,
            $val: $val_type,
        ) -> Result<&mut Self, crate::error::HederaError> {
            let val = $val.to_proto()?;
            self.services.$val = Some(val);
            Ok(self)
        }
    };
}

// u64 -> i64 util
macro_rules! gen_query_u64_to_i64 {
    ($val:ident, $target:ident, $fn_name:ident) => {
        pub fn $fn_name(&mut self, $val: u64) -> Result<&mut Self, crate::error::HederaError> {
            self.services.$target = i64::try_from($val)?;
            Ok(self)
        }
    };
}

macro_rules! gen_query_get_set_pb_simple_fns {
    ($val:ident, $val_type:ty, $get_fn_name:ident, $set_fn_name:ident) => {
        pub fn $get_fn_name(&self) -> $val_type {
            self.services.$val.clone()
        }
        pub fn $set_fn_name(
            &mut self,
            $val: $val_type,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.services.$val = $val;
            Ok(self)
        }
    };
}

// macro_rules! gen_query_get_set_pb_option_fns {
//     ($val:ident, $val_type:ty, $get_fn_name:ident, $set_fn_name:ident) => {
//         gen_get_proto_option_fn!($val, $val_type, $get_fn_name);
//         pub fn $set_fn_name(
//             &mut self,
//             $val: $val_type,
//         ) -> Result<&mut Self, crate::error::HederaError> {
//             self.services.$val = Some($val);
//             Ok(self)
//         }
//     };
// }

macro_rules! gen_query_get_set_from_proto_fns {
    ($val:ident, $val_type:ty, $from:path, $get_fn_name:ident, $set_fn_name:ident) => {
        gen_get_proto_from_fn!($val, $val_type, $from, $get_fn_name);
        gen_query_set_to_proto_fn!($val, $val_type, $set_fn_name);
    };
}

// Get-Set helper macro
macro_rules! gen_query_get_set_try_from_proto_fns {
    ($val:ident, $val_type:ty, $try_from:path, $get_fn_name:ident, $set_fn_name:ident) => {
        gen_get_try_from_fn!($val, $val_type, $try_from, $get_fn_name);
        gen_query_set_to_proto_fn!($val, $val_type, $set_fn_name);
    };
}

// transaction_id
macro_rules! gen_query_transaction_id_fns {
    () => {
        gen_query_get_set_try_from_proto_fns!(
            transaction_id,
            crate::transaction_id::TransactionId,
            crate::transaction_id::TransactionId::try_from,
            transaction_id,
            set_transaction_id
        );
    };
}

macro_rules! gen_query_account_id_fns {
    () => {
        gen_query_get_set_try_from_proto_fns!(
            account_id,
            crate::account_id::AccountId,
            crate::account_id::AccountId::try_from,
            account_id,
            set_account_id
        );
    };
}

macro_rules! gen_query_schedule_id_fns {
    () => {
        gen_query_get_set_from_proto_fns!(
            schedule_id,
            crate::schedule_id::ScheduleId,
            crate::schedule_id::ScheduleId::from,
            schedule_id,
            set_schedule_id
        );
    };
}

macro_rules! gen_query_contract_id_fns {
    () => {
        gen_query_get_set_from_proto_fns!(
            contract_id,
            crate::contract_id::ContractId,
            crate::contract_id::ContractId::from,
            contract_id,
            set_contract_id
        );
    };
}

macro_rules! gen_query_sender_id_option_fns {
    () => {
        gen_query_get_set_try_from_proto_fns!(
            sender_id,
            crate::account_id::AccountId,
            crate::account_id::AccountId::try_from,
            sender_id,
            set_sender_id
        );
    };
}

macro_rules! gen_query_file_id_fns {
    () => {
        gen_query_get_set_from_proto_fns!(
            file_id,
            crate::file_id::FileId,
            crate::file_id::FileId::from,
            file_id,
            set_file_id
        );
    };
}

macro_rules! gen_query_token_id_fns {
    () => {
        gen_query_get_set_from_proto_fns!(
            token,
            crate::token_id::TokenId,
            crate::token_id::TokenId::from,
            token_id,
            set_token_id
        );
    };
}

macro_rules! gen_query_nft_id_fns {
    () => {
        gen_query_get_set_try_from_proto_fns!(
            nft_id,
            crate::nft_id::NftId,
            crate::nft_id::NftId::try_from,
            nft_id,
            set_nft_id
        );
    };
}

macro_rules! gen_query_topic_id_fns {
    () => {
        gen_query_get_set_from_proto_fns!(
            topic_id,
            crate::topic_id::TopicId,
            crate::topic_id::TopicId::from,
            topic_id,
            set_topic_id
        );
    };
}

macro_rules! gen_query_include_duplicates_fns {
    () => {
        gen_query_get_set_pb_simple_fns!(
            include_duplicates,
            bool,
            include_duplicates,
            set_include_duplicates
        );
    };
}

macro_rules! gen_query_include_child_receipts_fns {
    () => {
        gen_query_get_set_pb_simple_fns!(
            include_child_receipts,
            bool,
            include_child_receipts,
            set_include_child_receipts
        );
    };
}

macro_rules! gen_query_include_child_records_fns {
    () => {
        gen_query_get_set_pb_simple_fns!(
            include_child_records,
            bool,
            include_child_records,
            set_include_child_records
        );
    };
}

// gas
macro_rules! gen_query_gas_fns {
    () => {
        gen_u64_from_i64!(gas, gas);
        gen_query_u64_to_i64!(gas, gas, set_gas);
    };
}

macro_rules! gen_query_contract_params {
    ($val:ident, $get_fn_name:ident, $set_raw_fn_name:ident, $set_from_params_fn_name:ident) => {
        pub fn $get_fn_name(&self) -> Vec<u8> {
            self.services.$val.clone()
        }

        pub fn $set_raw_fn_name(
            &mut self,
            params: Vec<u8>,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.services.$val = params;
            Ok(self)
        }

        pub fn $set_from_params_fn_name(
            &mut self,
            mut params: crate::contract_function_parameters::ContractFunctionParameters,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.services.$val = params.build(None)?;
            Ok(self)
        }
    };
}

macro_rules! gen_query_consensus_start_time_fns {
    () => {
        gen_query_get_set_try_from_proto_fns!(
            consensus_start_time,
            chrono::DateTime<chrono::Utc>,
            chrono::DateTime<chrono::Utc>::try_from,
            consensus_start_time,
            set_consensus_start_time
        );
    };
}

macro_rules! gen_query_consensus_end_time_fns {
    () => {
        gen_query_get_set_try_from_proto_fns!(
            consensus_end_time,
            chrono::DateTime<chrono::Utc>,
            chrono::DateTime<chrono::Utc>::try_from,
            consensus_end_time,
            set_consensus_end_time
        );
    };
}

macro_rules! gen_query_non_optional {
    ($val:ident, $val_type:ty, $get_fn_name:ident, $set_fn_name:ident) => {
        pub fn $get_fn_name(&self) -> $val_type {
            self.services.$val
        }

        pub fn $set_fn_name(
            &mut self,
            $val: $val_type,
        ) -> Result<&mut Self, crate::error::HederaError> {
            self.services.$val = $val;
            Ok(self)
        }
    };
}
