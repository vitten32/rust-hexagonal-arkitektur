# \DefaultApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**books_book_id_delete**](DefaultApi.md#books_book_id_delete) | **DELETE** /books/{bookId} | Delete a book by ID
[**books_book_id_get**](DefaultApi.md#books_book_id_get) | **GET** /books/{bookId} | Get a book by ID
[**books_book_id_put**](DefaultApi.md#books_book_id_put) | **PUT** /books/{bookId} | Update a book by ID
[**books_get**](DefaultApi.md#books_get) | **GET** /books | Get all books
[**books_post**](DefaultApi.md#books_post) | **POST** /books | Add a new book



## books_book_id_delete

> books_book_id_delete(book_id)
Delete a book by ID

Delete a book from the library by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** | ID of the book to delete | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## books_book_id_get

> models::Book books_book_id_get(book_id)
Get a book by ID

Retrieve a book by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** | ID of the book to retrieve | [required] |

### Return type

[**models::Book**](Book.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## books_book_id_put

> models::Book books_book_id_put(book_id, book)
Update a book by ID

Update the details of a book by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_id** | **String** | ID of the book to update | [required] |
**book** | [**Book**](Book.md) | Book details to update | [required] |

### Return type

[**models::Book**](Book.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## books_get

> Vec<models::Book> books_get()
Get all books

Retrieve a list of all books in the library.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Book>**](Book.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## books_post

> models::Book books_post(book)
Add a new book

Add a new book to the library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book** | [**Book**](Book.md) | Book to add | [required] |

### Return type

[**models::Book**](Book.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

