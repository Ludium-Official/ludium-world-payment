## 1. production 환경 파일 설정하기 
현재 루트 디렉토리에 production 폴더를 생성한다. 
```bash
|-- docker-compose
|-- src
|-- tests
| ...
|-- production
    |-- account_keys
    |-- self_signed_certs
        |-- cert.pem
        |-- key.pem
    |-- config.toml
|-- .env.production
```

### `.env.production` 파일 설정하기
`.env.production`에는 [`.env.example`](./.env.example) 파일을 참고하여 환경 변수를 설정한다.
- `HOST`: 서버 호스트 주소
- `PORT`: 서버 포트
- POSTGRES & DB Setting


### account_keys 폴더 설정하기
`account_keys` 폴더에는 `json` 형식으로 가스비 대납 및 보상 지급을 위한 계정 키를 설정한다. 
- 계정 키 파일은 [near/pagoda-relayer-rs readme](https://github.com/near/pagoda-relayer-rs#:~:text=Multiple%20Key%20Generation%20%2D%20OPTIONAL%2C%20but%20recommended%20for%20high%20throughput%20to%20prevent%20nonce%20race%20conditions) 참고 

### self_signed_certs 폴더 설정하기 
`self_signed_certs` 폴더에는 TLS 설정을 위한 `cert.pem`과 `key.pem` 파일을 설정한다.

### `config.toml` 파일 설정하기
`config.toml` 파일은 `near/pagoda-relayer-rs`에서 사용하는 설정 파일과 동일하다. 여기서는 `account_keys` 폴더 경로를 설정하고, relayer 계정 id를 설정한다. 
- testnet으로 설정되어있는 [config.toml](./config.toml) 참고 

## 2. production 환경에서 실행하기
docker-compose를 이용하여 production 환경에서 실행한다. 
```bash
cd docker-compose && docker-compose -f ./production.yml up -d
```
- `production.yml` 파일은 [docker-compose](./docker-compose/) 참고