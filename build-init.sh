docker run --name mydb -e MYSQL_ROOT_PASSWORD=root1234 -p 3306:3306 -d mariadb
docker exec -it mydb sh -c "apt-get update && apt-get install mysql-client -y"
docker cp build-init.sql mydb:/tmp/script.sql
docker exec -it mydb mysql --user=root --password=root1234 < /tmp/script.sql
