#include "ctp_wrapper.h"
#include "../include/ThostFtdcMdApi.h"
#include "../include/ThostFtdcTraderApi.h"
#include "../../common/debug_logger.h"
#include <iostream>
#include <stdio.h>
// MD API wrappers
extern "C" {
void *CThostFtdcMdApi_CreateFtdcMdApi(const char *pszFlowPath, int bIsUsingUdp,
                                      int bIsMulticast, int bIsProductionMode) {
  CTP_DEBUG("创建MD API, flow_path=%s, udp=%d, multicast=%d, production=%d", 
            pszFlowPath ? pszFlowPath : "null", bIsUsingUdp, bIsMulticast, bIsProductionMode);
  void* api = CThostFtdcMdApi::CreateFtdcMdApi(
      pszFlowPath, bIsUsingUdp != 0, bIsMulticast != 0, bIsProductionMode != 0);
  CTP_DEBUG("MD API创建完成, api指针=%p", api);
  return api;
}

void CThostFtdcMdApi_Release(void *api) {
  if (api) {
    static_cast<CThostFtdcMdApi *>(api)->Release();
  }
}

void CThostFtdcMdApi_Init(void *api) {
  CTP_DEBUG("MD API初始化开始, api=%p", api);
  if (api) {
    static_cast<CThostFtdcMdApi *>(api)->Init();
    CTP_DEBUG("MD API初始化完成, api=%p", api);
  } else {
    CTP_DEBUG("MD API初始化失败: API实例为空");
  }
}

int CThostFtdcMdApi_Join(void *api) {
  if (api) {
    return static_cast<CThostFtdcMdApi *>(api)->Join();
  }
  return -1;
}

const char *CThostFtdcMdApi_GetTradingDay(void *api) {
  if (api) {
    return static_cast<CThostFtdcMdApi *>(api)->GetTradingDay();
  }
  return nullptr;
}

void CThostFtdcMdApi_RegisterFront(void *api, const char *pszFrontAddress) {
  if (api) {
    static_cast<CThostFtdcMdApi *>(api)->RegisterFront(
        const_cast<char *>(pszFrontAddress));
  }
}

void CThostFtdcMdApi_RegisterNameServer(void *api, const char *pszNsAddress) {
  if (api) {
    static_cast<CThostFtdcMdApi *>(api)->RegisterNameServer(
        const_cast<char *>(pszNsAddress));
  }
}

void CThostFtdcMdApi_RegisterFensUserInfo(void *api, void *pFensUserInfo) {
  if (api) {
    static_cast<CThostFtdcMdApi *>(api)->RegisterFensUserInfo(
        static_cast<CThostFtdcFensUserInfoField *>(pFensUserInfo));
  }
}

void CThostFtdcMdApi_RegisterSpi(void *api, void *pSpi) {
  if (api) {
    static_cast<CThostFtdcMdApi *>(api)->RegisterSpi(
        static_cast<CThostFtdcMdSpi *>(pSpi));
  }
}

int CThostFtdcMdApi_ReqUserLogin(void *api, void *pReqUserLoginField,
                                 int nRequestID) {
  CTP_DEBUG("MD用户登录请求开始, api=%p, request_id=%d", api, nRequestID);
  if (api) {
    int result = static_cast<CThostFtdcMdApi *>(api)->ReqUserLogin(
        static_cast<CThostFtdcReqUserLoginField *>(pReqUserLoginField),
        nRequestID);
    CTP_DEBUG("MD用户登录请求完成, request_id=%d, 返回值=%d", nRequestID, result);
    return result;
  }
  CTP_DEBUG("MD用户登录请求失败: API实例为空, request_id=%d", nRequestID);
  return -1;
}

int CThostFtdcMdApi_ReqUserLogout(void *api, void *pUserLogout,
                                  int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcMdApi *>(api)->ReqUserLogout(
        static_cast<CThostFtdcUserLogoutField *>(pUserLogout), nRequestID);
  }
  return -1;
}

