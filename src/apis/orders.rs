use {
    crate::{
        client::ApiClient,
        constants::{
            APIVersion, BaseUrlType, ORDERS_CANCEL_MULTI_ORDER_ENDPOINT,
            ORDERS_CANCEL_ORDER_ENDPOINT, ORDERS_EXIT_ALL_POSITIONS_ENDPOINT,
            ORDERS_MODIFY_ORDER_ENDPOINT, ORDERS_ORDER_BOOK_ENDPOINT,
            ORDERS_ORDER_DETAILS_ENDPOINT, ORDERS_ORDER_HISTORY_ENDPOINT,
            ORDERS_ORDER_TRADES_ENDPOINT, ORDERS_PLACE_MULTI_ORDER_ENDPOINT,
            ORDERS_PLACE_ORDER_ENDPOINT, ORDERS_TRADE_HISTORY_ENDPOINT, ORDERS_TRADES_ENDPOINT,
        },
        models::{
            error_response::ErrorResponse,
            orders::{
                cancel_multi_order_request::CancelMultiOrderRequest,
                cancel_multi_order_response::CancelMultiOrderResponse,
                cancel_order_request::CancelOrderRequest,
                exit_all_positions_request::ExitAllPositionsRequest,
                modify_order_request::ModifyOrderRequest,
                order_details_request::OrderDetailsRequest,
                order_details_response::OrderDetailsResponse, order_response::OrderResponse,
                order_trades_request::OrderTradesRequest, orders_response::OrdersResponse,
                place_multi_order_request::PlaceMultiOrderRequest,
                place_multi_order_response::PlaceMultiOrderResponse,
                place_order_request::PlaceOrderRequest,
                place_order_v3_request::PlaceOrderV3Request,
                trade_details_response::TradeDetailsResponse,
                trade_history_request::TradeHistoryRequest,
                trade_history_response::TradeHistoryResponse,
            },
            success_response::SuccessResponse,
        },
        rate_limiter::RateLimitExceeded,
        utils::ToKeyValueTuples,
    },
    serde_valid::Validate,
};

