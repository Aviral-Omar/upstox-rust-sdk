use {
    crate::{
        client::ApiClient,
        constants::{OPTION_CHAIN_ENDPOINT, OPTION_CONTRACTS_ENDPOINT},
        models::{
            error_response::ErrorResponse,
            option_chain::{
                option_contracts_request::OptionContractsRequest,
                option_contracts_response::OptionContractResponse,
                put_call_option_chain_request::OptionChainRequest,
                put_call_option_chain_response::OptionChainResponse,
            },
            success_response::SuccessResponse,
        },
        utils::ToKeyValueTuples,
    },
    serde_valid::Validate,
};

impl ApiClient {
    pub async fn get_option_contracts(
        &self,
        option_contracts_params: OptionContractsRequest,
    ) -> Result<SuccessResponse<Vec<OptionContractResponse>>, ErrorResponse> {
        option_contracts_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                OPTION_CONTRACTS_ENDPOINT,
                true,
                Some(&option_contracts_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<OptionContractResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_option_chains(
        &self,
        option_chains_params: OptionChainRequest,
    ) -> Result<SuccessResponse<Vec<OptionChainResponse>>, ErrorResponse> {
        option_chains_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                OPTION_CHAIN_ENDPOINT,
                true,
                Some(&option_chains_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<OptionChainResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }
}
