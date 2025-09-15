# \CoursesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_course**](CoursesApi.md#get_course) | **GET** /api/courses/{course_slug} | Get course
[**get_courses**](CoursesApi.md#get_courses) | **GET** /api/courses | Get all courses
[**update_course**](CoursesApi.md#update_course) | **PUT** /api/courses/{course_slug} | Update course



## get_course

> models::Course get_course(course_slug)
Get course

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_slug** | **String** | Course slug | [required] |

### Return type

[**models::Course**](Course.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_courses

> Vec<models::Course> get_courses()
Get all courses

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Course>**](Course.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_course

> models::CreateSuccess update_course(course_slug, update_course_request)
Update course

If course is not found, it will be created

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_slug** | **String** | Course slug | [required] |
**update_course_request** | [**UpdateCourseRequest**](UpdateCourseRequest.md) |  | [required] |

### Return type

[**models::CreateSuccess**](CreateSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

