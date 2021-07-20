# streams-author
A microservice providing a REST API for IOTA Stream address and author information. Applications that require publication 
to an existing IOTA Streams instance should call this during bootstrap.

By running the service, a new Streams channel will be initiated and an api port opened for commands.

### Instructions
#### Configuration
Adjust the `config.json` file to match the configuration of your node structure.

Example:
```
{
  "node": "http://localhost:14265",
  "mwm": 5,
  "local_pow": true,
  "api_port": 8080,
  "seed": null
}
```

#### Running
Start a new Author instance with:
`cargo run`

This will return something like the following:
```
Making Streams channel...

Channel Address - 2cd768499b14cbdb4f9d5c0fcd2bd0f0089d7729e2bb12c2e48bbb877a17672c0000000000000000:82bff4907f5cfdb84786de26
                        ^--- This is the Channel Root => AppInst:MsgId

API listening on http://0.0.0.0:8080
```

### Demo API
Basic examples of available HTTP based curl commands

#### *get_channel_address*
Fetches the current channel application instance.

##### Args
`N/A`
##### Command
`curl --location --request GET '127.0.0.1:8080/get_channel_address' --header 'Content-Type: application/json'`
##### Return
Iota Streams `ChannelAddress/ApplicationInstance` for the current channel.
```Channel Address: 2cd768499b14cbdb4f9d5c0fcd2bd0f0089d7729e2bb12c2e48bbb877a17672c0000000000000000```
