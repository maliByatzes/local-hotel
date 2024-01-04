-- Add migration script here
create table payment_status (
  guest_id serial primary key,
  payment_status_name varchar(50) not null
);

create table booking (
  booking_id serial primary key,
  guest_id int not null,
  payment_status_id int not null,
  checkin_date timestamp not null,
  checkout_date timestamp not null,
  num_adults int not null,
  num_children int not null,
  booking_amount numeric(10,2) not null,
  foreign key (guest_id) references payment_status (guest_id)
);