int CThostFtdcMdApi_SubscribeMarketData(void *api, char *ppInstrumentID[],
                                        int nCount) {
  if (api) {
    return static_cast<CThostFtdcMdApi *>(api)->SubscribeMarketData(
        ppInstrumentID, nCount);
  }
  return -1;
}

int CThostFtdcMdApi_UnSubscribeMarketData(void *api, char *ppInstrumentID[],
                                          int nCount) {
  if (api) {
    return static_cast<CThostFtdcMdApi *>(api)->UnSubscribeMarketData(
        ppInstrumentID, nCount);
  }
  return -1;
}

int CThostFtdcMdApi_SubscribeForQuoteRsp(void *api, char *ppInstrumentID[],
                                         int nCount) {
  if (api) {
    return static_cast<CThostFtdcMdApi *>(api)->SubscribeForQuoteRsp(
        ppInstrumentID, nCount);
  }
  return -1;
}

int CThostFtdcMdApi_UnSubscribeForQuoteRsp(void *api, char *ppInstrumentID[],
                                           int nCount) {
  if (api) {
    return static_cast<CThostFtdcMdApi *>(api)->UnSubscribeForQuoteRsp(
        ppInstrumentID, nCount);
  }
  return -1;
}

const char *CThostFtdcMdApi_GetApiVersion() {
  return CThostFtdcMdApi::GetApiVersion();
}

// Trader API wrappers
void *CThostFtdcTraderApi_CreateFtdcTraderApi(const char *pszFlowPath,
                                              int bIsProductionMode) {
  CTP_DEBUG("创建Trader API, flow_path=%s, production=%d", 
            pszFlowPath ? pszFlowPath : "null", bIsProductionMode);
  void *api = CThostFtdcTraderApi::CreateFtdcTraderApi(pszFlowPath,
                                                       bIsProductionMode != 0);
  CTP_DEBUG("Trader API创建完成, api指针=%p", api);
  fflush(stdout);
  return api;
}

void CThostFtdcTraderApi_Release(void *api) {
  if (api) {
    static_cast<CThostFtdcTraderApi *>(api)->Release();
  }
}

void CThostFtdcTraderApi_Init(void *api) {
  CTP_DEBUG("Trader API初始化开始, api=%p", api);
  if (api) {
    static_cast<CThostFtdcTraderApi *>(api)->Init();
    CTP_DEBUG("Trader API初始化完成, api=%p", api);
  } else {
    CTP_DEBUG("Trader API初始化失败: API实例为空");
    std::cout << "[CTP_API] ❌ API实例为空!" << std::endl;
    std::cout.flush();
  }
}

int CThostFtdcTraderApi_Join(void *api) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->Join();
  }
  return -1;
}

const char *CThostFtdcTraderApi_GetTradingDay(void *api) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->GetTradingDay();
  }
  return nullptr;
}

void CThostFtdcTraderApi_RegisterFront(void *api, const char *pszFrontAddress) {
  CTP_DEBUG("注册前置机, api=%p, front_address=%s", 
            api, pszFrontAddress ? pszFrontAddress : "null");
  if (api) {
    static_cast<CThostFtdcTraderApi *>(api)->RegisterFront(
        const_cast<char *>(pszFrontAddress));
    CTP_DEBUG("注册前置机完成, api=%p", api);
  } else {
    CTP_DEBUG("注册前置机失败: API实例为空");
    std::cout << "[CTP_API] ❌ API实例为空!" << std::endl;
  }
}

void CThostFtdcTraderApi_RegisterNameServer(void *api,
                                            const char *pszNsAddress) {
  if (api) {
    static_cast<CThostFtdcTraderApi *>(api)->RegisterNameServer(
        const_cast<char *>(pszNsAddress));
  }
}

void CThostFtdcTraderApi_GetFrontInfo(void *api, void *pFrontInfo) {
  if (api) {
    static_cast<CThostFtdcTraderApi *>(api)->GetFrontInfo(
        static_cast<CThostFtdcFrontInfoField *>(pFrontInfo));
  }
}

void CThostFtdcTraderApi_RegisterFensUserInfo(void *api, void *pFensUserInfo) {
  if (api) {
    static_cast<CThostFtdcTraderApi *>(api)->RegisterFensUserInfo(
        static_cast<CThostFtdcFensUserInfoField *>(pFensUserInfo));
  }
}

