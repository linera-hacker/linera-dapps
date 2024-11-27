import axios, { AxiosError, AxiosResponse } from 'axios'
import * as notification from './notification'

const _notification = notification.useNotificationStore()

function processError (err: AxiosError, message?: notification.Notification) {
  if (message) {
    message.Description = err.response?.statusText
  }
  if (message) {
    _notification.Notifications.push(message)
  }
}

function doAction<MyRequest, MyResponse> (
  url: string,
  req: MyRequest,
  message: notification.ReqMessage,
  success: (resp: MyResponse) => void) {
  axios
    .post<MyRequest, AxiosResponse<MyResponse>>(url, req)
    .then((response: AxiosResponse<MyResponse>) => {
      success(response.data)
      if (message.Info) {
        _notification.Notifications.push(message.Info)
      }
    })
    .catch((err: AxiosError) => {
      processError(err, message.Error)
    })
}
function doActionWithError<MyRequest, MyResponse> (
  url: string,
  req: MyRequest,
  message: notification.ReqMessage,
  success: (resp: MyResponse) => void,
  error: () => void) {
  axios
    .post<MyRequest, AxiosResponse<MyResponse>>(url, req)
    .then((response: AxiosResponse<MyResponse>) => {
      success(response.data)
      if (message.Info) {
        _notification.Notifications.push(message.Info)
      }
    })
    .catch((err: AxiosError) => {
      processError(err, message.Error)
      error()
    })
}

function doGet<MyRequest, MyResponse> (
  url: string,
  req: MyRequest,
  message: notification.ReqMessage,
  success: (resp: MyResponse) => void) {
  axios
    .get<MyRequest, AxiosResponse<MyResponse>>(url)
    .then((response: AxiosResponse<MyResponse>) => {
      success(response.data)
      if (message.Info) {
        _notification.Notifications.push(message.Info)
      }
    })
    .catch((err: AxiosError) => {
      processError(err, message.Error)
    })
}

function doGetWithError<MyRequest, MyResponse> (
  url: string,
  req: MyRequest,
  message: notification.ReqMessage,
  success: (resp: MyResponse) => void,
  error: () => void) {
  axios
    .get<MyRequest, AxiosResponse<MyResponse>>(url)
    .then((response: AxiosResponse<MyResponse>) => {
      success(response.data)
      if (message.Info) {
        _notification.Notifications.push(message.Info)
      }
    })
    .catch((err: AxiosError) => {
      processError(err, message.Error)
      error()
    })
}

export {
  doAction,
  doActionWithError,
  doGet,
  doGetWithError
}
