#ifndef __URL_ENCODING_H__
#define __URL_ENCODING_H__

#ifdef __cplusplus
extern "C" {
#endif

/*
* Percent-encodes every byte except alphanumerics and -, _, ., ~. Assumes UTF-8 encoding.
* 
* Example:
* * *
* int main()
* {
*     char* res = url_encoding_encode("This string will be URL encoded.");
*     printf("%s\n", res);
*     url_encoding_free(res);
*     return 0;
* }
* * *
* 
* @param data
* @return dynamic string
*/
extern char* url_encoding_encode(const char* data);

/*
* Percent-encodes every byte except alphanumerics and -, _, ., ~.
* 
* Example:
* * *
* int main()
* {
*     const unsigned char* str = "This string will be URL encoded.";
*     char* res = url_encoding_encode_binary(str, strlen(str));
*     printf("%s\n", res);
*     url_encoding_free(res);
*     return 0;
* }
* * *
* 
* @param data
* @param length data length
* @return dynamic string
*/
extern char* url_encoding_encode_binary(const unsigned char* data, size_t length);

/*
* Decode percent-encoded string assuming UTF-8 encoding.
* 
* Example:
* * *
* int main()
* {
*     char* res = url_encoding_decode("%F0%9F%91%BE%20Exterminate%21");
*     printf("%s\n", res);
*     url_encoding_free(res);
*     return 0;
* }
* * *
* 
* @param data
* @return dynamic string
*/
extern char* url_encoding_decode(const char* data);

/*
* Decode percent-encoded string as binary data, in any encoding.
* 
* Example:
* * *
* int main()
* {
*     const unsigned char* str = "%F1%F2%F3%C0%C1%C2";
*     char* res = url_encoding_decode_binary(str, strlen(str));
*     printf("%s\n", res);
*     url_encoding_free(res);
*     return 0;
* }
* * *
* 
* @param data
* @param length data length
* @return dynamic string
*/
extern char* url_encoding_decode_binary(const unsigned char* data, size_t length);

/*
* function to free the memory after using urlencoding functions
*
* @param ptr string returned from urlencoding functions
*/
extern void url_encoding_free(char* ptr);

#ifdef __cplusplus
}
#endif

#endif // __URL_ENCODING_H__