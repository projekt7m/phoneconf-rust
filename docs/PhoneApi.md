# \PhoneApi

All URIs are relative to *https://phone.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_phones_id**](PhoneApi.md#delete_phones_id) | **DELETE** /phones/{id} | Delete a phone by its ID
[**get_phones**](PhoneApi.md#get_phones) | **GET** /phones | Get the list of all phones
[**get_phones_id**](PhoneApi.md#get_phones_id) | **GET** /phones/{id} | Get a single phone by its ID
[**post_phones**](PhoneApi.md#post_phones) | **POST** /phones | Create a new phone
[**put_phones_id**](PhoneApi.md#put_phones_id) | **PUT** /phones/{id} | Update an existing phone



## delete_phones_id

> delete_phones_id(id)
Delete a phone by its ID

Delete a phone by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the phone | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_phones

> crate::models::PhoneData get_phones()
Get the list of all phones

Get the list of all phones

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PhoneData**](PhoneData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_phones_id

> crate::models::Phone get_phones_id(id)
Get a single phone by its ID

Get a single phone by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the phone | [required] |

### Return type

[**crate::models::Phone**](Phone.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_phones

> crate::models::Phone post_phones(new_phone)
Create a new phone

Create a new phone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_phone** | [**NewPhone**](NewPhone.md) | The phone to be created | [required] |

### Return type

[**crate::models::Phone**](Phone.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_phones_id

> crate::models::Phone put_phones_id(id, new_phone)
Update an existing phone

Update an existing phone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the phone | [required] |
**new_phone** | [**NewPhone**](NewPhone.md) | The updated phone | [required] |

### Return type

[**crate::models::Phone**](Phone.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

