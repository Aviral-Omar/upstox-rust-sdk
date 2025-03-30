use {
    crate::{
        client::ApiClient,
        constants::{
            APIVersion, BaseUrlType, GTT_ORDERS_CANCEL_GTT_ORDER_ENDPOINT,
            GTT_ORDERS_GTT_ORDER_DETAILS_ENDPOINT, GTT_ORDERS_MODIFY_GTT_ORDER_ENDPOINT,
            GTT_ORDERS_PLACE_GTT_ORDER_ENDPOINT,
        },
        models::{
            error_response::ErrorResponse,
            gtt_orders::{
                cancel_gtt_order_request::CancelGTTOrderRequest,
                gtt_order_details_request::GTTOrderDetailsRequest,
                gtt_order_details_response::GTTOrderDetailsResponse,
                gtt_orders_response::GTTOrdersResponse,
                modify_gtt_order_request::ModifyGTTOrderRequest,
                place_gtt_order_request::PlaceGTTOrderRequest,
            },
            success_response::SuccessResponse,
        },
        rate_limiter::RateLimitExceeded,
        utils::ToKeyValueTuples,
    },
    serde_valid::Validate,
};

impl ApiClient {
    pub async fn place_gtt_order(
        &self,
        place_gtt_order_body: PlaceGTTOrderRequest,
    ) -> Result<Result<SuccessResponse<GTTOrdersResponse>, ErrorResponse>, RateLimitExceeded> {
        place_gtt_order_body.validate().unwrap();
        let res: reqwest::Response = self
            .post(
                GTT_ORDERS_PLACE_GTT_ORDER_ENDPOINT,
                true,
                Some(&place_gtt_order_body.to_key_value_tuples_vec()),
                None,
                BaseUrlType::REGULAR,
                APIVersion::V3,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<GTTOrdersResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn modify_gtt_order(
        &self,
        modify_gtt_order_body: ModifyGTTOrderRequest,
    ) -> Result<Result<SuccessResponse<GTTOrdersResponse>, ErrorResponse>, RateLimitExceeded> {
        modify_gtt_order_body.validate().unwrap();
        let res: reqwest::Response = self
            .put(
                GTT_ORDERS_MODIFY_GTT_ORDER_ENDPOINT,
                true,
                Some(&modify_gtt_order_body.to_key_value_tuples_vec()),
                None,
                BaseUrlType::REGULAR,
                APIVersion::V3,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<GTTOrdersResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn cancel_gtt_order(
        &self,
        gtt_order_id: String,
    ) -> Result<Result<SuccessResponse<GTTOrdersResponse>, ErrorResponse>, RateLimitExceeded> {
        let cancel_order_params: CancelGTTOrderRequest = CancelGTTOrderRequest { gtt_order_id };
        cancel_order_params.validate().unwrap();

        let res: reqwest::Response = self
            .delete(
                GTT_ORDERS_CANCEL_GTT_ORDER_ENDPOINT,
                true,
                None,
                Some(&cancel_order_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V3,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<GTTOrdersResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_gtt_order_details(
        &self,
        gtt_order_details_params: GTTOrderDetailsRequest,
    ) -> Result<
        Result<SuccessResponse<Vec<GTTOrderDetailsResponse>>, ErrorResponse>,
        RateLimitExceeded,
    > {
        gtt_order_details_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                GTT_ORDERS_GTT_ORDER_DETAILS_ENDPOINT,
                true,
                Some(&gtt_order_details_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V3,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 | 207 => Ok(res
                .json::<SuccessResponse<Vec<GTTOrderDetailsResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }
}
