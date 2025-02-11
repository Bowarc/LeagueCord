I don't know how to structure the project  

Do i make two exectutables (bot & webserver) ?  
but since the server will need to create (or ask to create) groups, it will need access to the bot's data  

So there is multiple solutions:  

1. Make a socket, a seccond thread in the bot exectuable that receive requests from the server and modifies the bot's data  
  that is stored in `Arc<RwLock<TypeMap>>` via a cloned ref of the data arc. See [serenity's context for more information](https://docs.rs/serenity/latest/serenity/client/struct.Context.html)  

  This is probably the best idea, but idk if two executables is necessary  

2. Make a single binary and share the data via `Arc<RwLock`s  

   This idea follows the first one, but only uses one executable, which can maybe? cause performance issues and  
   make the code more cluttered  

3. Save the data on the server, and make local routes for the bot to access data.  

   This solution is horrible, since it can lead to security and performance issues.  

-----

I think I'll go with the 2nd approach, making this using multiple processes would require a socket for comunication  
and it seems really overkill to me  

Due to how rocket and serenity take the control flow away from me,  
i made a dispatcher framework in serenity to be able to make my own control flow  

-----

My idea is:  
   Boot up a rocket thread with a channel in it's data storage.  

   boot up serenity with a dispatcher sending events to a sender i gave it.  

   when receiving a group creation request from the rocket webserver, we create a group, use a saved serenity context  
   to create the discord channels and the invite, send the invite back to the webserver where it can do it's things  
   (like create a route for that group and display appropriate informations)  


   IMPORTANT NOTE  
   In order to not flood that part of the program, we need to filter the events in the dispatcher (before sending them)  
   Thus only sending ?(idk?) latest serenity contexts ?  

----

Wouldn't it be better to just share some data across the serenity handlers and the webserver directly ?  

thus the server using the serenity context to (or just boot up a tokio task to) create channels in the discord ?

----

I tried making a structured workspace, but the bot and web server are too linked to truely use a lib system  
+ why tf would a implementation of a web server or a discord bot would be a lib, THEY CAN'T EVEN FUNCTION WITHOUT EACH OTHER  
Stop stressing about random sht.  

Thread1 (Bot)
	loop in serenity

	when user joins
		

	When a user leave a channel
		check if it's the channel is now empty, kick every player of that group.

	Moderation

	Tickets etc..

Thread2 (web server)

	loop in rocket

	when receiving a group creation request,
		use serenity context to create a channel

		create a group,

	when receiving a group join request,
		redirect to discord invite link
		(or error if invalid group)

	
	

