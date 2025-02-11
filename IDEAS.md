# General idea

## Components

This project will consist of 2 components 
- The web server, that will be used to display the website
- The discord bot, that will manage the discord server and the users


## User Story

When arriving in champ select, a player can create a 'group', by going on the website,  
this group will represent its team in the game (each group will have it's own generated unique ID)  
This will give that user a link that they can share to the rest of its team in lobby chat.  

Accessing that link will redirect to a discord invite to the leaguecord discord server,  

In the discord server, players will only be able to see the voice and text channel associated with their group,  
and maybe a ticket system.  

Other users will be hidden  

At the end of a league game, all players associated with the game will be removed from the discord server  
channels associated with the group will also be deleted.  

Unless a ticket was filed, in that case the channels and the ticket author will stay in the server.  


## Data cleanup

Discord servers cannot hold an infinite amount of users, we need to kick users at the end of their games.  

As for channels, it's important to also remove them to not clutter the server too much (+ rgpd ?)  


## Tickets management

This will not be a priority, but a ticket system could be nice to potentially ban some disturbing players  


## Abuse

To limit the potential ddos / flood of the discord server each invites will only be effective for 30 minutes(less if possible)  
And be limitted to 5 uses  
NOTE: Can a single user use more than once an invite link ?  
    If so, we cannot trust the invite usage cap  

To limit the potential ddos / flood of the webserver, a group that has one or less members after 5 minutes will be deleted  


# Technical side

## Technologies

It's gonna be almost 100% rust, with webassembly for the front end.  

For the bot, i think im gonna use [serenity](https://docs.rs/serenity)  

For the webserver, [rocket](https://docs.rs/rocket) seems fine.   

As for the front end, I know Yew a bit, so i might go for that.  

