alter table users alter avatar_emoji set default '🍕';

update
    users
set
    avatar_emoji = '🍕';