impl ApiClient {
    #[deprecated(note = "Use place_order_v3 instead")]
    pub async fn place_order(
        &self,
        place_order_body: PlaceOrderRequest,
        sandbox: bool,
    ) -> Result<Result<SuccessResponse<OrderResponse>, ErrorResponse>, RateLimitExceeded> {
        place_order_body.validate().unwrap();
        let res: reqwest::Response = self
            .post(
                ORDERS_PLACE_ORDER_ENDPOINT,
                true,
                Some(&place_order_body.to_key_value_tuples_vec()),
                None,
                if sandbox {
                    BaseUrlType::SANDBOX
                } else {
                    BaseUrlType::HFT
                },
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res.json::<SuccessResponse<OrderResponse>>().await.unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn place_order_v3(
        &self,
        place_order_body: PlaceOrderV3Request,
        sandbox: bool,
    ) -> Result<Result<SuccessResponse<OrdersResponse>, ErrorResponse>, RateLimitExceeded> {
        place_order_body.validate().unwrap();
        let res: reqwest::Response = self
            .post(
                ORDERS_PLACE_ORDER_ENDPOINT,
                true,
                Some(&place_order_body.to_key_value_tuples_vec()),
                None,
                if sandbox {
                    BaseUrlType::SANDBOX
                } else {
                    BaseUrlType::HFT
                },
                APIVersion::V3,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res.json::<SuccessResponse<OrdersResponse>>().await.unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn place_multi_order(
        &self,
        place_multi_order_body: Vec<PlaceMultiOrderRequest>,
        sandbox: bool,
    ) -> Result<Result<SuccessResponse<PlaceMultiOrderResponse>, ErrorResponse>, RateLimitExceeded>
    {
        place_multi_order_body.validate().unwrap();
        let res: reqwest::Response = self
            .post(
                ORDERS_PLACE_MULTI_ORDER_ENDPOINT,
                true,
                Some(&place_multi_order_body.to_key_value_tuples_vec()),
                None,
                if sandbox {
                    BaseUrlType::SANDBOX
                } else {
                    BaseUrlType::REGULAR
                },
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<PlaceMultiOrderResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    #[deprecated(note = "Use modify_order_v3 instead")]
    pub async fn modify_order(
        &self,
        modify_order_body: ModifyOrderRequest,
        sandbox: bool,
    ) -> Result<Result<SuccessResponse<OrderResponse>, ErrorResponse>, RateLimitExceeded> {
        modify_order_body.validate().unwrap();
        let res: reqwest::Response = self
            .put(
                ORDERS_MODIFY_ORDER_ENDPOINT,
                true,
                Some(&modify_order_body.to_key_value_tuples_vec()),
                None,
                if sandbox {
                    BaseUrlType::SANDBOX
                } else {
                    BaseUrlType::HFT
                },
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res.json::<SuccessResponse<OrderResponse>>().await.unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn modify_order_v3(
        &self,
        modify_order_body: ModifyOrderRequest,
        sandbox: bool,
    ) -> Result<Result<SuccessResponse<OrderResponse>, ErrorResponse>, RateLimitExceeded> {
        modify_order_body.validate().unwrap();
        let res: reqwest::Response = self
            .put(
                ORDERS_MODIFY_ORDER_ENDPOINT,
                true,
                Some(&modify_order_body.to_key_value_tuples_vec()),
                None,
                if sandbox {
                    BaseUrlType::SANDBOX
                } else {
                    BaseUrlType::HFT
                },
                APIVersion::V3,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res.json::<SuccessResponse<OrderResponse>>().await.unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    #[deprecated(note = "Use cancel_order_v3 instead")]
    pub async fn cancel_order(
        &self,
        order_id: String,
        sandbox: bool,
    ) -> Result<Result<SuccessResponse<OrderResponse>, ErrorResponse>, RateLimitExceeded> {
        let cancel_order_params: CancelOrderRequest = CancelOrderRequest { order_id };
        cancel_order_params.validate().unwrap();

        let res: reqwest::Response = self
            .delete::<()>(
                ORDERS_CANCEL_ORDER_ENDPOINT,
                true,
                Some(&cancel_order_params.to_key_value_tuples_vec()),
                None,
                if sandbox {
                    BaseUrlType::SANDBOX
                } else {
                    BaseUrlType::HFT
                },
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res.json::<SuccessResponse<OrderResponse>>().await.unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn cancel_order_v3(
        &self,
        order_id: String,
        sandbox: bool,
    ) -> Result<Result<SuccessResponse<OrderResponse>, ErrorResponse>, RateLimitExceeded> {
        let cancel_order_params: CancelOrderRequest = CancelOrderRequest { order_id };
        cancel_order_params.validate().unwrap();

        let res: reqwest::Response = self
            .delete::<()>(
                ORDERS_CANCEL_ORDER_ENDPOINT,
                true,
                Some(&cancel_order_params.to_key_value_tuples_vec()),
                None,
                if sandbox {
                    BaseUrlType::SANDBOX
                } else {
                    BaseUrlType::HFT
                },
                APIVersion::V3,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res.json::<SuccessResponse<OrderResponse>>().await.unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn cancel_multi_order(
        &self,
        cancel_multi_order_params: CancelMultiOrderRequest,
    ) -> Result<Result<SuccessResponse<CancelMultiOrderResponse>, ErrorResponse>, RateLimitExceeded>
    {
        cancel_multi_order_params.validate().unwrap();

        let res: reqwest::Response = self
            .post::<()>(
                ORDERS_CANCEL_MULTI_ORDER_ENDPOINT,
                true,
                None,
                Some(&cancel_multi_order_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<CancelMultiOrderResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn exit_all_positions(
        &self,
        exit_all_positions_params: ExitAllPositionsRequest,
    ) -> Result<Result<SuccessResponse<OrdersResponse>, ErrorResponse>, RateLimitExceeded> {
        exit_all_positions_params.validate().unwrap();

        let res: reqwest::Response = self
            .post::<()>(
                ORDERS_EXIT_ALL_POSITIONS_ENDPOINT,
                true,
                None,
                Some(&exit_all_positions_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res.json::<SuccessResponse<OrdersResponse>>().await.unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_order_details(
        &self,
        order_details_params: OrderDetailsRequest,
    ) -> Result<Result<SuccessResponse<OrderDetailsResponse>, ErrorResponse>, RateLimitExceeded>
    {
        order_details_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                ORDERS_ORDER_DETAILS_ENDPOINT,
                true,
                Some(&order_details_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 | 207 => Ok(res
                .json::<SuccessResponse<OrderDetailsResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_order_history(
        &self,
        order_history_params: OrderDetailsRequest,
    ) -> Result<Result<SuccessResponse<Vec<OrderDetailsResponse>>, ErrorResponse>, RateLimitExceeded>
    {
        order_history_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                ORDERS_ORDER_HISTORY_ENDPOINT,
                true,
                Some(&order_history_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<OrderDetailsResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_order_book(
        &self,
    ) -> Result<Result<SuccessResponse<Vec<OrderDetailsResponse>>, ErrorResponse>, RateLimitExceeded>
    {
        let res: reqwest::Response = self
            .get(
                ORDERS_ORDER_BOOK_ENDPOINT,
                true,
                None,
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<OrderDetailsResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_trades(
        &self,
    ) -> Result<Result<SuccessResponse<Vec<TradeDetailsResponse>>, ErrorResponse>, RateLimitExceeded>
    {
        let res: reqwest::Response = self
            .get(
                ORDERS_TRADES_ENDPOINT,
                true,
                None,
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<TradeDetailsResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_order_trades(
        &self,
        order_id: String,
    ) -> Result<Result<SuccessResponse<Vec<TradeDetailsResponse>>, ErrorResponse>, RateLimitExceeded>
    {
        let order_trades_params: OrderTradesRequest = OrderTradesRequest { order_id };
        order_trades_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                ORDERS_ORDER_TRADES_ENDPOINT,
                true,
                Some(&order_trades_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<TradeDetailsResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_trade_history(
        &self,
        trade_history_params: TradeHistoryRequest,
    ) -> Result<Result<SuccessResponse<Vec<TradeHistoryResponse>>, ErrorResponse>, RateLimitExceeded>
    {
        trade_history_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                ORDERS_TRADE_HISTORY_ENDPOINT,
                true,
                Some(&trade_history_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<TradeHistoryResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }
}
