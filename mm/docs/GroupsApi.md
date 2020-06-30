# \GroupsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**channels_channel_id_groups_get**](GroupsApi.md#channels_channel_id_groups_get) | **Get** /channels/{channel_id}/groups | Get channel groups
[**groups_get**](GroupsApi.md#groups_get) | **Get** /groups | Get groups
[**groups_group_id_channels_channel_id_get**](GroupsApi.md#groups_group_id_channels_channel_id_get) | **Get** /groups/{group_id}/channels/{channel_id} | Get GroupSyncable from channel ID
[**groups_group_id_channels_channel_id_link_delete**](GroupsApi.md#groups_group_id_channels_channel_id_link_delete) | **Delete** /groups/{group_id}/channels/{channel_id}/link | Delete a link from a channel to a group
[**groups_group_id_channels_channel_id_link_post**](GroupsApi.md#groups_group_id_channels_channel_id_link_post) | **Post** /groups/{group_id}/channels/{channel_id}/link | Link a channel to a group
[**groups_group_id_channels_channel_id_patch_put**](GroupsApi.md#groups_group_id_channels_channel_id_patch_put) | **Put** /groups/{group_id}/channels/{channel_id}/patch | Patch a GroupSyncable associated to Channel
[**groups_group_id_channels_get**](GroupsApi.md#groups_group_id_channels_get) | **Get** /groups/{group_id}/channels | Get group channels
[**groups_group_id_get**](GroupsApi.md#groups_group_id_get) | **Get** /groups/{group_id} | Get a group
[**groups_group_id_members_get**](GroupsApi.md#groups_group_id_members_get) | **Get** /groups/{group_id}/members | Get group users
[**groups_group_id_patch_put**](GroupsApi.md#groups_group_id_patch_put) | **Put** /groups/{group_id}/patch | Patch a group
[**groups_group_id_stats_get**](GroupsApi.md#groups_group_id_stats_get) | **Get** /groups/{group_id}/stats | Get group stats
[**groups_group_id_teams_get**](GroupsApi.md#groups_group_id_teams_get) | **Get** /groups/{group_id}/teams | Get group teams
[**groups_group_id_teams_team_id_get**](GroupsApi.md#groups_group_id_teams_team_id_get) | **Get** /groups/{group_id}/teams/{team_id} | Get GroupSyncable from Team ID
[**groups_group_id_teams_team_id_link_delete**](GroupsApi.md#groups_group_id_teams_team_id_link_delete) | **Delete** /groups/{group_id}/teams/{team_id}/link | Delete a link from a team to a group
[**groups_group_id_teams_team_id_link_post**](GroupsApi.md#groups_group_id_teams_team_id_link_post) | **Post** /groups/{group_id}/teams/{team_id}/link | Link a team to a group
[**groups_group_id_teams_team_id_patch_put**](GroupsApi.md#groups_group_id_teams_team_id_patch_put) | **Put** /groups/{group_id}/teams/{team_id}/patch | Patch a GroupSyncable associated to Team
[**ldap_groups_remote_id_link_delete**](GroupsApi.md#ldap_groups_remote_id_link_delete) | **Delete** /ldap/groups/{remote_id}/link | Delete a link for LDAP group
[**teams_team_id_groups_by_channels_get**](GroupsApi.md#teams_team_id_groups_by_channels_get) | **Get** /teams/{team_id}/groups_by_channels | Get team groups by channels
[**teams_team_id_groups_get**](GroupsApi.md#teams_team_id_groups_get) | **Get** /teams/{team_id}/groups | Get team groups
[**users_user_id_groups_get**](GroupsApi.md#users_user_id_groups_get) | **Get** /users/{user_id}/groups | Get groups for a userId



## channels_channel_id_groups_get

> Vec<crate::models::Group> channels_channel_id_groups_get(channel_id, page, per_page, filter_allow_reference)
Get channel groups

Retrieve the list of groups associated with a given channel.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of groups per page. |  |[default to 60]
**filter_allow_reference** | Option<**bool**> | Boolean which filters the group entries with the `allow_reference` attribute set. |  |[default to false]

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_get

> Vec<crate::models::Group> groups_get(not_associated_to_team, not_associated_to_channel, page, per_page, q, include_member_count, since)
Get groups

Retrieve a list of all groups not associated to a particular channel or team.  `not_associated_to_team` **OR** `not_associated_to_channel` is required.  If you use `not_associated_to_team`, you must be a team admin for that particular team (permission to manage that team).  If you use `not_associated_to_channel`, you must be a channel admin for that particular channel (permission to manage that channel).  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**not_associated_to_team** | **String** | Team GUID which is used to return all the groups not associated to this team | [required] |
**not_associated_to_channel** | **String** | Group GUID which is used to return all the groups not associated to this channel | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of groups per page. |  |[default to 60]
**q** | Option<**String**> | String to pattern match the `name` and `display_name` field. Will return all groups whose `name` and `display_name` field match any of the text. |  |
**include_member_count** | Option<**bool**> | Boolean which adds the `member_count` attribute to each group JSON object |  |
**since** | Option<**i32**> | Only return groups that have been modified since the given Unix timestamp (in milliseconds). All modified groups, including deleted and created groups, will be returned. __Minimum server version__: 5.24  |  |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_channels_channel_id_get

> crate::models::GroupSyncableChannel groups_group_id_channels_channel_id_get(group_id, channel_id)
Get GroupSyncable from channel ID

Get the GroupSyncable object with group_id and channel_id from params ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::GroupSyncableChannel**](GroupSyncableChannel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_channels_channel_id_link_delete

> crate::models::StatusOk groups_group_id_channels_channel_id_link_delete(group_id, channel_id)
Delete a link from a channel to a group

Delete a link from a channel to a group ##### Permissions If the channel is private, you must have `manage_private_channel_members` permission. Otherwise, you must have the `manage_public_channel_members` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_channels_channel_id_link_post

> crate::models::GroupSyncableChannel groups_group_id_channels_channel_id_link_post(group_id, channel_id)
Link a channel to a group

Link a channel to a group ##### Permissions If the channel is private, you must have `manage_private_channel_members` permission. Otherwise, you must have the `manage_public_channel_members` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::GroupSyncableChannel**](GroupSyncableChannel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_channels_channel_id_patch_put

> crate::models::GroupSyncableChannel groups_group_id_channels_channel_id_patch_put(group_id, channel_id, inline_object75)
Patch a GroupSyncable associated to Channel

Partially update a GroupSyncable by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**channel_id** | **String** | Channel GUID | [required] |
**inline_object75** | [**InlineObject75**](InlineObject75.md) |  | [required] |

### Return type

[**crate::models::GroupSyncableChannel**](GroupSyncableChannel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_channels_get

> Vec<crate::models::GroupSyncableChannels> groups_group_id_channels_get(group_id)
Get group channels

Retrieve the list of channels associated to the group ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |

### Return type

[**Vec<crate::models::GroupSyncableChannels>**](GroupSyncableChannels.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_get

> crate::models::Group groups_group_id_get(group_id)
Get a group

Get group from the provided group id string  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_members_get

> crate::models::InlineResponse20011 groups_group_id_members_get(group_id, page, per_page)
Get group users

Retrieve the list of users associated with a given group.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of groups per page. |  |[default to 60]

### Return type

[**crate::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_patch_put

> crate::models::Group groups_group_id_patch_put(group_id, inline_object73)
Patch a group

Partially update a group by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**inline_object73** | [**InlineObject73**](InlineObject73.md) |  | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_stats_get

> crate::models::InlineResponse20012 groups_group_id_stats_get(group_id)
Get group stats

Retrieve the stats of a given group.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.26 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |

### Return type

[**crate::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_teams_get

> Vec<crate::models::GroupSyncableTeams> groups_group_id_teams_get(group_id)
Get group teams

Retrieve the list of teams associated to the group ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |

### Return type

[**Vec<crate::models::GroupSyncableTeams>**](GroupSyncableTeams.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_teams_team_id_get

> crate::models::GroupSyncableTeam groups_group_id_teams_team_id_get(group_id, team_id)
Get GroupSyncable from Team ID

Get the GroupSyncable object with group_id and team_id from params ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**team_id** | **String** | Team GUID | [required] |

### Return type

[**crate::models::GroupSyncableTeam**](GroupSyncableTeam.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_teams_team_id_link_delete

> crate::models::StatusOk groups_group_id_teams_team_id_link_delete(group_id, team_id)
Delete a link from a team to a group

Delete a link from a team to a group ##### Permissions Must have `manage_team` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**team_id** | **String** | Team GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_teams_team_id_link_post

> crate::models::GroupSyncableTeam groups_group_id_teams_team_id_link_post(group_id, team_id)
Link a team to a group

Link a team to a group ##### Permissions Must have `manage_team` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**team_id** | **String** | Team GUID | [required] |

### Return type

[**crate::models::GroupSyncableTeam**](GroupSyncableTeam.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_group_id_teams_team_id_patch_put

> crate::models::GroupSyncableTeam groups_group_id_teams_team_id_patch_put(group_id, team_id, inline_object74)
Patch a GroupSyncable associated to Team

Partially update a GroupSyncable by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**team_id** | **String** | Team GUID | [required] |
**inline_object74** | [**InlineObject74**](InlineObject74.md) |  | [required] |

### Return type

[**crate::models::GroupSyncableTeam**](GroupSyncableTeam.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ldap_groups_remote_id_link_delete

> crate::models::StatusOk ldap_groups_remote_id_link_delete(remote_id)
Delete a link for LDAP group

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


## teams_team_id_groups_by_channels_get

> Vec<crate::models::Map> teams_team_id_groups_by_channels_get(team_id, page, per_page, filter_allow_reference)
Get team groups by channels

Retrieve the set of groups associated with the channels in the given team grouped by channel.  ##### Permissions Must have `manage_system` permission or can access only for current user  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of groups per page. |  |[default to 60]
**filter_allow_reference** | Option<**bool**> | Boolean which filters in the group entries with the `allow_reference` attribute set. |  |[default to false]

### Return type

[**Vec<crate::models::Map>**](map.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_team_id_groups_get

> Vec<crate::models::Group> teams_team_id_groups_get(team_id, page, per_page, filter_allow_reference)
Get team groups

Retrieve the list of groups associated with a given team.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of groups per page. |  |[default to 60]
**filter_allow_reference** | Option<**bool**> | Boolean which filters in the group entries with the `allow_reference` attribute set. |  |[default to false]

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_groups_get

> Vec<crate::models::Group> users_user_id_groups_get(user_id)
Get groups for a userId

Retrieve the list of groups associated to the user  __Minimum server version__: 5.24 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

