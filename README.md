# ROT-26-rs

ROT-26 algorithm implemented in Rust with a C interface

> Instead of only rotating 13 places, ROT26 rotates twice as many characters in the alphabet and is therefore twice as secure.

# Examples

## Rust
```rust
fn main() {
    let super_secret_data = get_password();
    let super_secret_data_encrypted = rot26_rs::cipher(super_secret_data);
    
    let super_secret_data = rot26::decipher(super_secret_data_encrypted);
}
```

## C
```c
...
#include <rot26.h>

int
main
(
    void
)
{
    const char *super_secret_data = get_password();
    const char *super_secret_data_encrypted = rot26_cipher(super_secret_data);
    
    const char *super_secret_data_deciphered = rot26_decipher(super_secret_data_encrypted);
    return 0;
}
```

## C
```c++
...
#include <rot26>

int
main
(
    void
)
{
    const char *super_secret_data = get_password();
    const char *super_secret_data_encrypted = rot26_cipher(super_secret_data);
    
    const char *super_secret_data_deciphered = rot26_decipher(super_secret_data_encrypted);
    return 0;
}
```