void CThostFtdcTraderApi_RegisterSpi(void *api, void *pSpi) {
  if (api) {
    static_cast<CThostFtdcTraderApi *>(api)->RegisterSpi(
        static_cast<CThostFtdcTraderSpi *>(pSpi));
  } else {
    std::cout << "[CTP_API] ❌ API实例为空!" << std::endl;
    std::cout.flush();
  }
}

int CThostFtdcTraderApi_ReqAuthenticate(void *api, void *pReqAuthenticateField,
                                        int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqAuthenticate(
        static_cast<CThostFtdcReqAuthenticateField *>(pReqAuthenticateField),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_RegisterUserSystemInfo(void *api,
                                               void *pUserSystemInfo) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->RegisterUserSystemInfo(
        static_cast<CThostFtdcUserSystemInfoField *>(pUserSystemInfo));
  }
  return -1;
}

int CThostFtdcTraderApi_SubmitUserSystemInfo(void *api, void *pUserSystemInfo) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->SubmitUserSystemInfo(
        static_cast<CThostFtdcUserSystemInfoField *>(pUserSystemInfo));
  }
  return -1;
}

int CThostFtdcTraderApi_RegisterWechatUserSystemInfo(void *api,
                                                     void *pUserSystemInfo) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)
        ->RegisterWechatUserSystemInfo(
            static_cast<CThostFtdcWechatUserSystemInfoField *>(
                pUserSystemInfo));
  }
  return -1;
}

int CThostFtdcTraderApi_SubmitWechatUserSystemInfo(void *api,
                                                   void *pUserSystemInfo) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->SubmitWechatUserSystemInfo(
        static_cast<CThostFtdcWechatUserSystemInfoField *>(pUserSystemInfo));
  }
  return -1;
}

int CThostFtdcTraderApi_ReqUserLogin(void *api, void *pReqUserLoginField,
                                     int nRequestID) {
  CTP_DEBUG("Trader用户登录请求开始, api=%p, request_id=%d", api, nRequestID);
  if (api) {
    int result = static_cast<CThostFtdcTraderApi *>(api)->ReqUserLogin(
        static_cast<CThostFtdcReqUserLoginField *>(pReqUserLoginField),
        nRequestID);
    CTP_DEBUG("Trader用户登录请求完成, request_id=%d, 返回值=%d", nRequestID, result);
    return result;
  }
  CTP_DEBUG("Trader用户登录请求失败: API实例为空, request_id=%d", nRequestID);
  return -1;
}

