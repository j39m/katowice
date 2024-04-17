#!/bin/bash

set -eu

DB_BASENAME="expenditures.db"

cp -iv ~/Documents/personal/"${DB_BASENAME}" ./
sqlite3 "./${DB_BASENAME}" \
    "pragma foreign_keys = on;
     create table currencies (id int primary key, description text);
     insert into currencies values(0, 'USD');
     insert into currencies values(1, 'JPY');
     create table types (id int primary key, description text);
     insert into types values(0, 'essential');
     insert into types values(1, 'personal');
     insert into types values(2, 'general informational');
     insert into types values(3, '2024 P&E Awards MiraclePtr spot bonus');
     create table expenditures (
        date text,
        description text,
        amount real,
        currency int,
        type int,
        primary key (date, description),
        foreign key (currency) references currencies(id),
        foreign key (type) references types(id)
     );
     insert into expenditures (date, description, amount, currency, type)
         select date, description, amount, 0, 0 from essexp;
     insert into expenditures (date, description, amount, currency, type)
        select date, description, amount, 0, 1 from perexp;
     insert into expenditures (date, description, amount, currency, type)
        select date, description, amount, 1, 0 from jp_essential;
     insert into expenditures (date, description, amount, currency, type)
        select date, description, amount, 1, 1 from jp_personal;
     drop table essexp;
     drop table perexp;
     drop table jp_essential;
     drop table jp_personal;
     "
