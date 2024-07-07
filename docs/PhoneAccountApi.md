# \PhoneAccountApi

All URIs are relative to *https://phone.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_phones_id_accounts_id**](PhoneAccountApi.md#delete_phones_id_accounts_id) | **DELETE** /phones/{pid}/accounts/{aid} | Delete a phone account by its ID
[**get_phones_id_accounts**](PhoneAccountApi.md#get_phones_id_accounts) | **GET** /phones/{pid}/accounts | Get list of all phone accounts of a phone
[**get_phones_id_accounts_id**](PhoneAccountApi.md#get_phones_id_accounts_id) | **GET** /phones/{pid}/accounts/{aid} | Request a single phone account by its ID
[**post_phones_id_accounts**](PhoneAccountApi.md#post_phones_id_accounts) | **POST** /phones/{pid}/accounts | Create a new phone account
[**put_phones_id_accounts_id**](PhoneAccountApi.md#put_phones_id_accounts_id) | **PUT** /phones/{pid}/accounts/{aid} | Update an existing phone account



## delete_phones_id_accounts_id

> delete_phones_id_accounts_id(pid, aid)
Delete a phone account by its ID

Delete a phone account by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | ID of the phone. | [required] |
**aid** | **String** | ID of the account. | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_phones_id_accounts

> crate::models::PhoneAccountData get_phones_id_accounts(pid)
Get list of all phone accounts of a phone

Get list of all phone accounts of a phone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | ID of the phone | [required] |

### Return type

[**crate::models::PhoneAccountData**](PhoneAccountData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_phones_id_accounts_id

> crate::models::PhoneAccount get_phones_id_accounts_id(pid, aid)
Request a single phone account by its ID

Request a single phone account by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | ID of the phone | [required] |
**aid** | **String** | ID Of the account | [required] |

### Return type

[**crate::models::PhoneAccount**](PhoneAccount.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_phones_id_accounts

> crate::models::PhoneAccount post_phones_id_accounts(pid, new_phone_account)
Create a new phone account

Create a new phone account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | ID of the phone | [required] |
**new_phone_account** | [**NewPhoneAccount**](NewPhoneAccount.md) | The phone account to be created. | [required] |

### Return type

[**crate::models::PhoneAccount**](PhoneAccount.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_phones_id_accounts_id

> crate::models::PhoneAccount put_phones_id_accounts_id(pid, aid, new_phone_account)
Update an existing phone account

Update an existing phone account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** | ID of the phone | [required] |
**aid** | **String** | ID of the phone account | [required] |
**new_phone_account** | [**NewPhoneAccount**](NewPhoneAccount.md) | The updated phone account configuration. | [required] |

### Return type

[**crate::models::PhoneAccount**](PhoneAccount.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

