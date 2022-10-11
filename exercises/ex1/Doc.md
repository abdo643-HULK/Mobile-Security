# 1.1

## Answer

-   Key: XIQVNALDEKWFOURHTPCBMGYS
-   Text: MOREOVER, THE RELATION'S CONTENTUAL INFORMATION MAY REVEAL VERY HIGH SENSITIVITY TO ALTERATIONS IN GENERAL. SOME EXTREME EXAMPLES OF SUCH DATA POOLS INCLUDE MEDICAL, MILITARY AND RESEARCH DATABASES: THOSE MAY CONTAIN INFORMATION WHICH REQUIRES UTTERLY PRECISE ACCURACY AND WON’T BE USEFUL OTHERWISE. EVEN WORSE, ONE MAY THINK OF SCENARIOS WHICH WILL NOT ONLY MAKE THE DATA IN QUESTION USELESS FOR ITS ORIGINAL INTENT, BUT MAY SERIOUSLY CORRUPT CORPORATIONS, RESEARCH RESULTS OR IN EXTREME CASES, EVEN PEOPLE. IN ADDITION, THERE ARE COUNTLESS HIGH-LEVEL DATA TYPES WHICH DO NOT TOLERATE EVEN SLIGHT MODIFICATIONS TO THE DATA THEY WERE DEVELOPED TO CONTAIN AND ACCUMULATE. EXAMPLES OF SUCH DATA TYPES INCLUDE EARTH COORDINATE FRAME DATA TYPES LIKE THE WORLD GEODETIC SYSTEM (THE LATEST BEING WGS84), PHARMACEUTICAL INDICATION OF CHEMICAL QUANTITIES, OR MONETARY DATA REFERRING TO THE GLOBAL BANKING SYSTEM.

## Guide

1. I started with a frequenzy analysis and the code for that is in `cryptoanalysis.rs` inside `MonoAlphabeticDecrypter::analyze`
2. I found that "RPI" is repeated a lot is the letters were also in the top 5 most accuring character. So I replaced it with "the", because it's the most common word in english texts.
3. In the decrypt code I replaced every character I didn't know the key for with an underscore to distinguish them.
4. After that I looked for other articles.
5. Than for two letter words.
6. From there some words began clear and I repeated the process until I found all letters.

# 1.2

Code in `playfair.rs`

## How to run

If you don't have rust go to: https://www.rust-lang.org/tools/install and download the toolchain (it's extremly easy).

Than just run the programm by typing:

```sh
cargo run -- -k PASSWORD -e 'SECURE MESSAGE'
```

# 1.3

## Answer

-   Key: 4
-   Text: FINDE DAS SCHLOSS BEI VIER ACHT ZWEI EINS DREI NEUN NEUN UND EINS VIER DREI EINS SECHS EINS SECHS

## Steps

From the hint on the webpage it looked like it was ROT-13 but it didn't work the other alghorithm that used a similar rotation is rail fance and it worked.

The code for the decoding is in `geocaching.rs`

# 1.4

## Cryptanalytic Approach

## Decoded Message

SELinux is a mandatory access control mechanism for the Linux kernel, implemented as a Linux security module. The Linux Security Modules (LSM) framework allows third-party access control mechanisms to be linked into the kernel and to modify the default DAC implementation. LSM is implemented as a series of security function hooks (upcalls) and related data structures that are integrated into the various modules of the Linux kernel responsible for access control.

Source: https://guide.aosp.ir/fa/latest/references/books/Android_Security_Internals.pdf

# Bonus
