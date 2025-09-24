#include "spi_bridge.h"
#include "../../common/debug_logger.h"
#include <cstring>
#include <ctime>
#include <iostream>

// MD SPI桥接器类
class MdSpiBridge : public CThostFtdcMdSpi {
private:
  MdSpiCallbacks callbacks;

public:
  MdSpiBridge(MdSpiCallbacks *cbs) { callbacks = *cbs; }

  virtual ~MdSpiBridge() {}

  // 当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用
  virtual void OnFrontConnected() override {
    CTP_DEBUG("MdSPI OnFrontConnected回调触发");
    if (callbacks.on_front_connected) {
      CTP_DEBUG("调用Rust回调 on_front_connected, user_data=%p",
                callbacks.user_data);
      callbacks.on_front_connected(callbacks.user_data);
      CTP_DEBUG("Rust回调 on_front_connected 完成");
    } else {
      CTP_DEBUG("on_front_connected回调为空，跳过调用");
      std::cout << "[MdSpiBridge] ❌ on_front_connected回调为空!" << std::endl;
    }
  }

  // 当客户端与交易后台通信连接断开时，该方法被调用
  virtual void OnFrontDisconnected(int nReason) override {
    if (callbacks.on_front_disconnected) {
      callbacks.on_front_disconnected(callbacks.user_data, nReason);
    }
  }

  // 心跳超时警告
  virtual void OnHeartBeatWarning(int nTimeLapse) override {
    if (callbacks.on_heart_beat_warning) {
      callbacks.on_heart_beat_warning(callbacks.user_data, nTimeLapse);
    }
  }

  // 登录请求响应
  virtual void OnRspUserLogin(CThostFtdcRspUserLoginField *pRspUserLogin,
                              CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                              bool bIsLast) override {
    CTP_DEBUG("MdSPI OnRspUserLogin回调触发, request_id=%d, is_last=%s",
              nRequestID, bIsLast ? "true" : "false");
    if (pRspInfo) {
      CTP_DEBUG("MD登录响应信息: ErrorID=%d, ErrorMsg=%.64s", pRspInfo->ErrorID,
                pRspInfo->ErrorMsg);
    }
    if (callbacks.on_rsp_user_login) {
      CTP_DEBUG("调用Rust回调 on_rsp_user_login");
      callbacks.on_rsp_user_login(callbacks.user_data, pRspUserLogin, pRspInfo,
                                  nRequestID, bIsLast ? 1 : 0);
      CTP_DEBUG("Rust回调 on_rsp_user_login 完成");
    }
  }

