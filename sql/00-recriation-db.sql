DROP DATABASE IF EXISTS todomvc;
DROP USER IF EXISTS todomvc_user;

CREATE USER todomvc_user PASSWORD 'postgres';
CREATE DATABASE todomvc OWNER admin ENCODING = 'UTF-8';
