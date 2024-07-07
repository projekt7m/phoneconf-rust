# \ContactListApi

All URIs are relative to *https://phone.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_contactlists_id**](ContactListApi.md#delete_contactlists_id) | **DELETE** /contactlists/{cid} | Delete a contact list by its ID
[**delete_contactlists_id_entries_id**](ContactListApi.md#delete_contactlists_id_entries_id) | **DELETE** /contactlists/{cid}/entries/{eid} | Delete a contact list entry by its ID
[**get_contact_lists**](ContactListApi.md#get_contact_lists) | **GET** /contactlists | Get all contact lists
[**get_contactlists_id**](ContactListApi.md#get_contactlists_id) | **GET** /contactlists/{cid} | Get a contact list by its ID
[**get_contactlists_id_entries**](ContactListApi.md#get_contactlists_id_entries) | **GET** /contactlists/{cid}/entries | Get all entries of a contact list
[**get_contactlists_id_entries_id**](ContactListApi.md#get_contactlists_id_entries_id) | **GET** /contactlists/{cid}/entries/{eid} | Get a single contact list entry by its ID
[**post_contactlists**](ContactListApi.md#post_contactlists) | **POST** /contactlists | Create a new contact list
[**post_contactlists_id_entries**](ContactListApi.md#post_contactlists_id_entries) | **POST** /contactlists/{cid}/entries | Add an entry to a contact list
[**put_contactlists_id**](ContactListApi.md#put_contactlists_id) | **PUT** /contactlists/{cid} | Update an existing contact list
[**put_contactlists_id_entries_id**](ContactListApi.md#put_contactlists_id_entries_id) | **PUT** /contactlists/{cid}/entries/{eid} | Update an exising contact list entry



## delete_contactlists_id

> delete_contactlists_id(cid)
Delete a contact list by its ID

Delete a contact list by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | ID of the contact list | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_contactlists_id_entries_id

> delete_contactlists_id_entries_id(cid, eid)
Delete a contact list entry by its ID

Delete a contact list entry by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | ID of the contact list | [required] |
**eid** | **String** | ID of the contact list entry | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contact_lists

> crate::models::ContactListData get_contact_lists()
Get all contact lists

Get all contact lists

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ContactListData**](ContactListData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contactlists_id

> crate::models::ContactList get_contactlists_id(cid)
Get a contact list by its ID

Get a contact list by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | ID of the contact list. | [required] |

### Return type

[**crate::models::ContactList**](ContactList.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contactlists_id_entries

> crate::models::ContactEntryData get_contactlists_id_entries(cid)
Get all entries of a contact list

Get all entries of a contact list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | ID of the contact list | [required] |

### Return type

[**crate::models::ContactEntryData**](ContactEntryData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contactlists_id_entries_id

> crate::models::ContactEntry get_contactlists_id_entries_id(cid, eid)
Get a single contact list entry by its ID

Get a single contact list entry by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | ID of the contact list. | [required] |
**eid** | **String** | ID of the entry. | [required] |

### Return type

[**crate::models::ContactEntry**](ContactEntry.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contactlists

> crate::models::ContactList post_contactlists(new_contact_list)
Create a new contact list

Create a new contact list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_contact_list** | [**NewContactList**](NewContactList.md) | The contact list to be created. | [required] |

### Return type

[**crate::models::ContactList**](ContactList.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contactlists_id_entries

> crate::models::ContactEntry post_contactlists_id_entries(cid, new_contact_entry)
Add an entry to a contact list

Add an entry to a contact list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | ID of the contact list. | [required] |
**new_contact_entry** | [**NewContactEntry**](NewContactEntry.md) | Contact list entry to be created. | [required] |

### Return type

[**crate::models::ContactEntry**](ContactEntry.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_contactlists_id

> crate::models::ContactList put_contactlists_id(cid, new_contact_list)
Update an existing contact list

Update an existing contact list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | ID of the contact list. | [required] |
**new_contact_list** | [**NewContactList**](NewContactList.md) | The updated contact list. | [required] |

### Return type

[**crate::models::ContactList**](ContactList.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_contactlists_id_entries_id

> crate::models::ContactEntry put_contactlists_id_entries_id(cid, eid, new_contact_entry)
Update an exising contact list entry

Update an exising contact list entry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | ID of the contact list | [required] |
**eid** | **String** | ID of the contact list entry | [required] |
**new_contact_entry** | [**NewContactEntry**](NewContactEntry.md) | The updated contact list entry. | [required] |

### Return type

[**crate::models::ContactEntry**](ContactEntry.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

