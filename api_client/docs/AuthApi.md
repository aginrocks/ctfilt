# \AuthApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_system_auth**](AuthApi.md#check_system_auth) | **GET** /api/auth/check | Check system auth
[**get_user**](AuthApi.md#get_user) | **GET** /api/user | Get user details
[**log_in**](AuthApi.md#log_in) | **GET** /api/login | Log in



## check_system_auth

> models::AuthCheckSuccess check_system_auth()
Check system auth

Checks if your API key is valid  This endpoint is only avaliable to users authenticating with a system-wide API key

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthCheckSuccess**](AuthCheckSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::User get_user()
Get user details

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## log_in

> log_in()
Log in

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

