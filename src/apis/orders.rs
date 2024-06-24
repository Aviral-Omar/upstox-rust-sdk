use {
    crate::{
        client::ApiClient,
        constants::{
            ORDERS_MODIFY_ORDER_ENDPOINT, ORDERS_ORDER_BOOK_ENDPOINT,
            ORDERS_ORDER_DETAILS_ENDPOINT, ORDERS_ORDER_TRADES_ENDPOINT,
            ORDERS_PLACE_ORDER_ENDPOINT, ORDERS_TRADES_ENDPOINT, ORDERS_TRADE_HISTORY_ENDPOINT,
        },
        models::{
            error_response::ErrorResponse,
            orders::{
                cancel_order_request::CancelOrderRequest, modify_order_request::ModifyOrderRequest,
                order_details_request::OrderDetailsRequest,
                order_details_response::OrderDetailsResponse, order_response::OrderResponse,
                order_trades_request::OrderTradesRequest, place_order_request::PlaceOrderRequest,
                trade_details_response::TradeDetailsResponse,
                trade_history_request::TradeHistoryRequest,
                trade_history_response::TradeHistoryResponse,
            },
            success_response::SuccessResponse,
        },
        utils::ToKeyValueTuples,
    },
    serde_valid::Validate,
};

impl ApiClient {
    pub async fn place_order(
        &self,
        place_order_body: PlaceOrderRequest,
    ) -> Result<SuccessResponse<OrderResponse>, ErrorResponse> {
        place_order_body.validate().unwrap();
        let res: reqwest::Response = self
            .post(
                ORDERS_PLACE_ORDER_ENDPOINT,
                true,
                Some(&place_order_body.to_key_value_tuples_vec()),
                None,
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res.json::<SuccessResponse<OrderResponse>>().await.unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn modify_order(
        &self,
        modify_order_body: ModifyOrderRequest,
    ) -> Result<SuccessResponse<OrderResponse>, ErrorResponse> {
        modify_order_body.validate().unwrap();
        let res: reqwest::Response = self
            .put(
                ORDERS_MODIFY_ORDER_ENDPOINT,
                true,
                Some(&modify_order_body.to_key_value_tuples_vec()),
                None,
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res.json::<SuccessResponse<OrderResponse>>().await.unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn cancel_order(
        &self,
        order_id: String,
    ) -> Result<SuccessResponse<OrderResponse>, ErrorResponse> {
        let cancel_order_params: CancelOrderRequest = CancelOrderRequest { order_id };
        cancel_order_params.validate().unwrap();

        let res: reqwest::Response = self
            .delete(
                ORDERS_MODIFY_ORDER_ENDPOINT,
                true,
                Some(&cancel_order_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res.json::<SuccessResponse<OrderResponse>>().await.unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_order_details(
        &self,
        order_details_params: OrderDetailsRequest,
    ) -> Result<SuccessResponse<OrderDetailsResponse>, ErrorResponse> {
        order_details_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                ORDERS_ORDER_DETAILS_ENDPOINT,
                true,
                Some(&order_details_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<OrderDetailsResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_order_history(
        &self,
        order_history_params: OrderDetailsRequest,
    ) -> Result<SuccessResponse<Vec<OrderDetailsResponse>>, ErrorResponse> {
        order_history_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                ORDERS_ORDER_DETAILS_ENDPOINT,
                true,
                Some(&order_history_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<OrderDetailsResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_order_book(
        &self,
    ) -> Result<SuccessResponse<Vec<OrderDetailsResponse>>, ErrorResponse> {
        let res: reqwest::Response = self.get(ORDERS_ORDER_BOOK_ENDPOINT, true, None).await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<OrderDetailsResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_trades(
        &self,
    ) -> Result<SuccessResponse<Vec<TradeDetailsResponse>>, ErrorResponse> {
        let res: reqwest::Response = self.get(ORDERS_TRADES_ENDPOINT, true, None).await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<TradeDetailsResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_order_trades(
        &self,
        order_id: String,
    ) -> Result<SuccessResponse<Vec<TradeDetailsResponse>>, ErrorResponse> {
        let order_trades_params: OrderTradesRequest = OrderTradesRequest { order_id };
        order_trades_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                ORDERS_ORDER_TRADES_ENDPOINT,
                true,
                Some(&order_trades_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<TradeDetailsResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_trade_history(
        &self,
        trade_history_params: TradeHistoryRequest,
    ) -> Result<SuccessResponse<Vec<TradeHistoryResponse>>, ErrorResponse> {
        trade_history_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                ORDERS_TRADE_HISTORY_ENDPOINT,
                true,
                Some(&trade_history_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<TradeHistoryResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }
}
