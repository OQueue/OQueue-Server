-- Your SQL goes here

create table "users" (
    "id" uuid not null,
    "name" varchar(255) not null,
    "email" varchar(255) not null,
    "pwhash" char(60) not null,

    primary key ("id")
);

create table "queues" (
    "id" uuid not null,
    "name" varchar(255) not null,
    "description" text not null,
    "organizers_ids" uuid[] not null,
    "created_at" timestamp not null,
    "exists_before" timestamp not null,

    primary key ("id")
);

create table "queues_entries" (
    "queue_id" uuid not null,
    "user_id" uuid not null,
    "joined_at" timestamp not null,
    "added_order" int not null,
    "has_priority" bool not null,
    "order" int not null,

    primary key ("queue_id", "user_id"),

    constraint "fk_user"
        foreign key("user_id")
            references "users"("id")
            on delete cascade,

    constraint "fk_queue"
        foreign key("queue_id")
            references "queues"("id")
            on delete cascade
);