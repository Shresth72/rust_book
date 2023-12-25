docker-compose up --build

# Sea-Orm
cargo install sea-orm-cli
sea-orm-cli migrate init

sea-orm-cli migrate -u postgresql://postgresdb:password@localhost:5432/rocket_db
sea-orm-cli generate entity -u postgresql://postgresdb:password@localhost:5432/rocket_db -o src/entity