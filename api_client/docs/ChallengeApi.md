# \ChallengeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_challenge**](ChallengeApi.md#get_challenge) | **GET** /api/challenges/{challenge_slug} | Get challenge
[**update_challenge**](ChallengeApi.md#update_challenge) | **PUT** /api/challenges/{challenge_slug} | Update challenge



## get_challenge

> models::Challenge get_challenge(challenge_slug)
Get challenge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**challenge_slug** | **String** | Challenge slug | [required] |

### Return type

[**models::Challenge**](Challenge.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_challenge

> models::CreateSuccess update_challenge(challenge_slug, update_challenge_request)
Update challenge

If challenge is not found, it will be created

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**challenge_slug** | **String** | Challenge slug | [required] |
**update_challenge_request** | [**UpdateChallengeRequest**](UpdateChallengeRequest.md) |  | [required] |

### Return type

[**models::CreateSuccess**](CreateSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

