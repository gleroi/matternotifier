# \LdapApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ldap_groups_get**](LdapApi.md#ldap_groups_get) | **Get** /ldap/groups | Returns a list of LDAP groups
[**ldap_groups_remote_id_link_post**](LdapApi.md#ldap_groups_remote_id_link_post) | **Post** /ldap/groups/{remote_id}/link | Link a LDAP group



## ldap_groups_get

> Vec<crate::models::LdapGroupsPaged> ldap_groups_get(q, page, per_page)
Returns a list of LDAP groups

##### Permissions Must have `manage_system` permission. __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Search term |  |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of users per page. There is a maximum limit of 200 users per page. |  |[default to 60]

### Return type

[**Vec<crate::models::LdapGroupsPaged>**](LDAPGroupsPaged.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ldap_groups_remote_id_link_post

> crate::models::StatusOk ldap_groups_remote_id_link_post(remote_id)
Link a LDAP group

##### Permissions Must have `manage_system` permission. __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remote_id** | **String** | Group GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

