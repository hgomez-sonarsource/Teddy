![Teddy Temp Logo](https://seeklogo.com/images/T/teddy-killerz-logo-C525D81288-seeklogo.com.png)

# Teddy
Teddy - The smart Bear's agent 

Deployable on Linux, Mac and even Windows, Teddy is a small deamon application able to execute a command, upload and download data on a machine. All operations are done via REST calls.

## Constraints
 - low memory consumption
 - compatible with Linux, Mac and Windows
 - self-content

# Technical concerns

## Build
Build with `cargo build`

## Run
Run with `cargo run`

## Run
Configuration can be overwritten in `config.json`:
```
{
  "Host": "0.0.0.0",
  "Port": "7000",
  "Authentication": {
    "User": "SonarSource",
    "Password": "number1"
  }
}
```
