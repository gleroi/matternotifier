# \PluginsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**plugins_get**](PluginsApi.md#plugins_get) | **Get** /plugins | Get plugins
[**plugins_install_from_url_post**](PluginsApi.md#plugins_install_from_url_post) | **Post** /plugins/install_from_url | Install plugin from url
[**plugins_marketplace_get**](PluginsApi.md#plugins_marketplace_get) | **Get** /plugins/marketplace | Gets all the marketplace plugins
[**plugins_marketplace_post**](PluginsApi.md#plugins_marketplace_post) | **Post** /plugins/marketplace | Installs a marketplace plugin
[**plugins_plugin_id_delete**](PluginsApi.md#plugins_plugin_id_delete) | **Delete** /plugins/{plugin_id} | Remove plugin
[**plugins_plugin_id_disable_post**](PluginsApi.md#plugins_plugin_id_disable_post) | **Post** /plugins/{plugin_id}/disable | Disable plugin
[**plugins_plugin_id_enable_post**](PluginsApi.md#plugins_plugin_id_enable_post) | **Post** /plugins/{plugin_id}/enable | Enable plugin
[**plugins_post**](PluginsApi.md#plugins_post) | **Post** /plugins | Upload plugin
[**plugins_statuses_get**](PluginsApi.md#plugins_statuses_get) | **Get** /plugins/statuses | Get plugins status
[**plugins_webapp_get**](PluginsApi.md#plugins_webapp_get) | **Get** /plugins/webapp | Get webapp plugins



## plugins_get

> crate::models::InlineResponse20014 plugins_get()
Get plugins

Get a list of inactive and a list of active plugin manifests. Plugins must be enabled in the server's config settings.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 4.4 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20014**](inline_response_200_14.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_install_from_url_post

> crate::models::StatusOk plugins_install_from_url_post(plugin_download_url, force)
Install plugin from url

Supply a URL to a plugin compressed in a .tar.gz file. Plugins must be enabled in the server's config settings.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_download_url** | **String** | URL used to download the plugin | [required] |
**force** | Option<**String**> | Set to 'true' to overwrite a previously installed plugin with the same ID, if any |  |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_marketplace_get

> Vec<crate::models::MarketplacePlugin> plugins_marketplace_get(page, per_page, filter, server_version, local_only)
Gets all the marketplace plugins

Gets all plugins from the marketplace server, merging data from locally installed plugins as well as prepackaged plugins shipped with the server.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.16 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number to be fetched. (not yet implemented) |  |
**per_page** | Option<**i32**> | Number of item per page. (not yet implemented) |  |
**filter** | Option<**String**> | Set to filter plugins by ID, name, or description. |  |
**server_version** | Option<**String**> | Set to filter minimum plugin server version. (not yet implemented) |  |
**local_only** | Option<**bool**> | Set true to only retrieve local plugins. |  |

### Return type

[**Vec<crate::models::MarketplacePlugin>**](MarketplacePlugin.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_marketplace_post

> crate::models::PluginManifest plugins_marketplace_post(inline_object83)
Installs a marketplace plugin

Installs a plugin listed in the marketplace server.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.16 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object83** | [**InlineObject83**](InlineObject83.md) |  | [required] |

### Return type

[**crate::models::PluginManifest**](PluginManifest.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_plugin_id_delete

> crate::models::StatusOk plugins_plugin_id_delete(plugin_id)
Remove plugin

Remove the plugin with the provided ID from the server. All plugin files are deleted. Plugins must be enabled in the server's config settings.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 4.4 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_plugin_id_disable_post

> crate::models::StatusOk plugins_plugin_id_disable_post(plugin_id)
Disable plugin

Disable a previously enabled plugin. Plugins must be enabled in the server's config settings.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 4.4 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_plugin_id_enable_post

> crate::models::StatusOk plugins_plugin_id_enable_post(plugin_id)
Enable plugin

Enable a previously uploaded plugin. Plugins must be enabled in the server's config settings.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 4.4 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_post

> crate::models::StatusOk plugins_post(plugin, force)
Upload plugin

Upload a plugin that is contained within a compressed .tar.gz file. Plugins and plugin uploads must be enabled in the server's config settings.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 4.4 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **std::path::PathBuf** | The plugin image to be uploaded | [required] |
**force** | Option<**String**> | Set to 'true' to overwrite a previously installed plugin with the same ID, if any |  |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_statuses_get

> Vec<crate::models::PluginStatus> plugins_statuses_get()
Get plugins status

Returns the status for plugins installed anywhere in the cluster  ##### Permissions No permissions required.  __Minimum server version__: 4.4 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PluginStatus>**](PluginStatus.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_webapp_get

> Vec<crate::models::PluginManifestWebapp> plugins_webapp_get()
Get webapp plugins

Get a list of web app plugins installed and activated on the server.  ##### Permissions No permissions required.  __Minimum server version__: 4.4 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PluginManifestWebapp>**](PluginManifestWebapp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

