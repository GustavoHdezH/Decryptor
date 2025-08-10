<div align="center"> 
  <h1>Configuration File</h1>
</div>

This document describes the configuration parameters required for the project's `.env` file.
Remember that this file must be located in the project root and should not be included in version control.

* Back to [start]

## GPG Configuration
Configuring the paths of the resources to be used

| Variable          | Description                     | Example                      |
|-------------------|---------------------------------|------------------------------|
| `INPUT_DIR`       | Input path for encrypted files  | `/home/files/input_files`    |
| `OUTPUT_DIR`      | Output path for decrypted files | `/home/files/output_files`   |
| `PASSPHRASE_FILE` | Path where the file is located  | `/home/files/passphrase.txt` |

## Age Configuration
Configuring the paths of the resources to be used

| Variable        | Description                     | Example                      |
|-----------------|---------------------------------|------------------------------|
| `INPUT_DIR`     | Input path for encrypted files  | `/home/files/input_files`    |
| `OUTPUT_DIR`    | Output path for decrypted files | `/home/files/output_files`   |
| `IDENTITY_FILE` | Path where the file is located  | `/home/files/passphrase.txt` |


## Server configuration
Configuration for CPU usage on the computer or server

| Variable         | Description                                      | Example |
|------------------|--------------------------------------------------|---------|
| `SERVER_THREADS` | Number of CPU threads to be used by the software | `6`     |



--------

<div align="center"> 
  <p>
    Made &#x2692;&#xfe0f; with &hearts; by Gussy &#x2693;  |  CXC 2020-2025
  </p>
</div>

<!-- Start of links in this document -->
[start]: ../README.md
<!-- End of links in this document -->