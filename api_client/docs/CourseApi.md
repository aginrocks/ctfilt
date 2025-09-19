# \CourseApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_course**](CourseApi.md#get_course) | **GET** /api/courses/{course_slug} | Get course
[**get_course_lesson**](CourseApi.md#get_course_lesson) | **GET** /api/courses/{course_slug}/lessons/{lesson_slug} | Get lesson
[**get_course_lessons**](CourseApi.md#get_course_lessons) | **GET** /api/courses/{course_slug}/lessons | Get lessons
[**update_course**](CourseApi.md#update_course) | **PUT** /api/courses/{course_slug} | Update course
[**update_course_lesson**](CourseApi.md#update_course_lesson) | **PUT** /api/courses/{course_slug}/lessons/{lesson_slug} | Update lesson



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


## get_course_lesson

> Vec<models::LessonMetadata> get_course_lesson(course_slug, lesson_slug)
Get lesson

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_slug** | **String** | Course slug | [required] |
**lesson_slug** | **String** | Lesson slug | [required] |

### Return type

[**Vec<models::LessonMetadata>**](LessonMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_course_lessons

> Vec<models::LessonMetadata> get_course_lessons(course_slug)
Get lessons

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_slug** | **String** | Course slug | [required] |

### Return type

[**Vec<models::LessonMetadata>**](LessonMetadata.md)

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


## update_course_lesson

> models::CreateSuccess update_course_lesson(course_slug, lesson_slug, update_lesson_request)
Update lesson

If lesson is not found, it will be created

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_slug** | **String** | Course slug | [required] |
**lesson_slug** | **String** | Lesson slug | [required] |
**update_lesson_request** | [**UpdateLessonRequest**](UpdateLessonRequest.md) |  | [required] |

### Return type

[**models::CreateSuccess**](CreateSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

