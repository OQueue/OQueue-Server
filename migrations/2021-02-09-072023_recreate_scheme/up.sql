-- Your SQL goes here

drop table "queues_entries";
drop table "queues";
drop table "users";

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
    "organizer_id" uuid,
    "created_at" timestamp not null,
    "exists_before" timestamp not null,

    primary key ("id")
);

create table "queue_entries" (
    "queue_id" uuid not null,
    "user_id" uuid not null,
    "order" int not null,
    "has_priority" bool not null,
    "is_held" bool not null,
    "joined_at" timestamp not null,

    primary key ("queue_id", "user_id"),

    constraint "fk_user_id"
      foreign key("user_id")
          references "users"("id")
          on delete cascade,

    constraint "fk_queue_id"
      foreign key("queue_id")
          references "queues"("id")
          on delete cascade
);