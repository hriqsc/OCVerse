rm -r dist/backend
rm -r dist/frontend
rm -r dist/logs
rm -r dist/

mkdir -p dist/
mkdir -p dist/data/db
mkdir -p dist/data/images
mkdir -p dist/logs/backend
mkdir -p dist/logs/frontend
mkdir -p dist/backend/target/release
mkdir -p dist/frontend/dist


cp -f backend/target/release/backend dist/backend/target/release/backend
cp -r frontend/dist dist/frontend/dist
cp -f nginx.conf.template dist/
cp -f compose.yml dist/

rm /dist/*/*:Zone.Identifier

if [ ! -f dist/.env ]; then
    cp -f .env.template dist/.env
fi