# EMBED DECRYPT

A simple rust API to decrypt encrypted embed sources.

<h2> Table of Contents </h2>

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
    - [Installation](#installation)
    - [Running the Server](#running-the-server)
    - [API Routes](#api-routes)
- [License](#license)


## Prerequisites
- [Ts-Node](https://www.npmjs.com/package/ts-node)
- [Cargo](https://www.rust-lang.org/tools/install)

## Quick Start

Clone the respository and build the project

### Installation
```bash
git clone https://github.com/eatmynerds/embed_decrypt.git
cd embed_decrypt
cargo build --release
mv target/release/embed_decrypt .
```

# Running the server
To start the server simply run
```bash
./embed_decrypt
```
By default, the server will run on
`http://localhost:3000`

# API Routes

## `GET /` 
Returns a welcome message.

#### Example Request:
```bash
curl -X GET "http://localhost:3000/"
```

#### Example Response:
```
Welcome to the home page!
```

## `GET /embed`
Decrypts the embed source URL and returns decrypted video sources and tracks.

#### Request Parameters:
- `embed_url` - The URL of the encrypted embed source.
- `referrer` - The referrer URL of the embed source.

#### Example Request:
```bash
curl -X GET "http://localhost:3000/embed?embed_url=https://pepepeyo.xyz/v2/embed-4/DcwrA8YHCpgF?z=&referrer=https://flixhq.to"
```

#### Example Response:
```json
{
  "sources": [
    {
      "file": "https://blornixcloud65.xyz/file1/29DMMWA8BSkTLenUu0onDu6c0Eb7JW2FOBTaVx~WSD7sirwxQfizCqyTnjjFXkKnE8T1xTYMKaXpENSYoRu2K6JXM2DCwJJNl4Y+iRMDtaERJJm3MoGfvnSSfC6lKjsuQYsbmBfsNNfTfunxkZP1ikcoVNVdbEEQxVjddYdcJlU=/NzIw/aW5kZXgubTN1OA==.m3u8",
      "type": "hls"
    }
  ],
  "tracks": [
    {
      "file": "https://cc.subsceness.xyz/85/be/85be56539a0253f73cd14c9315132252/ara-6.vtt",
      "label": "Arabic - Arabic",
      "kind": "captions"
    },
    {
      "file": "https://cc.subsceness.xyz/85/be/85be56539a0253f73cd14c9315132252/eng-2.vtt",
      "label": "English - English",
      "kind": "captions",
      "default": true
    },
    ...
  ],
  "t": 1,
  "server": 29
}
```

### Docker 
Docker image is available at [Docker Hub](https://hub.docker.com/r/eatmynerds/embed_decrypt).

run the following command to pull and run the docker image.

```bash
docker pull eatmynerds/embed_decrypt
docker run -p 3000:3000 eatmynerds/embed_decrypt
```
This will start the server on port 3000. You can access the server at http://localhost:3000/, And can change the port by changing the -p option to `-p <port>:3000`.

You can add `-d` flag to run the server in detached mode.

## License
This project is licensed under the [MIT License](./LICENSE).
