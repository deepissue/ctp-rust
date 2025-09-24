#ifndef SPI_BRIDGE_H
#define SPI_BRIDGE_H

#include "../include/ThostFtdcMdApi.h"
#include "../include/ThostFtdcTraderApi.h"

#ifdef __cplusplus
extern "C" {
#endif

// Rust回调函数类型定义
typedef void (*OnFrontConnectedCallback)(void *user_data);
typedef void (*OnFrontDisconnectedCallback)(void *user_data, int reason);
typedef void (*OnHeartBeatWarningCallback)(void *user_data, int time_lapse);
typedef void (*OnRspUserLoginCallback)(void *user_data, void *user_login,
                                       void *rsp_info, int request_id,
                                       int is_last);
typedef void (*OnRspUserLogoutCallback)(void *user_data, void *user_logout,
                                        void *rsp_info, int request_id,
                                        int is_last);
typedef void (*OnRspErrorCallback)(void *user_data, void *rsp_info,
                                   int request_id, int is_last);
typedef void (*OnRspSubMarketDataCallback)(void *user_data,
                                           void *specific_instrument,
                                           void *rsp_info, int request_id,
                                           int is_last);
typedef void (*OnRspUnSubMarketDataCallback)(void *user_data,
                                             void *specific_instrument,
                                             void *rsp_info, int request_id,
                                             int is_last);
typedef void (*OnRtnDepthMarketDataCallback)(void *user_data,
                                             void *market_data);
typedef void (*OnRtnForQuoteRspCallback)(void *user_data, void *for_quote_rsp);

// 交易API回调函数类型定义
typedef void (*OnRspAuthenticateCallback)(void *user_data,
                                          void *rsp_authenticate,
                                          void *rsp_info, int request_id,
                                          int is_last);
typedef void (*OnRspOrderInsertCallback)(void *user_data, void *input_order,
                                         void *rsp_info, int request_id,
                                         int is_last);
typedef void (*OnRspOrderActionCallback)(void *user_data,
                                         void *input_order_action,
                                         void *rsp_info, int request_id,
                                         int is_last);
typedef void (*OnRtnOrderCallback)(void *user_data, void *order);
typedef void (*OnRtnTradeCallback)(void *user_data, void *trade);
typedef void (*OnRspQryTradingAccountCallback)(void *user_data,
                                               void *trading_account,
                                               void *rsp_info, int request_id,
                                               int is_last);
typedef void (*OnRspQryInvestorPositionCallback)(void *user_data,
                                                 void *investor_position,
                                                 void *rsp_info, int request_id,
                                                 int is_last);

// 第一阶段新增回调函数类型定义
typedef void (*OnErrRtnOrderInsertCallback)(void *user_data, void *input_order,
                                            void *rsp_info);
typedef void (*OnErrRtnOrderActionCallback)(void *user_data, void *order_action,
                                            void *rsp_info);
typedef void (*OnRspQryOrderCallback)(void *user_data, void *order,
                                      void *rsp_info, int request_id,
                                      int is_last);
typedef void (*OnRspQryTradeCallback)(void *user_data, void *trade,
                                      void *rsp_info, int request_id,
                                      int is_last);
typedef void (*OnRspQryInstrumentCallback)(void *user_data, void *instrument,
                                           void *rsp_info, int request_id,
                                           int is_last);

// 第二阶段新增回调函数类型定义
typedef void (*OnRspQryInstrumentMarginRateCallback)(void *user_data,
                                                     void *margin_rate,
                                                     void *rsp_info,
                                                     int request_id,
                                                     int is_last);
typedef void (*OnRspQryInstrumentCommissionRateCallback)(void *user_data,
                                                         void *commission_rate,
                                                         void *rsp_info,
                                                         int request_id,
                                                         int is_last);
typedef void (*OnRspQryExchangeCallback)(void *user_data, void *exchange,
                                         void *rsp_info, int request_id,
                                         int is_last);
typedef void (*OnRspQryProductCallback)(void *user_data, void *product,
                                        void *rsp_info, int request_id,
                                        int is_last);
typedef void (*OnRspSettlementInfoConfirmCallback)(
    void *user_data, void *settlement_info_confirm, void *rsp_info,
    int request_id, int is_last);
typedef void (*OnRspParkedOrderInsertCallback)(void *user_data,
                                               void *parked_order,
                                               void *rsp_info, int request_id,
                                               int is_last);
typedef void (*OnRspParkedOrderActionCallback)(void *user_data,
                                               void *parked_order_action,
                                               void *rsp_info, int request_id,
                                               int is_last);

// 第三阶段新增回调函数类型定义
typedef void (*OnRspExecOrderInsertCallback)(void *user_data,
                                             void *input_exec_order,
                                             void *rsp_info, int request_id,
                                             int is_last);
typedef void (*OnRspExecOrderActionCallback)(void *user_data,
                                             void *input_exec_order_action,
                                             void *rsp_info, int request_id,
                                             int is_last);
typedef void (*OnRspForQuoteInsertCallback)(void *user_data,
                                            void *input_for_quote,
                                            void *rsp_info, int request_id,
                                            int is_last);
typedef void (*OnRspQuoteInsertCallback)(void *user_data, void *input_quote,
                                         void *rsp_info, int request_id,
                                         int is_last);
typedef void (*OnRspQuoteActionCallback)(void *user_data,
                                         void *input_quote_action,
                                         void *rsp_info, int request_id,
                                         int is_last);
typedef void (*OnRspBatchOrderActionCallback)(void *user_data,
                                              void *input_batch_order_action,
                                              void *rsp_info, int request_id,
                                              int is_last);
typedef void (*OnRspRemoveParkedOrderCallback)(void *user_data,
                                               void *remove_parked_order,
                                               void *rsp_info, int request_id,
                                               int is_last);
typedef void (*OnRspRemoveParkedOrderActionCallback)(
    void *user_data, void *remove_parked_order_action, void *rsp_info,
    int request_id, int is_last);
typedef void (*OnRspQryMaxOrderVolumeCallback)(void *user_data,
                                               void *qry_max_order_volume,
                                               void *rsp_info, int request_id,
                                               int is_last);
typedef void (*OnRspQryDepthMarketDataCallback)(void *user_data,
                                                void *depth_market_data,
                                                void *rsp_info, int request_id,
                                                int is_last);
typedef void (*OnRspQrySettlementInfoCallback)(void *user_data,
                                               void *settlement_info,
                                               void *rsp_info, int request_id,
                                               int is_last);
typedef void (*OnRspQryTransferBankCallback)(void *user_data,
                                             void *transfer_bank,
                                             void *rsp_info, int request_id,
                                             int is_last);
typedef void (*OnRspQryInvestorPositionDetailCallback)(
    void *user_data, void *investor_position_detail, void *rsp_info,
    int request_id, int is_last);
typedef void (*OnRspQryNoticeCallback)(void *user_data, void *notice,
                                       void *rsp_info, int request_id,
                                       int is_last);

// 行情SPI回调结构体
typedef struct {
  void *user_data;
  OnFrontConnectedCallback on_front_connected;
  OnFrontDisconnectedCallback on_front_disconnected;
  OnHeartBeatWarningCallback on_heart_beat_warning;
  OnRspUserLoginCallback on_rsp_user_login;
  OnRspUserLogoutCallback on_rsp_user_logout;
  OnRspErrorCallback on_rsp_error;
  OnRspSubMarketDataCallback on_rsp_sub_market_data;
  OnRspUnSubMarketDataCallback on_rsp_unsub_market_data;
  OnRtnDepthMarketDataCallback on_rtn_depth_market_data;
  OnRtnForQuoteRspCallback on_rtn_for_quote_rsp;
} MdSpiCallbacks;

// 交易SPI回调结构体
typedef struct {
  void *user_data;
  OnFrontConnectedCallback on_front_connected;
  OnFrontDisconnectedCallback on_front_disconnected;
  OnHeartBeatWarningCallback on_heart_beat_warning;
  OnRspAuthenticateCallback on_rsp_authenticate;
  OnRspUserLoginCallback on_rsp_user_login;
  OnRspUserLogoutCallback on_rsp_user_logout;
  OnRspErrorCallback on_rsp_error;
  OnRspOrderInsertCallback on_rsp_order_insert;
  OnRspOrderActionCallback on_rsp_order_action;
  OnRtnOrderCallback on_rtn_order;
  OnRtnTradeCallback on_rtn_trade;
  OnRspQryTradingAccountCallback on_rsp_qry_trading_account;
  OnRspQryInvestorPositionCallback on_rsp_qry_investor_position;

  // 第一阶段新增回调
  OnErrRtnOrderInsertCallback on_err_rtn_order_insert;
  OnErrRtnOrderActionCallback on_err_rtn_order_action;
  OnRspQryOrderCallback on_rsp_qry_order;
  OnRspQryTradeCallback on_rsp_qry_trade;
  OnRspQryInstrumentCallback on_rsp_qry_instrument;

  // 第二阶段新增回调
  OnRspQryInstrumentMarginRateCallback on_rsp_qry_instrument_margin_rate;
  OnRspQryInstrumentCommissionRateCallback
      on_rsp_qry_instrument_commission_rate;
  OnRspQryExchangeCallback on_rsp_qry_exchange;
  OnRspQryProductCallback on_rsp_qry_product;
  OnRspSettlementInfoConfirmCallback on_rsp_settlement_info_confirm;
  OnRspParkedOrderInsertCallback on_rsp_parked_order_insert;
  OnRspParkedOrderActionCallback on_rsp_parked_order_action;

  // 第三阶段新增回调
  OnRspExecOrderInsertCallback on_rsp_exec_order_insert;
  OnRspExecOrderActionCallback on_rsp_exec_order_action;
  OnRspForQuoteInsertCallback on_rsp_for_quote_insert;
  OnRspQuoteInsertCallback on_rsp_quote_insert;
  OnRspQuoteActionCallback on_rsp_quote_action;
  OnRspBatchOrderActionCallback on_rsp_batch_order_action;
  OnRspRemoveParkedOrderCallback on_rsp_remove_parked_order;
  OnRspRemoveParkedOrderActionCallback on_rsp_remove_parked_order_action;
  OnRspQryMaxOrderVolumeCallback on_rsp_qry_max_order_volume;
  OnRspQryDepthMarketDataCallback on_rsp_qry_depth_market_data;
  OnRspQrySettlementInfoCallback on_rsp_qry_settlement_info;
  OnRspQryTransferBankCallback on_rsp_qry_transfer_bank;
  OnRspQryInvestorPositionDetailCallback on_rsp_qry_investor_position_detail;
  OnRspQryNoticeCallback on_rsp_qry_notice;
} TraderSpiCallbacks;

// 创建行情SPI桥接器
void *CreateMdSpiBridge(MdSpiCallbacks *callbacks);

// 销毁行情SPI桥接器
void DestroyMdSpiBridge(void *spi_bridge);

// 创建交易SPI桥接器
void *CreateTraderSpiBridge(TraderSpiCallbacks *callbacks);

// 销毁交易SPI桥接器
void DestroyTraderSpiBridge(void *spi_bridge);

#ifdef __cplusplus
}
#endif

#endif // SPI_BRIDGE_H
