# Web API サーバー on Rust サンプル

- Rust で実装した Web API サーバーのサンプル。

## 必要要件

- Docker
- docker-compose
- Rust
- diesel

```term
$ cd server
$ cargo install 
```

## 使い方

### ビルド方法

```term
docker-compose build
```

### 起動方法

```term
docker-compose up -d
```

### 終了方法

```term
docker-compose down
```

## 動作例

```
$ curl -X GET localhost:8080/pokemon
{"message":"FAILURE Get Pokemon List","type":"get_pokemon_list_error"}

$ curl -X POST -H "Content-Type: application/json" -d '{"number":1, "name":"test_name", "types": [ "Fire" ]}' localhost:8080/pokemon
SUCCESS Register Pokemon
$ curl -X POST -H "Content-Type: application/json" -d '{"number":2, "name":"test_name2", "types": [ "Water", "Electric" ]}' localhost:8080/pokemon
SUCCESS Register Pokemon
$ curl -X GET localhost:8080/pokemon
[{"number":1,"name":"test_name","types":["Fire"]},{"number":2,"name":"test_name2","types":["Water","Electric"]}]

$ curl -X GET localhost:8080/pokemon/1
{"number":1,"name":"test_name","types":["Fire"]}

$ curl -X PUT -H "Content-Type: application/json" -d '{"number":1, "name":"test_name2", "types": [ "Water" ]}' localhost:8080/pokemon/1
SUCCESS Update Pokemon: no 1
$ curl -X GET localhost:8080/pokemon/1
{"number":1,"name":"test_name2","types":["Water"]}

$ curl -X DELETE localhost:8080/pokemon/1
SUCCESS Delete Pokemon: no 1
$ curl -X GET localhost:8080/pokemon
[{"number":2,"name":"test_name2","types":["Water","Electric"]}]
```
