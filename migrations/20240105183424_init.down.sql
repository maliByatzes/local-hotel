-- Add down migration script here

-- Delete guest table

drop table if exists "guest"

-- Delete payment_status table

drop table if exists "payment_status"

-- Delete booking table

drop table if exists "booking"
