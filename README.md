# liburlencoding

[![](https://img.shields.io/github/v/tag/thechampagne/liburlencoding?label=version)](https://github.com/thechampagne/liburlencoding/releases/latest) [![](https://img.shields.io/github/license/thechampagne/liburlencoding)](https://github.com/thechampagne/liburlencoding/blob/main/LICENSE)

A **C** library for doing URL percentage encoding.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/liburlencoding.git
```
#### 2. Navigate to the root
```
cd liburlencoding
```
#### 3. Build the project
```
cargo build
```

### Example

```c
#include <stdio.h>
#include <urlencoding.h>

int main()
{
     char* res = url_encoding_encode("This string will be URL encoded.");
     printf("%s\n", res);
     url_encoding_free(res);
     return 0;
}
```

### References
 - [urlencoding](https://github.com/kornelski/rust_urlencoding)

### License

This repo is released under the [MIT](https://github.com/thechampagne/liburlencoding/blob/main/LICENSE).
