-- add the avatar background and avatar emoji columns to user table
alter table 
    users
add column avatar_background varchar(9) not null default '#F6B67A',
add column avatar_emoji varchar(20) not null default '0x1F355';
