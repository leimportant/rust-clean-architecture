

src/
├── main.rs
├── app.rs                     # composition root / wiring
├── config/
│   └── settings.rs            # env, db driver
│
├── domain/
│   ├── user/
|              ├── entity.rs
|              ├── repository.rs
|              ├── service.rs
|              └── mod.rs
│   ├── catalog/
|              ├── entity.rs
|              ├── repository.rs
|              ├── service.rs
|              └── mod.rs
│              
│
├── application/
│   ├── user/
|   |_______register_user.rs
|   |_______login_user.rs
|   |_______mod.rs
│   ├── catalog/
|   |_______create_catalog.rs
|   |_______mod.rs
│   └── mod.rs
│
├── adapters/
│   ├── inbound/
│   │   └── http/
│   │       ├── user.rs
│   │       └── catalog.rs
│   │
│   └── outbound/
│       └── persistence/
│           ├── mod.rs
│           ├── user_repository.rs     # SINGLE ADAPTER
│           ├── catalog_repository.rs     # SINGLE ADAPTER
│           └── sql/
│               ├── mod.rs
│               ├── mysql.rs
│               └── postgres.rs
│
├── infrastructure/
│   └── database/
│       ├── mod.rs
│       ├── mysql.rs
│       └── postgres.rs
│
└── routes.rs


# Command
```bash
mkdir -p src/{config,domain/{user,catalog},application/{user,catalog},adapters/{inbound/http,outbound/persistence/sql},infrastructure/database} \
&& touch src/{main.rs,app.rs,routes.rs} \
&& touch src/config/settings.rs \
&& touch src/domain/mod.rs src/domain/user/{entity.rs,repository.rs,service.rs,mod.rs} src/domain/catalog/{entity.rs,repository.rs,service.rs,mod.rs} \
&& touch src/application/mod.rs src/application/user/{register_user.rs,login_user.rs,mod.rs} src/application/catalog/{create_catalog.rs,mod.rs} \
&& touch src/adapters/inbound/http/{user.rs,catalog.rs} \
&& touch src/adapters/outbound/persistence/{mod.rs,user_repository.rs,catalog_repository.rs} \
&& touch src/adapters/outbound/persistence/sql/{mod.rs,mysql.rs,postgres.rs} \
&& touch src/infrastructure/database/{mod.rs,mysql.rs,postgres.rs}
```