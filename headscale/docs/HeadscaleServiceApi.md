# \HeadscaleServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**headscale_service_backfill_node_ips**](HeadscaleServiceApi.md#headscale_service_backfill_node_ips) | **POST** /api/v1/node/backfillips | 
[**headscale_service_create_api_key**](HeadscaleServiceApi.md#headscale_service_create_api_key) | **POST** /api/v1/apikey | --- ApiKeys start ---
[**headscale_service_create_pre_auth_key**](HeadscaleServiceApi.md#headscale_service_create_pre_auth_key) | **POST** /api/v1/preauthkey | --- PreAuthKeys start ---
[**headscale_service_create_user**](HeadscaleServiceApi.md#headscale_service_create_user) | **POST** /api/v1/user | --- User start ---
[**headscale_service_debug_create_node**](HeadscaleServiceApi.md#headscale_service_debug_create_node) | **POST** /api/v1/debug/node | --- Node start ---
[**headscale_service_delete_api_key**](HeadscaleServiceApi.md#headscale_service_delete_api_key) | **DELETE** /api/v1/apikey/{prefix} | 
[**headscale_service_delete_node**](HeadscaleServiceApi.md#headscale_service_delete_node) | **DELETE** /api/v1/node/{nodeId} | 
[**headscale_service_delete_user**](HeadscaleServiceApi.md#headscale_service_delete_user) | **DELETE** /api/v1/user/{id} | 
[**headscale_service_expire_api_key**](HeadscaleServiceApi.md#headscale_service_expire_api_key) | **POST** /api/v1/apikey/expire | 
[**headscale_service_expire_node**](HeadscaleServiceApi.md#headscale_service_expire_node) | **POST** /api/v1/node/{nodeId}/expire | 
[**headscale_service_expire_pre_auth_key**](HeadscaleServiceApi.md#headscale_service_expire_pre_auth_key) | **POST** /api/v1/preauthkey/expire | 
[**headscale_service_get_node**](HeadscaleServiceApi.md#headscale_service_get_node) | **GET** /api/v1/node/{nodeId} | 
[**headscale_service_get_policy**](HeadscaleServiceApi.md#headscale_service_get_policy) | **GET** /api/v1/policy | --- Policy start ---
[**headscale_service_list_api_keys**](HeadscaleServiceApi.md#headscale_service_list_api_keys) | **GET** /api/v1/apikey | 
[**headscale_service_list_nodes**](HeadscaleServiceApi.md#headscale_service_list_nodes) | **GET** /api/v1/node | 
[**headscale_service_list_pre_auth_keys**](HeadscaleServiceApi.md#headscale_service_list_pre_auth_keys) | **GET** /api/v1/preauthkey | 
[**headscale_service_list_users**](HeadscaleServiceApi.md#headscale_service_list_users) | **GET** /api/v1/user | 
[**headscale_service_move_node**](HeadscaleServiceApi.md#headscale_service_move_node) | **POST** /api/v1/node/{nodeId}/user | 
[**headscale_service_register_node**](HeadscaleServiceApi.md#headscale_service_register_node) | **POST** /api/v1/node/register | 
[**headscale_service_rename_node**](HeadscaleServiceApi.md#headscale_service_rename_node) | **POST** /api/v1/node/{nodeId}/rename/{newName} | 
[**headscale_service_rename_user**](HeadscaleServiceApi.md#headscale_service_rename_user) | **POST** /api/v1/user/{oldId}/rename/{newName} | 
[**headscale_service_set_approved_routes**](HeadscaleServiceApi.md#headscale_service_set_approved_routes) | **POST** /api/v1/node/{nodeId}/approve_routes | 
[**headscale_service_set_policy**](HeadscaleServiceApi.md#headscale_service_set_policy) | **PUT** /api/v1/policy | 
[**headscale_service_set_tags**](HeadscaleServiceApi.md#headscale_service_set_tags) | **POST** /api/v1/node/{nodeId}/tags | 



## headscale_service_backfill_node_ips

> models::V1BackfillNodeIpsResponse headscale_service_backfill_node_ips(confirmed)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**confirmed** | Option<**bool**> |  |  |

### Return type

[**models::V1BackfillNodeIpsResponse**](v1BackfillNodeIPsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_create_api_key

> models::V1CreateApiKeyResponse headscale_service_create_api_key(body)
--- ApiKeys start ---

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1CreateApiKeyRequest**](V1CreateApiKeyRequest.md) |  | [required] |

### Return type

[**models::V1CreateApiKeyResponse**](v1CreateApiKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_create_pre_auth_key

> models::V1CreatePreAuthKeyResponse headscale_service_create_pre_auth_key(body)
--- PreAuthKeys start ---

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1CreatePreAuthKeyRequest**](V1CreatePreAuthKeyRequest.md) |  | [required] |

### Return type

[**models::V1CreatePreAuthKeyResponse**](v1CreatePreAuthKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_create_user

> models::V1CreateUserResponse headscale_service_create_user(body)
--- User start ---

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1CreateUserRequest**](V1CreateUserRequest.md) |  | [required] |

### Return type

[**models::V1CreateUserResponse**](v1CreateUserResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_debug_create_node

> models::V1DebugCreateNodeResponse headscale_service_debug_create_node(body)
--- Node start ---

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1DebugCreateNodeRequest**](V1DebugCreateNodeRequest.md) |  | [required] |

### Return type

[**models::V1DebugCreateNodeResponse**](v1DebugCreateNodeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_delete_api_key

> serde_json::Value headscale_service_delete_api_key(prefix)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_delete_node

> serde_json::Value headscale_service_delete_node(node_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_delete_user

> serde_json::Value headscale_service_delete_user(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_expire_api_key

> serde_json::Value headscale_service_expire_api_key(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1ExpireApiKeyRequest**](V1ExpireApiKeyRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_expire_node

> models::V1ExpireNodeResponse headscale_service_expire_node(node_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** |  | [required] |

### Return type

[**models::V1ExpireNodeResponse**](v1ExpireNodeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_expire_pre_auth_key

> serde_json::Value headscale_service_expire_pre_auth_key(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1ExpirePreAuthKeyRequest**](V1ExpirePreAuthKeyRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_get_node

> models::V1GetNodeResponse headscale_service_get_node(node_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** |  | [required] |

### Return type

[**models::V1GetNodeResponse**](v1GetNodeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_get_policy

> models::V1GetPolicyResponse headscale_service_get_policy()
--- Policy start ---

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::V1GetPolicyResponse**](v1GetPolicyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_list_api_keys

> models::V1ListApiKeysResponse headscale_service_list_api_keys()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::V1ListApiKeysResponse**](v1ListApiKeysResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_list_nodes

> models::V1ListNodesResponse headscale_service_list_nodes(user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | Option<**String**> |  |  |

### Return type

[**models::V1ListNodesResponse**](v1ListNodesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_list_pre_auth_keys

> models::V1ListPreAuthKeysResponse headscale_service_list_pre_auth_keys(user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | Option<**String**> |  |  |

### Return type

[**models::V1ListPreAuthKeysResponse**](v1ListPreAuthKeysResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_list_users

> models::V1ListUsersResponse headscale_service_list_users(id, name, email)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**email** | Option<**String**> |  |  |

### Return type

[**models::V1ListUsersResponse**](v1ListUsersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_move_node

> models::V1MoveNodeResponse headscale_service_move_node(node_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** |  | [required] |
**body** | [**HeadscaleServiceMoveNodeBody**](HeadscaleServiceMoveNodeBody.md) |  | [required] |

### Return type

[**models::V1MoveNodeResponse**](v1MoveNodeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_register_node

> models::V1RegisterNodeResponse headscale_service_register_node(user, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | Option<**String**> |  |  |
**key** | Option<**String**> |  |  |

### Return type

[**models::V1RegisterNodeResponse**](v1RegisterNodeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_rename_node

> models::V1RenameNodeResponse headscale_service_rename_node(node_id, new_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** |  | [required] |
**new_name** | **String** |  | [required] |

### Return type

[**models::V1RenameNodeResponse**](v1RenameNodeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_rename_user

> models::V1RenameUserResponse headscale_service_rename_user(old_id, new_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**old_id** | **String** |  | [required] |
**new_name** | **String** |  | [required] |

### Return type

[**models::V1RenameUserResponse**](v1RenameUserResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_set_approved_routes

> models::V1SetApprovedRoutesResponse headscale_service_set_approved_routes(node_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** |  | [required] |
**body** | [**HeadscaleServiceSetApprovedRoutesBody**](HeadscaleServiceSetApprovedRoutesBody.md) |  | [required] |

### Return type

[**models::V1SetApprovedRoutesResponse**](v1SetApprovedRoutesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_set_policy

> models::V1SetPolicyResponse headscale_service_set_policy(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1SetPolicyRequest**](V1SetPolicyRequest.md) |  | [required] |

### Return type

[**models::V1SetPolicyResponse**](v1SetPolicyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## headscale_service_set_tags

> models::V1SetTagsResponse headscale_service_set_tags(node_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** |  | [required] |
**body** | [**HeadscaleServiceSetTagsBody**](HeadscaleServiceSetTagsBody.md) |  | [required] |

### Return type

[**models::V1SetTagsResponse**](v1SetTagsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

