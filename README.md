# ShipWreck-Server

난파선 서버 입니다.

## 프로젝트 세팅

### 환경 설정

- mysql-client와 build-essential이 깔려있어야 합니다.

```
sudo apt install mysql-client build-essential
```

- DB 커넥터 역할을 하는 `diesel`의 `cli`도 깔려있어야 합니다. 기본 기능을 사용하지 않고 `mysql`만 사용하므로 다음과 같은 명령어를 입력합니다.

```
cargo install diesel_cli --no-default-features --features mysql
```

### 환경 변수 설정

- `.env`파일이 있어야 합니다.

```
DATABASE_URL=MYSQL_서버_설정
```
