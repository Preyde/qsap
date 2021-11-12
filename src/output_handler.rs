use sap_adt_bindings::config::{
    freestyle_config::{FreeStyleConfig, FreeStyleResponse},
    AdtError, AdtResponse, Config, Responses,
};

pub mod output_handler {}

// pub trait Handler<C, T, E>
// where
//     C: Config<T, E>,
//     T: AdtResponse<Responses>,
//     E: AdtError,
// {
//     fn handle(response: Responses);
// }

pub struct OutputHandler {}

// impl OutputHandler {
//     fn handle<T, E>(response: Result<T, E>)
//     where
//         T: AdtResponse<Responses> + Sized,
//         E: AdtError,
//     {
//     }
// }
// impl Handler<FreeStyleResponse> for OutputHandler {
//     fn handle(res: FreeStyleResponse)
//     where
//         T: Config + Sized,
//     {
//         res.get_table();
//     }
// }
