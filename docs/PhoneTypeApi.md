# \PhoneTypeApi

All URIs are relative to *https://phone.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_phone_types**](PhoneTypeApi.md#get_phone_types) | **GET** /phonetypes | Get the list of all phone types
[**get_phone_types_id**](PhoneTypeApi.md#get_phone_types_id) | **GET** /phonetypes/{ptid} | Get a single phone type by its ID



## get_phone_types

> crate::models::PhoneTypeData get_phone_types()
Get the list of all phone types

Get the list of all phone types

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PhoneTypeData**](PhoneTypeData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_phone_types_id

> crate::models::PhoneType get_phone_types_id(ptid)
Get a single phone type by its ID

Get a single phone type by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ptid** | **String** | The ID of the phone type. | [required] |

### Return type

[**crate::models::PhoneType**](PhoneType.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

