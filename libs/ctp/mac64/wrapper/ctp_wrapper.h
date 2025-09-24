#ifndef CTP_WRAPPER_H
#define CTP_WRAPPER_H

#include "spi_bridge.h"
#include "../../common/debug_logger.h"

#ifdef __cplusplus
extern "C" {
#endif

// C wrapper for CTP MD API
void *CThostFtdcMdApi_CreateFtdcMdApi(const char *pszFlowPath, int bIsUsingUdp,
                                      int bIsMulticast, int bIsProductionMode);
void CThostFtdcMdApi_Release(void *api);
void CThostFtdcMdApi_Init(void *api);
int CThostFtdcMdApi_Join(void *api);
const char *CThostFtdcMdApi_GetTradingDay(void *api);
void CThostFtdcMdApi_RegisterFront(void *api, const char *pszFrontAddress);
void CThostFtdcMdApi_RegisterNameServer(void *api, const char *pszNsAddress);
void CThostFtdcMdApi_RegisterFensUserInfo(void *api, void *pFensUserInfo);
void CThostFtdcMdApi_RegisterSpi(void *api, void *pSpi);
int CThostFtdcMdApi_ReqUserLogin(void *api, void *pReqUserLoginField,
                                 int nRequestID);
int CThostFtdcMdApi_ReqUserLogout(void *api, void *pUserLogout, int nRequestID);
int CThostFtdcMdApi_SubscribeMarketData(void *api, char *ppInstrumentID[],
                                        int nCount);
int CThostFtdcMdApi_UnSubscribeMarketData(void *api, char *ppInstrumentID[],
                                          int nCount);
int CThostFtdcMdApi_SubscribeForQuoteRsp(void *api, char *ppInstrumentID[],
                                         int nCount);
int CThostFtdcMdApi_UnSubscribeForQuoteRsp(void *api, char *ppInstrumentID[],
                                           int nCount);
const char *CThostFtdcMdApi_GetApiVersion();

// C wrapper for CTP Trader API
void *CThostFtdcTraderApi_CreateFtdcTraderApi(const char *pszFlowPath,
                                              int bIsProductionMode);
void CThostFtdcTraderApi_Release(void *api);
void CThostFtdcTraderApi_Init(void *api);
int CThostFtdcTraderApi_Join(void *api);
const char *CThostFtdcTraderApi_GetTradingDay(void *api);
void CThostFtdcTraderApi_RegisterFront(void *api, const char *pszFrontAddress);
void CThostFtdcTraderApi_RegisterNameServer(void *api,
                                            const char *pszNsAddress);
void CThostFtdcTraderApi_GetFrontInfo(void *api, void *pFrontInfo);
void CThostFtdcTraderApi_RegisterFensUserInfo(void *api, void *pFensUserInfo);
void CThostFtdcTraderApi_RegisterSpi(void *api, void *pSpi);
int CThostFtdcTraderApi_ReqAuthenticate(void *api, void *pReqAuthenticateField,
                                        int nRequestID);
int CThostFtdcTraderApi_RegisterUserSystemInfo(void *api,
                                               void *pUserSystemInfo);
int CThostFtdcTraderApi_SubmitUserSystemInfo(void *api, void *pUserSystemInfo);
int CThostFtdcTraderApi_RegisterWechatUserSystemInfo(void *api,
                                                     void *pUserSystemInfo);
int CThostFtdcTraderApi_SubmitWechatUserSystemInfo(void *api,
                                                   void *pUserSystemInfo);
int CThostFtdcTraderApi_ReqUserLogin(void *api, void *pReqUserLoginField,
                                     int nRequestID);
int CThostFtdcTraderApi_ReqUserLogout(void *api, void *pUserLogout,
                                      int nRequestID);
int CThostFtdcTraderApi_ReqUserPasswordUpdate(void *api,
                                              void *pUserPasswordUpdate,
                                              int nRequestID);
int CThostFtdcTraderApi_ReqTradingAccountPasswordUpdate(
    void *api, void *pTradingAccountPasswordUpdate, int nRequestID);
int CThostFtdcTraderApi_ReqUserAuthMethod(void *api, void *pReqUserAuthMethod,
                                          int nRequestID);
int CThostFtdcTraderApi_ReqGenUserCaptcha(void *api, void *pReqGenUserCaptcha,
                                          int nRequestID);
int CThostFtdcTraderApi_ReqGenUserText(void *api, void *pReqGenUserText,
                                       int nRequestID);
int CThostFtdcTraderApi_ReqUserLoginWithCaptcha(void *api,
                                                void *pReqUserLoginWithCaptcha,
                                                int nRequestID);
int CThostFtdcTraderApi_ReqUserLoginWithText(void *api,
                                             void *pReqUserLoginWithText,
                                             int nRequestID);
int CThostFtdcTraderApi_ReqUserLoginWithOTP(void *api,
                                            void *pReqUserLoginWithOTP,
                                            int nRequestID);
int CThostFtdcTraderApi_ReqOrderInsert(void *api, void *pInputOrder,
                                       int nRequestID);
int CThostFtdcTraderApi_ReqParkedOrderInsert(void *api, void *pParkedOrder,
                                             int nRequestID);
int CThostFtdcTraderApi_ReqParkedOrderAction(void *api,
                                             void *pParkedOrderAction,
                                             int nRequestID);
int CThostFtdcTraderApi_ReqOrderAction(void *api, void *pInputOrderAction,
                                       int nRequestID);
int CThostFtdcTraderApi_ReqQryMaxOrderVolume(void *api,
                                             void *pQryMaxOrderVolume,
                                             int nRequestID);
int CThostFtdcTraderApi_ReqSettlementInfoConfirm(void *api,
                                                 void *pSettlementInfoConfirm,
                                                 int nRequestID);
int CThostFtdcTraderApi_ReqRemoveParkedOrder(void *api,
                                             void *pRemoveParkedOrder,
                                             int nRequestID);
int CThostFtdcTraderApi_ReqRemoveParkedOrderAction(
    void *api, void *pRemoveParkedOrderAction, int nRequestID);
int CThostFtdcTraderApi_ReqExecOrderInsert(void *api, void *pInputExecOrder,
                                           int nRequestID);
int CThostFtdcTraderApi_ReqExecOrderAction(void *api,
                                           void *pInputExecOrderAction,
                                           int nRequestID);
int CThostFtdcTraderApi_ReqForQuoteInsert(void *api, void *pInputForQuote,
                                          int nRequestID);
int CThostFtdcTraderApi_ReqQuoteInsert(void *api, void *pInputQuote,
                                       int nRequestID);
int CThostFtdcTraderApi_ReqQuoteAction(void *api, void *pInputQuoteAction,
                                       int nRequestID);
int CThostFtdcTraderApi_ReqBatchOrderAction(void *api,
                                            void *pInputBatchOrderAction,
                                            int nRequestID);
int CThostFtdcTraderApi_ReqOptionSelfCloseInsert(void *api,
                                                 void *pInputOptionSelfClose,
                                                 int nRequestID);
int CThostFtdcTraderApi_ReqOptionSelfCloseAction(
    void *api, void *pInputOptionSelfCloseAction, int nRequestID);
int CThostFtdcTraderApi_ReqCombActionInsert(void *api, void *pInputCombAction,
                                            int nRequestID);
int CThostFtdcTraderApi_ReqQryOrder(void *api, void *pQryOrder, int nRequestID);
int CThostFtdcTraderApi_ReqQryTrade(void *api, void *pQryTrade, int nRequestID);
int CThostFtdcTraderApi_ReqQryInvestorPosition(void *api,
                                               void *pQryInvestorPosition,
                                               int nRequestID);
int CThostFtdcTraderApi_ReqQryTradingAccount(void *api,
                                             void *pQryTradingAccount,
                                             int nRequestID);
int CThostFtdcTraderApi_ReqQryInvestor(void *api, void *pQryInvestor,
                                       int nRequestID);
int CThostFtdcTraderApi_ReqQryTradingCode(void *api, void *pQryTradingCode,
                                          int nRequestID);
int CThostFtdcTraderApi_ReqQryInstrumentMarginRate(
    void *api, void *pQryInstrumentMarginRate, int nRequestID);
int CThostFtdcTraderApi_ReqQryInstrumentCommissionRate(
    void *api, void *pQryInstrumentCommissionRate, int nRequestID);
int CThostFtdcTraderApi_ReqQryExchange(void *api, void *pQryExchange,
                                       int nRequestID);
int CThostFtdcTraderApi_ReqQryProduct(void *api, void *pQryProduct,
                                      int nRequestID);
int CThostFtdcTraderApi_ReqQryInstrument(void *api, void *pQryInstrument,
                                         int nRequestID);
int CThostFtdcTraderApi_ReqQryDepthMarketData(void *api,
                                              void *pQryDepthMarketData,
                                              int nRequestID);
int CThostFtdcTraderApi_ReqQrySettlementInfo(void *api,
                                             void *pQrySettlementInfo,
                                             int nRequestID);
const char *CThostFtdcTraderApi_GetApiVersion();

// Debug logging functions
void CTP_InitializeDebugLogging(const CtpLogConfig* config);
void CTP_CleanupDebugLogging();

#ifdef __cplusplus
}
#endif

#endif // CTP_WRAPPER_H
