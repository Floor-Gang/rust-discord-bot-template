# Rust discord template
aThis template is configured with postgres and serenity.

## How to start
Clone the repo with 
```bash
git clone https://github.com/DankDumpster/rust-discord-bot-template
```

Build and run it for the first time
```bash
cargo run
```
NOTE: This will panic, and will create a config.yml file for you

Now fill in the config.yml like it asks you to.
If anything goes wrong for any reason you can manually make a config.yml in the root with this:
```yaml
---
token: ""
prefix: ;
db_uri: "postgres://postgres:postgres@localhost/postgres"
```

Now run it again and it should work
```bash 
cargo run
```

## Notes
### Custom Config Location
To set a custom config location, set the enviroment variable "CONFIG_PATH" and point it to a file.

###### Massive thanks to [dylan](https://github.com/dylhack) for the config function
