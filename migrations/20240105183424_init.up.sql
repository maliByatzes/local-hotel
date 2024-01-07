-- Add up migration script here

-- Create guest table

create table if not exists "guest" (
  id serial primary key not null,
  first_name varchar(100) not null,
  last_name varchar(100) not null,
  email_address varchar(100) not null unique,
  password varchar(100) not null,
  verified boolean not null default false,
  phone_number varchar(20) not null unique,
  created_at timestamptz default now(),
  updated_at timestamptz default now()
);

-- Create payment_status table

create table if not exists "payment_status" (
  id serial primary key not null,
  payment_status_name varchar(10),
  created_at timestamptz default now(),
  updated_at timestamptz default now()
);

-- Create booking table

create table if not exists "booking" (
  id serial primary key not null,
  guest_id int not null,
  payment_status_id int not null,
  checkin_date date not null,
  checkout_date date not null,
  num_adults int not null,
  num_children int not null,
  booking_amount numeric(10,2) not null,
  created_at timestamptz default now(),
  updated_at timestamptz default now(),
  foreign key (guest_id) references guest (id),
  foreign key (payment_status_id) references payment_status (id)
);
