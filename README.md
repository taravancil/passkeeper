# passkeeper

passkeeper is a tool for managing secrets like passwords or recovery codes.

All secrets are stored in an encrypted form and can only be decrypted with your
master password, which should be a unique and strong password (like a diceware
passphrase).

## Dependencies

[libsodium](https://github.com/jeisct1/libsodium) is required for [sodiumoxide]
(https://github.com/dnaq/sodiumoxide). Get the latest tarball, unpack it, then:

```bash
./configure
make
sudo make install
```

Then set `SODIUM_LIB_DIR` to the directory with libsodium.a:

```bash
export SODIUM_LIB_DIR=/usr/local/lib
```

## Usage

```bash
passkeeper usage
```

## How it works
On initialization, passkeeper prompts the user for a master password. A master key is
then derived from the master password using a key derivation function, scrypt.

Next, passkeeper generates a master public and private keypair. The master private key
is encrypted with the previously-generated master key. The public key is left in
plaintext, and both are stored in a configuration file.

When adding a secret to the vault, a new public and private key pair is generated for
that secret. The secret is encrypted with the master public key and the new private key,
then the encrypted secret and public key are both stored in the vault.

When fetching a secret from the vault, the secret is decrypted with site's public key
and the master private key. But first the master private key must be decrypted.

To decrypt the master private key, the master password is required. Once again, scrypt
is used to derive the master key from the master password, which is then used to decrypt
the master private key.

### Overview

* master password: user-supplied password
* master key: derived from master password used to encrypt/decrypt master private key
* master private key: encrypted with master key and stored in config file, used for
  decrypting secrets
* master public key: stored in config file as plaintext, used for encrypting secrets
* secret-specific private key: used to encrypt the secret, then discarded
* secret-specific public key: stored in vault as plaintext, used to decrypt the secret

All authentication and asymmetric and symmetric encryption/decryption is done with
sodiumoxide, which provides bindings to libsodium.
