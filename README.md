# Web API サーバー on Rust サンプル

- Rust で実装した Web API サーバーのサンプル。

## 必要要件

- Docker
- docker-composed
- Rust
- diesel

```term
$ cd server
$ cargo install 
```

## 使い方

### ビルド方法

```term
docker-composed build
```

### 起動方法

```term
docker-composed up -d
```

### 終了方法

```term
docker-composed down
```

## 動作例

```
$ curl -X GET localhost:8088/pokemon
""

$ curl -X POST -H "Content-Type: application/json" -d '{"number":1, "name":"test_name", "types": [ "Fire" ]}' localhost:8088/pokemon
$ curl -X POST -H "Content-Type: application/json" -d '{"number":2, "name":"test_name2", "types": [ "Water", "Electric" ]}' localhost:8088/pokemon
$ curl -X GET localhost:8088/pokemon
[{"number":1,"name":"test_name","types":["Fire"]},{"number":2,"name":"test_name2","types":["Water","Electric"]}]

$ curl -X GET localhost:8088/pokemon/1
{"number":1,"name":"test_name","types":["Fire"]}

$ curl -X PUT -H "Content-Type: application/json" -d '{"number":1, "name":"test_name2", "types": [ "Water" ]}' localhost:8088/pokemon/1
$ curl -X GET localhost:8088/pokemon/1
{"number":1,"name":"test_name2","types":["Water"]}

$ curl -X DELETE localhost:8088/pokemon/1
$ curl -X GET localhost:8088/pokemon
[{"number":2,"name":"test_name2","types":["Water","Electric"]}]
```
