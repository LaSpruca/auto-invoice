-- Users table

create table users
(
    id             varchar not null
        constraint users_pk
            primary key,
    street_address varchar,
    suburb         varchar,
    city           varchar,
    postcode       integer,
    bank_account   integer,
    phone_number   integer,
    email          varchar
);

create unique index users_id_uindex
    on users (id);

-- Clients table

create table clients
(
    id             integer not null
        constraint clients_pk
            primary key autoincrement,
    owning_user    varchar not null
        constraint owning_user
            references users,
    name           varchar not null,
    company_name   varchar,
    street_address varchar,
    suburb         varchar,
    city           varchar,
    postcode       varchar
);

create unique index clients_id_uindex
    on clients (id);

-- Templates table

create table templates
(
    id              integer not null
        constraint templates_pk
            primary key autoincrement,
    user            varchar not null
        constraint owning_user
            references users,
    template_header text    not null,
    template_itens  text    not null,
    template_footer text    not null,
    styles          text    not null
);

create unique index templates_id_uindex
    on templates (id);

-- Invoices table

create table invoices
(
    id          integer not null
        constraint invoices_pk
            primary key autoincrement,
    user        varchar not null
        constraint owning_user
            references users,
    client      integer not null
        constraint client
            references clients,
    adjustments decimal default 0 not null,
    notes       text
);

create unique index invoices_id_uindex
    on invoices (id);

-- Invoice items table

create table invoice_items
(
    id          integer not null
        constraint invoice_items_pk
            primary key autoincrement,
    invoice     integer not null
        constraint invoice
            references invoices (id),
    description varchar not null,
    quantity    decimal default 0.0 not null,
    unit_price  decimal default 0.0 not null
);

create unique index invoice_items_id_uindex
    on invoice_items (id);
