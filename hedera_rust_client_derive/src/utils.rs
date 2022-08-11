use darling::FromMeta;
use proc_macro2::{Ident, Span};
use syn::Path;

pub fn to_ident(d: &str) -> Ident {
    Ident::new(d, Span::call_site())
}

pub fn to_path(d: &str) -> Path {
    syn::parse_str::<Path>(d).expect("unable to parse map_response")
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub struct ExeFields {
    pub get_node_account_id: Option<String>,
    pub get_method: Option<String>,
    pub make_request: Option<String>,
    pub advance_request: Option<String>,
    pub map_response_status: Option<String>,
    pub should_retry: Option<String>,
    pub map_response: Option<String>,
}

impl ExeFields {
    pub fn get_node_account_id(&self, d: &'static str) -> Path {
        to_path(&self.get_node_account_id.as_ref().unwrap_or(&d.to_string()))
    }
    pub fn get_method(&self, d: &'static str) -> Path {
        to_path(&self.get_method.as_ref().unwrap_or(&d.to_string()))
    }
    pub fn make_request(&self, d: &'static str) -> Path {
        to_path(&self.make_request.as_ref().unwrap_or(&d.to_string()))
    }
    pub fn advance_request(&self, d: &'static str) -> Path {
        to_path(&self.advance_request.as_ref().unwrap_or(&d.to_string()))
    }
    pub fn map_response_status(&self, d: &'static str) -> Path {
        to_path(&self.map_response_status.as_ref().unwrap_or(&d.to_string()))
    }
    pub fn should_retry(&self, d: &'static str) -> Path {
        to_path(&self.should_retry.as_ref().unwrap_or(&d.to_string()))
    }
    pub fn map_response(&self, d: &'static str) -> Path {
        to_path(&self.map_response.as_ref().unwrap_or(&d.to_string()))
    }
}

pub fn map_exe(
    get_node_account_id_str: &'static str,
    get_method_str: &'static str,
    make_request_str: &'static str,
    advance_request_str: &'static str,
    map_response_status_str: &'static str,
    should_retry_str: &'static str,
    map_response_str: &'static str,
    exe: &Option<ExeFields>,
) -> (Path, Path, Path, Path, Path, Path, Path) {
    let get_node_account_id;
    let get_method;
    let make_request;
    let advance_request;
    let map_response_status;
    let should_retry;
    let map_response;

    match exe {
        Some(exe) => {
            get_node_account_id = exe.get_node_account_id(get_node_account_id_str);
            get_method = exe.get_method(get_method_str);
            make_request = exe.make_request(make_request_str);
            advance_request = exe.advance_request(advance_request_str);
            map_response_status = exe.map_response_status(map_response_status_str);
            should_retry = exe.should_retry(should_retry_str);
            map_response = exe.map_response(map_response_str);
        }
        None => {
            get_node_account_id = to_path(get_node_account_id_str);
            get_method = to_path(get_method_str);
            make_request = to_path(make_request_str);
            advance_request = to_path(advance_request_str);
            map_response_status = to_path(map_response_status_str);
            should_retry = to_path(should_retry_str);
            map_response = to_path(map_response_str);
        }
    }
    (
        get_node_account_id,
        get_method,
        make_request,
        advance_request,
        map_response_status,
        should_retry,
        map_response,
    )
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub struct ProtoTypeFields {
    pub proto_enum: Option<String>,
    pub proto_type: Option<String>,
}

impl ProtoTypeFields {
    pub fn proto_enum(&self) -> Ident {
        to_ident(&self.proto_enum.as_ref().unwrap())
    }
    pub fn proto_type(&self) -> Ident {
        to_ident(&self.proto_type.as_ref().unwrap())
    }
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub struct ProtoFields {
    pub proto_enum: Option<String>,
    pub response_enum: Option<String>,
}

impl ProtoFields {
    pub fn proto_enum(&self) -> Ident {
        to_ident(&self.proto_enum.as_ref().unwrap())
    }
    pub fn response_enum(&self) -> Ident {
        to_ident(&self.response_enum.as_ref().unwrap())
    }
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub struct ServiceFields {
    pub method_service_name: Option<String>,
    pub method_service_fn: Option<String>,
}

impl ServiceFields {
    pub fn method_service_name(&self) -> Path {
        to_path(&self.method_service_name.as_ref().unwrap())
    }
    pub fn method_service_fn(&self) -> Path {
        to_path(&self.method_service_fn.as_ref().unwrap())
    }
}
