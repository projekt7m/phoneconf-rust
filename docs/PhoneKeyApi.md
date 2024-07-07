# \PhoneKeyApi

All URIs are relative to *https://phone.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_phones_id_keys**](PhoneKeyApi.md#get_phones_id_keys) | **GET** /phones/{pid}/keys | Get all the key configuration of a phone
[**get_phones_id_keys_id**](PhoneKeyApi.md#get_phones_id_keys_id) | **GET** /phones/{pid}/keys/{kid} | Get a single phone key by its ID
[**post_phones_id_keys**](PhoneKeyApi.md#post_phones_id_keys) | **POST** /phones/{pid}/keys | Create a new phone key
[**put_phones_id_keys_id**](PhoneKeyApi.md#put_phones_id_keys_id) | **PUT** /phones/{pid}/keys/{kid} | Update an existing phone key



## get_phones_id_keys

> crate::models::PhoneKeyData get_phones_id_keys(pid)
Get all the key configuration of a phone

Get all the key configuration of a phone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | ID of the phone | [required] |

### Return type

[**crate::models::PhoneKeyData**](PhoneKeyData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_phones_id_keys_id

> crate::models::PhoneKey get_phones_id_keys_id(pid, kid)
Get a single phone key by its ID

Get a single phone key by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | ID of the phone | [required] |
**kid** | **String** | ID of the phone key | [required] |

### Return type

[**crate::models::PhoneKey**](PhoneKey.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_phones_id_keys

> crate::models::PhoneKey post_phones_id_keys(pid, new_phone_key)
Create a new phone key

Create a new phone key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | ID of the phone | [required] |
**new_phone_key** | [**NewPhoneKey**](NewPhoneKey.md) | The phone key to be created | [required] |

### Return type

[**crate::models::PhoneKey**](PhoneKey.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_phones_id_keys_id

> crate::models::PhoneKey put_phones_id_keys_id(pid, kid, new_phone_key)
Update an existing phone key

Update an existing phone key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | ID of the phone | [required] |
**kid** | **String** | ID of the phone key | [required] |
**new_phone_key** | [**NewPhoneKey**](NewPhoneKey.md) | The updated phone key configuration | [required] |

### Return type

[**crate::models::PhoneKey**](PhoneKey.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