  // 登出请求响应
  virtual void OnRspUserLogout(CThostFtdcUserLogoutField *pUserLogout,
                               CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                               bool bIsLast) override {
    if (callbacks.on_rsp_user_logout) {
      callbacks.on_rsp_user_logout(callbacks.user_data, pUserLogout, pRspInfo,
                                   nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 错误应答
  virtual void OnRspError(CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                          bool bIsLast) override {
    if (callbacks.on_rsp_error) {
      callbacks.on_rsp_error(callbacks.user_data, pRspInfo, nRequestID,
                             bIsLast ? 1 : 0);
    }
  }

  // 订阅行情应答
  virtual void
  OnRspSubMarketData(CThostFtdcSpecificInstrumentField *pSpecificInstrument,
                     CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                     bool bIsLast) override {
    if (callbacks.on_rsp_sub_market_data) {
      callbacks.on_rsp_sub_market_data(callbacks.user_data, pSpecificInstrument,
                                       pRspInfo, nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 取消订阅行情应答
  virtual void
  OnRspUnSubMarketData(CThostFtdcSpecificInstrumentField *pSpecificInstrument,
                       CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                       bool bIsLast) override {
    if (callbacks.on_rsp_unsub_market_data) {
      callbacks.on_rsp_unsub_market_data(callbacks.user_data,
                                         pSpecificInstrument, pRspInfo,
                                         nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 深度行情通知
  virtual void OnRtnDepthMarketData(
      CThostFtdcDepthMarketDataField *pDepthMarketData) override {
    if (pDepthMarketData) {
      CTP_DEBUG("MdSPI OnRtnDepthMarketData回调触发, InstrumentID=%.31s, "
                "LastPrice=%.8f",
                pDepthMarketData->InstrumentID, pDepthMarketData->LastPrice);
    } else {
      CTP_DEBUG("MdSPI OnRtnDepthMarketData回调触发, 但数据为空");
    }
    if (callbacks.on_rtn_depth_market_data) {
      callbacks.on_rtn_depth_market_data(callbacks.user_data, pDepthMarketData);
    }
  }

  // 询价通知
  virtual void
  OnRtnForQuoteRsp(CThostFtdcForQuoteRspField *pForQuoteRsp) override {
    if (callbacks.on_rtn_for_quote_rsp) {
      callbacks.on_rtn_for_quote_rsp(callbacks.user_data, pForQuoteRsp);
    }
  }
};

// Trader SPI桥接器类
class TraderSpiBridge : public CThostFtdcTraderSpi {
private:
  TraderSpiCallbacks callbacks;

public:
  TraderSpiBridge(TraderSpiCallbacks *cbs) { callbacks = *cbs; }

  virtual ~TraderSpiBridge() {}

  // 当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用
  virtual void OnFrontConnected() override {
    CTP_DEBUG("TraderSPI OnFrontConnected回调触发");
    if (callbacks.on_front_connected) {
      CTP_DEBUG("调用Rust回调 on_front_connected, user_data=%p",
                callbacks.user_data);
      callbacks.on_front_connected(callbacks.user_data);
      CTP_DEBUG("Rust回调 on_front_connected 完成");
    } else {
      CTP_DEBUG("on_front_connected回调为空，跳过调用");
      std::cout << "[TraderSpiBridge] ❌ on_front_connected回调为空!"
                << std::endl;
    }
  }

  // 当客户端与交易后台通信连接断开时，该方法被调用
  virtual void OnFrontDisconnected(int nReason) override {
    if (callbacks.on_front_disconnected) {
      callbacks.on_front_disconnected(callbacks.user_data, nReason);
    }
  }

  // 心跳超时警告
  virtual void OnHeartBeatWarning(int nTimeLapse) override {
    if (callbacks.on_heart_beat_warning) {
      callbacks.on_heart_beat_warning(callbacks.user_data, nTimeLapse);
    }
  }

  // 客户端认证响应
  virtual void
  OnRspAuthenticate(CThostFtdcRspAuthenticateField *pRspAuthenticateField,
                    CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                    bool bIsLast) override {
    if (callbacks.on_rsp_authenticate) {
      callbacks.on_rsp_authenticate(callbacks.user_data, pRspAuthenticateField,
                                    pRspInfo, nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 登录请求响应
  virtual void OnRspUserLogin(CThostFtdcRspUserLoginField *pRspUserLogin,
                              CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                              bool bIsLast) override {
    CTP_DEBUG("TraderSPI OnRspUserLogin回调触发, request_id=%d, is_last=%s",
              nRequestID, bIsLast ? "true" : "false");
    if (pRspInfo) {
      CTP_DEBUG("Trader登录响应信息: ErrorID=%d, ErrorMsg=%.64s",
                pRspInfo->ErrorID, pRspInfo->ErrorMsg);
    }
    if (callbacks.on_rsp_user_login) {
      CTP_DEBUG("调用Rust回调 on_rsp_user_login");
      callbacks.on_rsp_user_login(callbacks.user_data, pRspUserLogin, pRspInfo,
                                  nRequestID, bIsLast ? 1 : 0);
      CTP_DEBUG("Rust回调 on_rsp_user_login 完成");
    }
  }

  // 登出请求响应
  virtual void OnRspUserLogout(CThostFtdcUserLogoutField *pUserLogout,
                               CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                               bool bIsLast) override {
    if (callbacks.on_rsp_user_logout) {
      callbacks.on_rsp_user_logout(callbacks.user_data, pUserLogout, pRspInfo,
                                   nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 错误应答
  virtual void OnRspError(CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                          bool bIsLast) override {
    if (callbacks.on_rsp_error) {
      callbacks.on_rsp_error(callbacks.user_data, pRspInfo, nRequestID,
                             bIsLast ? 1 : 0);
    }
  }

  // 报单录入响应
  virtual void OnRspOrderInsert(CThostFtdcInputOrderField *pInputOrder,
                                CThostFtdcRspInfoField *pRspInfo,
                                int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_order_insert) {
      callbacks.on_rsp_order_insert(callbacks.user_data, pInputOrder, pRspInfo,
                                    nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 报单操作响应
  virtual void
  OnRspOrderAction(CThostFtdcInputOrderActionField *pInputOrderAction,
                   CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                   bool bIsLast) override {
    if (callbacks.on_rsp_order_action) {
      callbacks.on_rsp_order_action(callbacks.user_data, pInputOrderAction,
                                    pRspInfo, nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 报单通知
  virtual void OnRtnOrder(CThostFtdcOrderField *pOrder) override {
    if (pOrder) {
      CTP_DEBUG("TraderSPI OnRtnOrder回调触发, OrderRef=%.13s, OrderStatus=%c",
                pOrder->OrderRef, pOrder->OrderStatus);
    } else {
      CTP_DEBUG("TraderSPI OnRtnOrder回调触发, 但数据为空");
    }
    if (callbacks.on_rtn_order) {
      callbacks.on_rtn_order(callbacks.user_data, pOrder);
    }
  }

  // 成交通知
  virtual void OnRtnTrade(CThostFtdcTradeField *pTrade) override {
    if (pTrade) {
      CTP_DEBUG(
          "TraderSPI OnRtnTrade回调触发, TradeID=%.21s, Price=%.8f, Volume=%d",
          pTrade->TradeID, pTrade->Price, pTrade->Volume);
    } else {
      CTP_DEBUG("TraderSPI OnRtnTrade回调触发, 但数据为空");
    }
    if (callbacks.on_rtn_trade) {
      callbacks.on_rtn_trade(callbacks.user_data, pTrade);
    }
  }

  // 请求查询资金账户响应
  virtual void
  OnRspQryTradingAccount(CThostFtdcTradingAccountField *pTradingAccount,
                         CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                         bool bIsLast) override {

    if (callbacks.on_rsp_qry_trading_account) {
      callbacks.on_rsp_qry_trading_account(callbacks.user_data, pTradingAccount,
                                           pRspInfo, nRequestID,
                                           bIsLast ? 1 : 0);
    }
  }

  // 请求查询投资者持仓响应
  virtual void
  OnRspQryInvestorPosition(CThostFtdcInvestorPositionField *pInvestorPosition,
                           CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                           bool bIsLast) override {
    if (callbacks.on_rsp_qry_investor_position) {
      callbacks.on_rsp_qry_investor_position(callbacks.user_data,
                                             pInvestorPosition, pRspInfo,
                                             nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 第一阶段新增回调方法

  // 报单录入错误回报
  virtual void OnErrRtnOrderInsert(CThostFtdcInputOrderField *pInputOrder,
                                   CThostFtdcRspInfoField *pRspInfo) override {
    if (callbacks.on_err_rtn_order_insert) {
      callbacks.on_err_rtn_order_insert(callbacks.user_data, pInputOrder,
                                        pRspInfo);
    }
  }

  // 报单操作错误回报
  virtual void OnErrRtnOrderAction(CThostFtdcOrderActionField *pOrderAction,
                                   CThostFtdcRspInfoField *pRspInfo) override {
    if (callbacks.on_err_rtn_order_action) {
      callbacks.on_err_rtn_order_action(callbacks.user_data, pOrderAction,
                                        pRspInfo);
    }
  }

  // 请求查询报单响应
  virtual void OnRspQryOrder(CThostFtdcOrderField *pOrder,
                             CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                             bool bIsLast) override {
    if (callbacks.on_rsp_qry_order) {
      callbacks.on_rsp_qry_order(callbacks.user_data, pOrder, pRspInfo,
                                 nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 请求查询成交响应
  virtual void OnRspQryTrade(CThostFtdcTradeField *pTrade,
                             CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                             bool bIsLast) override {
    if (callbacks.on_rsp_qry_trade) {
      callbacks.on_rsp_qry_trade(callbacks.user_data, pTrade, pRspInfo,
                                 nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 请求查询合约响应
  virtual void OnRspQryInstrument(CThostFtdcInstrumentField *pInstrument,
                                  CThostFtdcRspInfoField *pRspInfo,
                                  int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_qry_instrument) {
      callbacks.on_rsp_qry_instrument(callbacks.user_data, pInstrument,
                                      pRspInfo, nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 第二阶段新增回调方法

  // 请求查询合约保证金率响应
  virtual void OnRspQryInstrumentMarginRate(
      CThostFtdcInstrumentMarginRateField *pInstrumentMarginRate,
      CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_qry_instrument_margin_rate) {
      callbacks.on_rsp_qry_instrument_margin_rate(
          callbacks.user_data, pInstrumentMarginRate, pRspInfo, nRequestID,
          bIsLast ? 1 : 0);
    }
  }

  // 请求查询合约手续费率响应
  virtual void OnRspQryInstrumentCommissionRate(
      CThostFtdcInstrumentCommissionRateField *pInstrumentCommissionRate,
      CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_qry_instrument_commission_rate) {
      callbacks.on_rsp_qry_instrument_commission_rate(
          callbacks.user_data, pInstrumentCommissionRate, pRspInfo, nRequestID,
          bIsLast ? 1 : 0);
    }
  }

  // 请求查询交易所响应
  virtual void OnRspQryExchange(CThostFtdcExchangeField *pExchange,
                                CThostFtdcRspInfoField *pRspInfo,
                                int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_qry_exchange) {
      callbacks.on_rsp_qry_exchange(callbacks.user_data, pExchange, pRspInfo,
                                    nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 请求查询产品响应
  virtual void OnRspQryProduct(CThostFtdcProductField *pProduct,
                               CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                               bool bIsLast) override {
    if (callbacks.on_rsp_qry_product) {
      callbacks.on_rsp_qry_product(callbacks.user_data, pProduct, pRspInfo,
                                   nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 投资者结算结果确认响应
  virtual void OnRspSettlementInfoConfirm(
      CThostFtdcSettlementInfoConfirmField *pSettlementInfoConfirm,
      CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_settlement_info_confirm) {
      callbacks.on_rsp_settlement_info_confirm(callbacks.user_data,
                                               pSettlementInfoConfirm, pRspInfo,
                                               nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 预埋单录入请求响应
  virtual void OnRspParkedOrderInsert(CThostFtdcParkedOrderField *pParkedOrder,
                                      CThostFtdcRspInfoField *pRspInfo,
                                      int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_parked_order_insert) {
      callbacks.on_rsp_parked_order_insert(callbacks.user_data, pParkedOrder,
                                           pRspInfo, nRequestID,
                                           bIsLast ? 1 : 0);
    }
  }

  // 预埋撤单录入请求响应
  virtual void
  OnRspParkedOrderAction(CThostFtdcParkedOrderActionField *pParkedOrderAction,
                         CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                         bool bIsLast) override {
    if (callbacks.on_rsp_parked_order_action) {
      callbacks.on_rsp_parked_order_action(callbacks.user_data,
                                           pParkedOrderAction, pRspInfo,
                                           nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 第三阶段新增回调方法

  // 执行宣告录入请求响应
  virtual void
  OnRspExecOrderInsert(CThostFtdcInputExecOrderField *pInputExecOrder,
                       CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                       bool bIsLast) override {
    if (callbacks.on_rsp_exec_order_insert) {
      callbacks.on_rsp_exec_order_insert(callbacks.user_data, pInputExecOrder,
                                         pRspInfo, nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 执行宣告操作请求响应
  virtual void OnRspExecOrderAction(
      CThostFtdcInputExecOrderActionField *pInputExecOrderAction,
      CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_exec_order_action) {
      callbacks.on_rsp_exec_order_action(callbacks.user_data,
                                         pInputExecOrderAction, pRspInfo,
                                         nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 询价录入请求响应
  virtual void OnRspForQuoteInsert(CThostFtdcInputForQuoteField *pInputForQuote,
                                   CThostFtdcRspInfoField *pRspInfo,
                                   int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_for_quote_insert) {
      callbacks.on_rsp_for_quote_insert(callbacks.user_data, pInputForQuote,
                                        pRspInfo, nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 报价录入请求响应
  virtual void OnRspQuoteInsert(CThostFtdcInputQuoteField *pInputQuote,
                                CThostFtdcRspInfoField *pRspInfo,
                                int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_quote_insert) {
      callbacks.on_rsp_quote_insert(callbacks.user_data, pInputQuote, pRspInfo,
                                    nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 报价操作请求响应
  virtual void
  OnRspQuoteAction(CThostFtdcInputQuoteActionField *pInputQuoteAction,
                   CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                   bool bIsLast) override {
    if (callbacks.on_rsp_quote_action) {
      callbacks.on_rsp_quote_action(callbacks.user_data, pInputQuoteAction,
                                    pRspInfo, nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 批量报单操作请求响应
  virtual void OnRspBatchOrderAction(
      CThostFtdcInputBatchOrderActionField *pInputBatchOrderAction,
      CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_batch_order_action) {
      callbacks.on_rsp_batch_order_action(callbacks.user_data,
                                          pInputBatchOrderAction, pRspInfo,
                                          nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 删除预埋单响应
  virtual void
  OnRspRemoveParkedOrder(CThostFtdcRemoveParkedOrderField *pRemoveParkedOrder,
                         CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                         bool bIsLast) override {
    if (callbacks.on_rsp_remove_parked_order) {
      callbacks.on_rsp_remove_parked_order(callbacks.user_data,
                                           pRemoveParkedOrder, pRspInfo,
                                           nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 删除预埋撤单响应
  virtual void OnRspRemoveParkedOrderAction(
      CThostFtdcRemoveParkedOrderActionField *pRemoveParkedOrderAction,
      CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_remove_parked_order_action) {
      callbacks.on_rsp_remove_parked_order_action(
          callbacks.user_data, pRemoveParkedOrderAction, pRspInfo, nRequestID,
          bIsLast ? 1 : 0);
    }
  }

  // 查询最大报单数量响应
  virtual void
  OnRspQryMaxOrderVolume(CThostFtdcQryMaxOrderVolumeField *pQryMaxOrderVolume,
                         CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                         bool bIsLast) override {
    if (callbacks.on_rsp_qry_max_order_volume) {
      callbacks.on_rsp_qry_max_order_volume(callbacks.user_data,
                                            pQryMaxOrderVolume, pRspInfo,
                                            nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 请求查询行情响应
  virtual void
  OnRspQryDepthMarketData(CThostFtdcDepthMarketDataField *pDepthMarketData,
                          CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                          bool bIsLast) override {
    if (callbacks.on_rsp_qry_depth_market_data) {
      callbacks.on_rsp_qry_depth_market_data(callbacks.user_data,
                                             pDepthMarketData, pRspInfo,
                                             nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 请求查询投资者结算结果响应
  virtual void
  OnRspQrySettlementInfo(CThostFtdcSettlementInfoField *pSettlementInfo,
                         CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                         bool bIsLast) override {
    if (callbacks.on_rsp_qry_settlement_info) {
      callbacks.on_rsp_qry_settlement_info(callbacks.user_data, pSettlementInfo,
                                           pRspInfo, nRequestID,
                                           bIsLast ? 1 : 0);
    }
  }

  // 请求查询转帐银行响应
  virtual void OnRspQryTransferBank(CThostFtdcTransferBankField *pTransferBank,
                                    CThostFtdcRspInfoField *pRspInfo,
                                    int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_qry_transfer_bank) {
      callbacks.on_rsp_qry_transfer_bank(callbacks.user_data, pTransferBank,
                                         pRspInfo, nRequestID, bIsLast ? 1 : 0);
    }
  }

  // 请求查询投资者持仓明细响应
  virtual void OnRspQryInvestorPositionDetail(
      CThostFtdcInvestorPositionDetailField *pInvestorPositionDetail,
      CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast) override {
    if (callbacks.on_rsp_qry_investor_position_detail) {
      callbacks.on_rsp_qry_investor_position_detail(
          callbacks.user_data, pInvestorPositionDetail, pRspInfo, nRequestID,
          bIsLast ? 1 : 0);
    }
  }

  // 请求查询客户通知响应
  virtual void OnRspQryNotice(CThostFtdcNoticeField *pNotice,
                              CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                              bool bIsLast) override {
    if (callbacks.on_rsp_qry_notice) {
      callbacks.on_rsp_qry_notice(callbacks.user_data, pNotice, pRspInfo,
                                  nRequestID, bIsLast ? 1 : 0);
    }
  }
};

extern "C" {
void *CreateMdSpiBridge(MdSpiCallbacks *callbacks) {
  if (callbacks) {
    // MD回调结构体有效
  } else {
    std::cout << "[Bridge] ❌ MD回调结构体为空!" << std::endl;
  }
  return new MdSpiBridge(callbacks);
}

void DestroyMdSpiBridge(void *spi_bridge) {
  if (spi_bridge) {
    delete static_cast<MdSpiBridge *>(spi_bridge);
  }
}

void *CreateTraderSpiBridge(TraderSpiCallbacks *callbacks) {
  if (callbacks) {
    // Trader回调结构体有效
  } else {
    std::cout << "[Bridge] ❌ Trader回调结构体为空!" << std::endl;
    std::cout.flush();
  }
  TraderSpiBridge *bridge = new TraderSpiBridge(callbacks);
  return bridge;
}

void DestroyTraderSpiBridge(void *spi_bridge) {
  if (spi_bridge) {
    delete static_cast<TraderSpiBridge *>(spi_bridge);
  }
}
}
