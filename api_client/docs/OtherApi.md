# \OtherApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_system_auth**](OtherApi.md#check_system_auth) | **GET** /api/auth/check | Check system auth
[**get_health**](OtherApi.md#get_health) | **GET** /api/health | Check server health



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


## get_health

> String get_health()
Check server health

This endpoint returns `ok`

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

