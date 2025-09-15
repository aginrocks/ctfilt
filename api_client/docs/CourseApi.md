# \CourseApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_course_lesson**](CourseApi.md#get_course_lesson) | **GET** /api/courses/{course_slug}/lessons/{lesson_slug} | Get lesson
[**get_course_lessons**](CourseApi.md#get_course_lessons) | **GET** /api/courses/{course_slug}/lessons | Get lessons



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

