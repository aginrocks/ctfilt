# CourseMetadataDetailedCourseItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | A short description of the course | 
**difficulty** | [**models::CourseDifficulty**](CourseDifficulty.md) | The difficulty level of the course | 
**items** | [**Vec<models::CourseMetadataDetailedCourseItemItemsInner>**](CourseMetadata_DetailedCourseItem_items_inner.md) | Items included in the course (order matters) | 
**name** | **String** | A short unique name for the course | 
**objectives** | Option<**Vec<String>**> | A list of learning objectives for the course | [optional]
**prerequisites** | Option<**Vec<String>**> | A list of course slugs that are prerequisites for this course | [optional]
**slug** | **String** | A URL-friendly unique identifier for the course | 
**tags** | Option<**Vec<String>**> | Tags associated with the course | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


