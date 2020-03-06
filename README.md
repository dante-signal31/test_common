# test_common
Common functions useful for tests.
____

In this crate you can find some functions I use frequently at my tests.

## Modules list
### fs 
Filesystem utilities. They are useful to prepare folders and files for your tests. Includes next modules:
###### crypto
Cryptographic functions for your tests. Here you can find hashing functions to check file contents.
###### tmp
Functions to create temporal folder and files.
###### ops
Functions for file operations (copy, delete, etc).
### random 
Utilities to generate random content for your tests. Includes next modules:
###### crypto
Functions to create random strings.
### system
Utilities to deal with your hot operating system. Includes next modules:
###### env
Functions to manipulate environment variables.
