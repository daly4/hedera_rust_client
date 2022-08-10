use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, 
    DeriveInput, 
};
use darling::FromDeriveInput;

mod utils;
mod id;
mod query_base;
mod query_executable;
mod query_execute_async_with_cost_check;
mod query_execute_async;
mod query_get_cost;
mod transaction_base;
mod transaction_executable;
mod transaction_execute;
mod transaction_schedule;
mod transaction_chunked;
mod transaction_proto;
mod transaction_chunked_schedule;

////////////////////////////////////////////////////////////////////////////////
// AccountId
////////////////////////////////////////////////////////////////////////////////
#[proc_macro_derive(Id, attributes(hedera_rust_client_derive))]
pub fn id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = match id::Id::from_derive_input(&parse_macro_input!(input)) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    proc_macro::TokenStream::from(output)
}

#[proc_macro_derive(IdProto, attributes(hedera_rust_client_derive))]
pub fn id_proto(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = match id::IdProto::from_derive_input(&parse_macro_input!(input)) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    proc_macro::TokenStream::from(output)
}

#[proc_macro_derive(IdPartialEq, attributes(hedera_rust_client_derive))]
pub fn id_sup(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = match id::IdSup::from_derive_input(&parse_macro_input!(input)) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    proc_macro::TokenStream::from(output)
}

#[proc_macro_derive(IdValidateChecksum, attributes(hedera_rust_client_derive))]
pub fn id_val(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = match id::IdValidateChecksum::from_derive_input(&parse_macro_input!(input)) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    proc_macro::TokenStream::from(output)
}

////////////////////////////////////////////////////////////////////////////////
// QueryBase
////////////////////////////////////////////////////////////////////////////////
#[proc_macro_derive(QueryBase)]
pub fn query_base(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let expanded = query_base::gen_query_base(&ast);
    TokenStream::from(expanded)
}

////////////////////////////////////////////////////////////////////////////////
// QueryExecutable
////////////////////////////////////////////////////////////////////////////////
#[proc_macro_derive(QueryExecutable, attributes(hedera_rust_client_derive))]
pub fn query_executable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = match query_executable::QueryExecutable::from_derive_input(&parse_macro_input!(input)) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    proc_macro::TokenStream::from(output)
}

////////////////////////////////////////////////////////////////////////////////
// QueryGetCost
////////////////////////////////////////////////////////////////////////////////
#[proc_macro_derive(QueryGetCost, attributes(hedera_rust_client_derive))]
pub fn query_get_cost(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = match query_get_cost::QueryGetCost::from_derive_input(&parse_macro_input!(input)) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    proc_macro::TokenStream::from(output)
}


////////////////////////////////////////////////////////////////////////////////
// QueryExecuteAsync
////////////////////////////////////////////////////////////////////////////////
#[proc_macro_derive(QueryExecuteAsync, attributes(hedera_rust_client_derive))]
pub fn query_execute_async(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let base = query_base::gen_query_base(&ast);
    let executable = match query_executable::QueryExecutable::from_derive_input(&ast) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    let execute = match query_execute_async::QueryExecute::from_derive_input(&ast) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    let expanded = quote! {
        #base
        #executable
        #execute
    };
    TokenStream::from(expanded)
}


////////////////////////////////////////////////////////////////////////////////
// QueryExecuteAsyncWithCostCheck
////////////////////////////////////////////////////////////////////////////////
#[proc_macro_derive(QueryExecuteAsyncWithCostCheck, attributes(hedera_rust_client_derive))]
pub fn query_execute_with_cost_check(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);
    let base = query_base::gen_query_base(&ast);
    let executable = match query_executable::QueryExecutable::from_derive_input(&ast) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    let cost = match query_get_cost::QueryGetCost::from_derive_input(&ast) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    let execute = match query_execute_async_with_cost_check::QueryExecute::from_derive_input(&ast) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    let expanded = quote! {
        #base
        #executable
        #cost
        #execute
    };
    TokenStream::from(expanded)
}

////////////////////////////////////////////////////////////////////////////////
// TransactionBase
////////////////////////////////////////////////////////////////////////////////
#[proc_macro_derive(TransactionBase)]
pub fn transaction_base(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);
    let expanded = transaction_base::gen_transaction_base(&ast);
    TokenStream::from(expanded)
}


////////////////////////////////////////////////////////////////////////////////
// TransactionExecutable
////////////////////////////////////////////////////////////////////////////////
#[proc_macro_derive(TransactionExecutable, attributes(hedera_rust_client_derive))]
pub fn transaction_executable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = match transaction_executable::TransactionExecutable::from_derive_input(&parse_macro_input!(input)) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    proc_macro::TokenStream::from(output)
}


////////////////////////////////////////////////////////////////////////////////
// TransactionSchedule
////////////////////////////////////////////////////////////////////////////////
// schedule
#[proc_macro_derive(TransactionSchedule)]
pub fn transaction_schedule(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let expanded = transaction_schedule::gen_transaction_schedule(&ast);
    TokenStream::from(expanded)
}


////////////////////////////////////////////////////////////////////////////////
// TransactionExecute
////////////////////////////////////////////////////////////////////////////////
// execute
#[proc_macro_derive(TransactionExecute, attributes(hedera_rust_client_derive))]
pub fn transaction_execute(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let base = transaction_base::gen_transaction_base(&ast);
    let executable = match transaction_executable::TransactionExecutable::from_derive_input(&ast) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    let execute = transaction_execute::gen_transaction_execute(&ast);
    let expanded = quote! {
        #base
        #executable
        #execute
    };
    TokenStream::from(expanded)
}

////////////////////////////////////////////////////////////////////////////////
// TransactionProto
////////////////////////////////////////////////////////////////////////////////
#[proc_macro_derive(TransactionProto, attributes(hedera_rust_client_derive))]
pub fn derive_transaction_proto(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = match transaction_proto::TransactionProto::from_derive_input(&parse_macro_input!(input)) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    proc_macro::TokenStream::from(output)
}


////////////////////////////////////////////////////////////////////////////////
// TransactionChunked
////////////////////////////////////////////////////////////////////////////////
#[proc_macro_derive(TransactionChunked, attributes(hedera_rust_client_derive))]
pub fn transaction_chunked(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let base = transaction_base::gen_transaction_base(&ast);
    let chunked = match transaction_chunked::TransactionChunked::from_derive_input(&ast) {
        Ok(object_args) => object_args.into_token_stream(),
        Err(err) => proc_macro2::TokenStream::from(err.write_errors()),
    };
    let expanded = quote! {
        #base
        #chunked
    };
    TokenStream::from(expanded)
}


////////////////////////////////////////////////////////////////////////////////
// TransactionChunkedSchedule
////////////////////////////////////////////////////////////////////////////////
// schedule
#[proc_macro_derive(TransactionChunkedSchedule)]
pub fn transaction_chunked_schedule(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let expanded = transaction_chunked_schedule::gen_transaction_chunked_schedule(&ast);
    TokenStream::from(expanded)
}