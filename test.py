import requests

content = r"""
#include <stdio.h>
int main() {
    // printf() displays the string inside quotation
    printf("Hello, World!\n");
    return 0;
}
"""
url ="http://localhost:3000/c"

resp = requests.post(url, json={"content":content})
print(resp.text)