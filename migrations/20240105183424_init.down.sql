-- Add down migration script here

-- Delete guest table

drop table if exists "guest" cascade;

-- Delete payment_status table

drop table if exists "payment_status" cascade;

-- Delete booking table

drop table if exists "booking" cascade;
