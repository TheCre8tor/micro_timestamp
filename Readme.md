# Micro Timestamp

### 1. Test

```
cargo test
```

### 2. Start Server

```
cargo run
```

### 3. Endpoints & Sample Response

```
❯ http | GET | http://127.0.0.1:6070/api/
```

```json
HTTP/1.1 200 OK
content-length: 59
content-type: application/json
date: Mon, 07 Feb 2022 07:35:11 GMT

{
  "unix": 1644219311,
  "utc": "Mon, 07 Feb 2022 07:35:11 +0000"
}
```

```
❯ http | GET | http://127.0.0.1:6070/api/1441670400
```

```json
HTTP/1.1 200 OK
content-length: 59
content-type: application/json
date: Mon, 07 Feb 2022 07:35:11 GMT

{
  "unix": 1644219311,
  "utc": "Mon, 07 Feb 2022 07:35:11 +0000"
}
```

```
❯ http | GET | http://127.0.0.1:6070/api/2015-05-10
```

```json
HTTP/1.1 200 OK
content-length: 59
content-type: application/json
date: Mon, 07 Feb 2022 07:35:11 GMT

{
  "unix": 1431216000,
  "utc": "Sun, 10 May 2015 00:00:00 +0000"
}
```
