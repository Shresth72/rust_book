docker-compose up --build

# Sea-Orm
cargo install sea-orm-cli
sea-orm-cli migrate init

postgresql://539srijansiddharth:A2G3SlPMBHLo@ep-patient-feather-a189osl1.ap-southeast-1.aws.neon.tech/neondb?sslmode=require

sea-orm-cli migrate -u postgresql://postgresdb:password@localhost:5432/rocket_db
sea-orm-cli generate entity -u postgresql://postgresdb:password@localhost:5432/rocket_db -o src/entity