int CThostFtdcTraderApi_ReqUserLogout(void *api, void *pUserLogout,
                                      int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqUserLogout(
        static_cast<CThostFtdcUserLogoutField *>(pUserLogout), nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqUserPasswordUpdate(void *api,
                                              void *pUserPasswordUpdate,
                                              int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqUserPasswordUpdate(
        static_cast<CThostFtdcUserPasswordUpdateField *>(pUserPasswordUpdate),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqTradingAccountPasswordUpdate(
    void *api, void *pTradingAccountPasswordUpdate, int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)
        ->ReqTradingAccountPasswordUpdate(
            static_cast<CThostFtdcTradingAccountPasswordUpdateField *>(
                pTradingAccountPasswordUpdate),
            nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqUserAuthMethod(void *api, void *pReqUserAuthMethod,
                                          int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqUserAuthMethod(
        static_cast<CThostFtdcReqUserAuthMethodField *>(pReqUserAuthMethod),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqGenUserCaptcha(void *api, void *pReqGenUserCaptcha,
                                          int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqGenUserCaptcha(
        static_cast<CThostFtdcReqGenUserCaptchaField *>(pReqGenUserCaptcha),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqGenUserText(void *api, void *pReqGenUserText,
                                       int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqGenUserText(
        static_cast<CThostFtdcReqGenUserTextField *>(pReqGenUserText),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqUserLoginWithCaptcha(void *api,
                                                void *pReqUserLoginWithCaptcha,
                                                int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqUserLoginWithCaptcha(
        static_cast<CThostFtdcReqUserLoginWithCaptchaField *>(
            pReqUserLoginWithCaptcha),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqUserLoginWithText(void *api,
                                             void *pReqUserLoginWithText,
                                             int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqUserLoginWithText(
        static_cast<CThostFtdcReqUserLoginWithTextField *>(
            pReqUserLoginWithText),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqUserLoginWithOTP(void *api,
                                            void *pReqUserLoginWithOTP,
                                            int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqUserLoginWithOTP(
        static_cast<CThostFtdcReqUserLoginWithOTPField *>(pReqUserLoginWithOTP),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqOrderInsert(void *api, void *pInputOrder,
                                       int nRequestID) {
  CTP_DEBUG("报单录入请求开始, api=%p, request_id=%d", api, nRequestID);
  if (api) {
    int result = static_cast<CThostFtdcTraderApi *>(api)->ReqOrderInsert(
        static_cast<CThostFtdcInputOrderField *>(pInputOrder), nRequestID);
    CTP_DEBUG("报单录入请求完成, request_id=%d, 返回值=%d", nRequestID, result);
    return result;
  }
  CTP_DEBUG("报单录入请求失败: API实例为空, request_id=%d", nRequestID);
  return -1;
}

int CThostFtdcTraderApi_ReqParkedOrderInsert(void *api, void *pParkedOrder,
                                             int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqParkedOrderInsert(
        static_cast<CThostFtdcParkedOrderField *>(pParkedOrder), nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqParkedOrderAction(void *api,
                                             void *pParkedOrderAction,
                                             int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqParkedOrderAction(
        static_cast<CThostFtdcParkedOrderActionField *>(pParkedOrderAction),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqOrderAction(void *api, void *pInputOrderAction,
                                       int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqOrderAction(
        static_cast<CThostFtdcInputOrderActionField *>(pInputOrderAction),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQryMaxOrderVolume(void *api,
                                             void *pQryMaxOrderVolume,
                                             int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQryMaxOrderVolume(
        static_cast<CThostFtdcQryMaxOrderVolumeField *>(pQryMaxOrderVolume),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqSettlementInfoConfirm(void *api,
                                                 void *pSettlementInfoConfirm,
                                                 int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqSettlementInfoConfirm(
        static_cast<CThostFtdcSettlementInfoConfirmField *>(
            pSettlementInfoConfirm),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqRemoveParkedOrder(void *api,
                                             void *pRemoveParkedOrder,
                                             int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqRemoveParkedOrder(
        static_cast<CThostFtdcRemoveParkedOrderField *>(pRemoveParkedOrder),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqRemoveParkedOrderAction(
    void *api, void *pRemoveParkedOrderAction, int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqRemoveParkedOrderAction(
        static_cast<CThostFtdcRemoveParkedOrderActionField *>(
            pRemoveParkedOrderAction),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqExecOrderInsert(void *api, void *pInputExecOrder,
                                           int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqExecOrderInsert(
        static_cast<CThostFtdcInputExecOrderField *>(pInputExecOrder),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqExecOrderAction(void *api,
                                           void *pInputExecOrderAction,
                                           int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqExecOrderAction(
        static_cast<CThostFtdcInputExecOrderActionField *>(
            pInputExecOrderAction),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqForQuoteInsert(void *api, void *pInputForQuote,
                                          int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqForQuoteInsert(
        static_cast<CThostFtdcInputForQuoteField *>(pInputForQuote),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQuoteInsert(void *api, void *pInputQuote,
                                       int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQuoteInsert(
        static_cast<CThostFtdcInputQuoteField *>(pInputQuote), nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQuoteAction(void *api, void *pInputQuoteAction,
                                       int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQuoteAction(
        static_cast<CThostFtdcInputQuoteActionField *>(pInputQuoteAction),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqBatchOrderAction(void *api,
                                            void *pInputBatchOrderAction,
                                            int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqBatchOrderAction(
        static_cast<CThostFtdcInputBatchOrderActionField *>(
            pInputBatchOrderAction),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqOptionSelfCloseInsert(void *api,
                                                 void *pInputOptionSelfClose,
                                                 int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqOptionSelfCloseInsert(
        static_cast<CThostFtdcInputOptionSelfCloseField *>(
            pInputOptionSelfClose),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqOptionSelfCloseAction(
    void *api, void *pInputOptionSelfCloseAction, int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqOptionSelfCloseAction(
        static_cast<CThostFtdcInputOptionSelfCloseActionField *>(
            pInputOptionSelfCloseAction),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqCombActionInsert(void *api, void *pInputCombAction,
                                            int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqCombActionInsert(
        static_cast<CThostFtdcInputCombActionField *>(pInputCombAction),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQryOrder(void *api, void *pQryOrder,
                                    int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQryOrder(
        static_cast<CThostFtdcQryOrderField *>(pQryOrder), nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQryTrade(void *api, void *pQryTrade,
                                    int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQryTrade(
        static_cast<CThostFtdcQryTradeField *>(pQryTrade), nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQryInvestorPosition(void *api,
                                               void *pQryInvestorPosition,
                                               int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQryInvestorPosition(
        static_cast<CThostFtdcQryInvestorPositionField *>(pQryInvestorPosition),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQryTradingAccount(void *api,
                                             void *pQryTradingAccount,
                                             int nRequestID) {
  CTP_DEBUG("查询资金账户请求开始, api=%p, request_id=%d", api, nRequestID);
  if (api) {
    int result = static_cast<CThostFtdcTraderApi *>(api)->ReqQryTradingAccount(
        static_cast<CThostFtdcQryTradingAccountField *>(pQryTradingAccount),
        nRequestID);
    CTP_DEBUG("查询资金账户请求完成, request_id=%d, 返回值=%d", nRequestID, result);
    return result;
  }
  CTP_DEBUG("查询资金账户请求失败: API实例为空, request_id=%d", nRequestID);
  return -1;
}

int CThostFtdcTraderApi_ReqQryInvestor(void *api, void *pQryInvestor,
                                       int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQryInvestor(
        static_cast<CThostFtdcQryInvestorField *>(pQryInvestor), nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQryTradingCode(void *api, void *pQryTradingCode,
                                          int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQryTradingCode(
        static_cast<CThostFtdcQryTradingCodeField *>(pQryTradingCode),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQryInstrumentMarginRate(
    void *api, void *pQryInstrumentMarginRate, int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQryInstrumentMarginRate(
        static_cast<CThostFtdcQryInstrumentMarginRateField *>(
            pQryInstrumentMarginRate),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQryInstrumentCommissionRate(
    void *api, void *pQryInstrumentCommissionRate, int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)
        ->ReqQryInstrumentCommissionRate(
            static_cast<CThostFtdcQryInstrumentCommissionRateField *>(
                pQryInstrumentCommissionRate),
            nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQryExchange(void *api, void *pQryExchange,
                                       int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQryExchange(
        static_cast<CThostFtdcQryExchangeField *>(pQryExchange), nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQryProduct(void *api, void *pQryProduct,
                                      int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQryProduct(
        static_cast<CThostFtdcQryProductField *>(pQryProduct), nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQryInstrument(void *api, void *pQryInstrument,
                                         int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQryInstrument(
        static_cast<CThostFtdcQryInstrumentField *>(pQryInstrument),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQryDepthMarketData(void *api,
                                              void *pQryDepthMarketData,
                                              int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQryDepthMarketData(
        static_cast<CThostFtdcQryDepthMarketDataField *>(pQryDepthMarketData),
        nRequestID);
  }
  return -1;
}

int CThostFtdcTraderApi_ReqQrySettlementInfo(void *api,
                                             void *pQrySettlementInfo,
                                             int nRequestID) {
  if (api) {
    return static_cast<CThostFtdcTraderApi *>(api)->ReqQrySettlementInfo(
        static_cast<CThostFtdcQrySettlementInfoField *>(pQrySettlementInfo),
        nRequestID);
  }
  return -1;
}

const char *CThostFtdcTraderApi_GetApiVersion() {
  return CThostFtdcTraderApi::GetApiVersion();
}
}
