# Type Recommendation Project

When we start typing on a search engine, a list of recommendations for names are suggested related to what we wrote. This warp server simulates that. It is a prefix tree that stores nodes for each character of the names. When a prefix name is reached, it will count all the children and return a list of the top entries found. If one of the entries is selected, it will increment one of the entries by 1.


It is also intended as an example of conditional compilation in Rust. If feature `dotenv` is enabled, it will load the local '.env' file. If compiled for `--release`, it will not show debug messages.



# Start up 

On start up, it will load entries from a 'names.json' file. Each entry has a string name and the number of clicks that name received.

# EndPoints

---

Endpoint: `/rec/{prefix}`

Method: `GET`

Description:

Retrieve the list of the top names that start with the given prefix. If omitted, return the top entries overall.

Parameter:
 
 - `prefix` (optional, string): The string to be used as prefix to filter the resources.
 
Example Request:

```bash
$ curl http://127.0.0.1:3030/rec/ | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   367  100   367    0     0   296k      0 --:--:-- --:--:-- --:--:--  358k
[
  {
    "name": "Abel",
    "times": 999
  },
  {
    "name": "Adria Acevedo",
    "times": 999
  },
  {
    "name": "Aiko",
    "times": 999
  },
  {
    "name": "Allen Nicholson",
    "times": 999
  },
  {
    "name": "Amaya",
    "times": 999
  },
  {
    "name": "Amber Roberts",
    "times": 999
  },
  {
    "name": "Amber Wall",
    "times": 999
  },
  {
    "name": "Amber Wilkerson",
    "times": 999
  },
  {
    "name": "Amir",
    "times": 999
  },
  {
    "name": "Amy Mcfarland",
    "times": 999
  },
  {
    "name": "Andrew",
    "times": 999
  }
]
```

---

Example Request with prefix:

```bash
$ curl http://127.0.0.1:3030/rec/ame | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   177  100   177    0     0   540k      0 --:--:-- --:--:-- --:--:--  172k
[
  {
    "name": "Amela",
    "times": 678
  },
  {
    "name": "Amethyst Vance",
    "times": 673
  },
  {
    "name": "Amethyst Buck",
    "times": 533
  },
  {
    "name": "Amela Acosta",
    "times": 148
  },
  {
    "name": "Amery Mcbride",
    "times": 91
  }
]
```

---

Endpoint: '/rec/

Method: `POST`

Description:
This endpoint increments an existing entry by 1. Requires a JSON object as the Request body with the following properties:

 - `name` (required, string): The full name to be incremented.

It will not create new entries. If name doesn't exist it will return an error.

Example Request Body:

```json
{
    "name":"Amber Wilkerson"
}
```

Example Success Response:

```bash
$ curl http://127.0.0.1:3030/rec/ame | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   177  100   177    0     0   540k      0 --:--:-- --:--:-- --:--:--  172k
[
  {
    "name": "Amela",
    "times": 678
  },
  {
    "name": "Amethyst Vance",
    "times": 673
  },
  {
    "name": "Amethyst Buck",
    "times": 533
  },
  {
    "name": "Amela Acosta",
    "times": 148
  },
  {
    "name": "Amery Mcbride",
    "times": 91
  }
]
```

If prefix is not found, it returns an empty list:

```bash
$ curl http://127.0.0.1:3030/rec/ann
[]
```

---

# Environment Variables

The following environment variables need to be set before running the application:

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
|  `HOST`  | Server Host | No       | '0.0.0.0' |
|  `PORT`  | Server Port | Yes      | N/A     |
| `SUGGESTION_NUMBER` | Maximum number of entries that can be returned by a request | YES | N/A |

To load from an existing '.env' file, enable the feature 'dotenv'.

```bash
cargo run --features dotenv
```

or

```bash
cargo build --features dotenv
```

---

# Debugging

If compiled as release, debug messages will not exist in the machine code. To do so, just include the `--release` option.

```bash
cargo build --release
```

or

```bash
cargo run --release
```

To see a detailed description of the search through the prefix tree. Just compile it without the `--release` option.
