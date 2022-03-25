-- Dev Only
DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

-- Dev Only
CREATE USER app_user PASSWORD 'app_pwd_to_change';
CREATE DATABASE app_db owner app_user ENCODING = 'UTF-8';