# \ChallengeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**extend_challenge**](ChallengeApi.md#extend_challenge) | **POST** /api/challenges/{challenge_slug}/extend | Extend a challenge
[**get_challenge**](ChallengeApi.md#get_challenge) | **GET** /api/challenges/{challenge_slug} | Get challenge
[**start_challenge**](ChallengeApi.md#start_challenge) | **POST** /api/challenges/{challenge_slug}/start | Start a challenge
[**stop_challenge**](ChallengeApi.md#stop_challenge) | **POST** /api/challenges/{challenge_slug}/stop | Stop a challenge
[**submit_flag**](ChallengeApi.md#submit_flag) | **POST** /api/challenges/{challenge_slug}/submit | Submit a flag
[**update_challenge**](ChallengeApi.md#update_challenge) | **PUT** /api/challenges/{challenge_slug} | Update challenge



## extend_challenge

> models::KubernetesActionResult extend_challenge(challenge_slug)
Extend a challenge

This endpoint adds more time to a challenge instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**challenge_slug** | **String** | Challenge slug | [required] |

### Return type

[**models::KubernetesActionResult**](KubernetesActionResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## start_challenge

> models::KubernetesActionResult start_challenge(challenge_slug)
Start a challenge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**challenge_slug** | **String** | Challenge slug | [required] |

### Return type

[**models::KubernetesActionResult**](KubernetesActionResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_challenge

> models::KubernetesActionResult stop_challenge(challenge_slug)
Stop a challenge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**challenge_slug** | **String** | Challenge slug | [required] |

### Return type

[**models::KubernetesActionResult**](KubernetesActionResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_flag

> models::FlagSubmissionResult submit_flag(challenge_slug, flag_submission_request)
Submit a flag

Submits a flag for the specified challenge. If all correct flags have been submitted, the challenge instance will be stopped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**challenge_slug** | **String** | Challenge slug | [required] |
**flag_submission_request** | [**FlagSubmissionRequest**](FlagSubmissionRequest.md) |  | [required] |

### Return type

[**models::FlagSubmissionResult**](FlagSubmissionResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
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

