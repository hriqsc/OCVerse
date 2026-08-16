mkdir -p dist/
mkdir -p dist/db
mkdir -p dist/images
mkdir -p dist/logs
mkdir -p dist/logs/backend
mkdir -p dist/logs/frontend
mkdir -p dist/backend/target/release
mkdir -p dist/frontend/dist

cp backend/target/release/backend dist/backend/target/release/backend
cp .env dist/
cp -r frontend/dist dist/frontend/dist

rm /dist/*/*:Zone.Identifier