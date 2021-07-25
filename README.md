# Vernam cipher

Encode and decode a message with a vername cipher (or one-time pad).

In cryptography, the one-time pad (OTP) is an encryption technique that cannot be cracked, but requires the use of a single-use pre-shared key that is no smaller than the message being sent. In this technique, a plaintext is paired with a random secret key (also referred to as a one-time pad). Then, each bit or character of the plaintext is encrypted by combining it with the corresponding bit or character from the pad using modular addition. [Read more on wikipedia](https://en.wikipedia.org/wiki/One-time_pad)

Encrypt
---

```sh
vernam encrypt HELLO WMCKL # Outputs DQNVZ
```

Decrypt
---

```sh
vernam decrypt DQNVZ WMCKL # Outputs HELLO
```