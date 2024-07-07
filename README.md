# Rust API client for p7m-phoneconf

## API for managing phones connected to P7M cloud PBX

The purpose of this API is mainly to manage the configuration of phones that use P7M auto provisioning.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.3.0
- Package version: 0.3.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `p7m-phoneconf` and add the following to `Cargo.toml` under `[dependencies]`:

```
p7m-phoneconf = { path = "./p7m-phoneconf" }
```

## Documentation for API Endpoints

All URIs are relative to *https://phone.api.p7m.de/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ContactListApi* | [**delete_contactlists_id**](docs/ContactListApi.md#delete_contactlists_id) | **DELETE** /contactlists/{cid} | Delete a contact list by its ID
*ContactListApi* | [**delete_contactlists_id_entries_id**](docs/ContactListApi.md#delete_contactlists_id_entries_id) | **DELETE** /contactlists/{cid}/entries/{eid} | Delete a contact list entry by its ID
*ContactListApi* | [**get_contact_lists**](docs/ContactListApi.md#get_contact_lists) | **GET** /contactlists | Get all contact lists
*ContactListApi* | [**get_contactlists_id**](docs/ContactListApi.md#get_contactlists_id) | **GET** /contactlists/{cid} | Get a contact list by its ID
*ContactListApi* | [**get_contactlists_id_entries**](docs/ContactListApi.md#get_contactlists_id_entries) | **GET** /contactlists/{cid}/entries | Get all entries of a contact list
*ContactListApi* | [**get_contactlists_id_entries_id**](docs/ContactListApi.md#get_contactlists_id_entries_id) | **GET** /contactlists/{cid}/entries/{eid} | Get a single contact list entry by its ID
*ContactListApi* | [**post_contactlists**](docs/ContactListApi.md#post_contactlists) | **POST** /contactlists | Create a new contact list
*ContactListApi* | [**post_contactlists_id_entries**](docs/ContactListApi.md#post_contactlists_id_entries) | **POST** /contactlists/{cid}/entries | Add an entry to a contact list
*ContactListApi* | [**put_contactlists_id**](docs/ContactListApi.md#put_contactlists_id) | **PUT** /contactlists/{cid} | Update an existing contact list
*ContactListApi* | [**put_contactlists_id_entries_id**](docs/ContactListApi.md#put_contactlists_id_entries_id) | **PUT** /contactlists/{cid}/entries/{eid} | Update an exising contact list entry
*PhoneApi* | [**delete_phones_id**](docs/PhoneApi.md#delete_phones_id) | **DELETE** /phones/{id} | Delete a phone by its ID
*PhoneApi* | [**get_phones**](docs/PhoneApi.md#get_phones) | **GET** /phones | Get the list of all phones
*PhoneApi* | [**get_phones_id**](docs/PhoneApi.md#get_phones_id) | **GET** /phones/{id} | Get a single phone by its ID
*PhoneApi* | [**post_phones**](docs/PhoneApi.md#post_phones) | **POST** /phones | Create a new phone
*PhoneApi* | [**put_phones_id**](docs/PhoneApi.md#put_phones_id) | **PUT** /phones/{id} | Update an existing phone
*PhoneAccountApi* | [**delete_phones_id_accounts_id**](docs/PhoneAccountApi.md#delete_phones_id_accounts_id) | **DELETE** /phones/{pid}/accounts/{aid} | Delete a phone account by its ID
*PhoneAccountApi* | [**get_phones_id_accounts**](docs/PhoneAccountApi.md#get_phones_id_accounts) | **GET** /phones/{pid}/accounts | Get list of all phone accounts of a phone
*PhoneAccountApi* | [**get_phones_id_accounts_id**](docs/PhoneAccountApi.md#get_phones_id_accounts_id) | **GET** /phones/{pid}/accounts/{aid} | Request a single phone account by its ID
*PhoneAccountApi* | [**post_phones_id_accounts**](docs/PhoneAccountApi.md#post_phones_id_accounts) | **POST** /phones/{pid}/accounts | Create a new phone account
*PhoneAccountApi* | [**put_phones_id_accounts_id**](docs/PhoneAccountApi.md#put_phones_id_accounts_id) | **PUT** /phones/{pid}/accounts/{aid} | Update an existing phone account
*PhoneKeyApi* | [**get_phones_id_keys**](docs/PhoneKeyApi.md#get_phones_id_keys) | **GET** /phones/{pid}/keys | Get all the key configuration of a phone
*PhoneKeyApi* | [**get_phones_id_keys_id**](docs/PhoneKeyApi.md#get_phones_id_keys_id) | **GET** /phones/{pid}/keys/{kid} | Get a single phone key by its ID
*PhoneKeyApi* | [**post_phones_id_keys**](docs/PhoneKeyApi.md#post_phones_id_keys) | **POST** /phones/{pid}/keys | Create a new phone key
*PhoneKeyApi* | [**put_phones_id_keys_id**](docs/PhoneKeyApi.md#put_phones_id_keys_id) | **PUT** /phones/{pid}/keys/{kid} | Update an existing phone key
*PhoneTypeApi* | [**get_phone_types**](docs/PhoneTypeApi.md#get_phone_types) | **GET** /phonetypes | Get the list of all phone types
*PhoneTypeApi* | [**get_phone_types_id**](docs/PhoneTypeApi.md#get_phone_types_id) | **GET** /phonetypes/{ptid} | Get a single phone type by its ID


## Documentation For Models

 - [ContactEntry](docs/ContactEntry.md)
 - [ContactEntryData](docs/ContactEntryData.md)
 - [ContactList](docs/ContactList.md)
 - [ContactListData](docs/ContactListData.md)
 - [KeyType](docs/KeyType.md)
 - [NewContactEntry](docs/NewContactEntry.md)
 - [NewContactList](docs/NewContactList.md)
 - [NewPhone](docs/NewPhone.md)
 - [NewPhoneAccount](docs/NewPhoneAccount.md)
 - [NewPhoneKey](docs/NewPhoneKey.md)
 - [Phone](docs/Phone.md)
 - [PhoneAccount](docs/PhoneAccount.md)
 - [PhoneAccountData](docs/PhoneAccountData.md)
 - [PhoneData](docs/PhoneData.md)
 - [PhoneKey](docs/PhoneKey.md)
 - [PhoneKeyData](docs/PhoneKeyData.md)
 - [PhoneType](docs/PhoneType.md)
 - [PhoneTypeData](docs/PhoneTypeData.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

tech@p7m.de

