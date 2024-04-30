*** INSTALL DOCKER ***

    a. On windows, don't install it on ProgramFiles folder

    b. Download it, and then inside that download folder, run powershell with admin right -> run: Start-Process "Docker Desktop Installer.exe" -Verb RunAs -Wait -ArgumentList "install --installation-dir=C:\Docker\"

* Install postgres on docker: docker pull postgres:12-alpine <light weight version>

    a. Read about postgres environment variable in Docker: https://hub.docker.com/_/postgres

    b. run docker container from pulled image: docker run --name postgres12 -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:12-alpine

* Access Docker console to use postgres db

    a. docker exec -it postgres12 psql -U root

*** Table plus ***

* connection to postgresSQL:

    1. name: postgres12
    
    2. host: localhost - port 5432

    3. user: root - pw:secret - database: root

*** Download linker cc (wsl2 - ubuntu 22.04) ***

1. sudo apt-get update

2. sudo apt install build-essential


*** Postgres(Docker) ***

postgres:
	docker run --name postgres12 -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:12-alpine

createdb:
	docker exec -it postgres12 createdb --username=root --owner=root workspaces

dropdb:
	docker exec -it postgres12 dropdb workspaces
