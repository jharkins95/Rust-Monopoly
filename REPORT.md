Final Report: Rust-Monopoly
===========================

Summary
-------

I created a simple clone of the board game Monopoly for my CIS 198 final project. Most rules of the game are implemented, including:

* buying properties
* collecting rents
* passing GO
* jail (and rolling doubles/paying the fine to get out)
* landing on Chance and Community Chest spaces (and having associated actions with them, such as going to jail or collecting a prize)
* having a monopoly (all of a color group's properties), which increases rent charged to other players when they land on those spaces
* ability to buy and sell houses/hotels if a player has a monopoly, increasing rent further
* simple bankruptcy (in this game, the player loses as soon as their cash becomes negative; they are given no option to sell properties to pay off debt)

Some things I did not implement:

* trading properties (this would likely require making the game turn state much more complicated, and it's already quite complicated!)
* auctions (another layer of complexity due to timing of bids)
* networked play (since Monopoly is inherently complex, it would take time to come up with a well-designed protocol to handle changes in the game state)
* concurrency (the piston drawing thread crashed as soon as I tried to spawn it. I don't think it plays well with concurrency.)
* property mortgages
* Get Out of Jail Free Cards

Accomplishments
---------------

* Able to draw the board, player tokens, and houses/hotels on screen with the Piston OpenGL framework
     - Comment: Piston's drawing API is simple and doesn't allow you to put text easily on the screen, so I had to manually insert the text into the board image using GIMP.
* Mechanism to handle actions taken by the player on their turn (see the relevant state structs in game.rs)
     - Comment: This was probably the trickiest part of implementing the game. I was originally just having the players enter commands into the console, but this prevented the game from rendering properly.
* Able to move players around the board, collect rents, buy properties, and play the game (see Summary above).

Components
----------

* main.rs – contains the entry point to the game
* game.rs – contains all of the game state, including a finite state machine to handle player turns, as well as the game board and drawing window
* board.rs – contains state relevant to the board, including the players, properties, spaces, and card decks
* cards.rs – contains enum definitions for the Chance and Community Chest cards
* player.rs – contains information about a player, including their cash reserves, the properties they own, the space they are currently on, and whether or not they are in jail
* property.rs – contains information about a property, such as its color group, its owner, the number of houses and hotels it has, its base rent, and its purchase price

The game was designed to be as modular as possible. I tried to separate board state (board.rs) from player state (player.rs) and general game state (game.rs) as much as possible by limiting access in game.rs to only some of the board state, such as the current player (not all of the players).

Given Rust's lack of a notion of traditional inheritance (in contrast to Java or C++), I was forced to design the game differently than I would have done in those languages. For example, if I was working in Java, I probably would have made the Property struct a subtype of Space instead of creating a type enum in Space. This probably would have also simplified keeping track of the spaces and properties since I could have created a more generic container or list for Spaces, and then checked, for every space, if it happened to also be a Property.

Because Rust is very strict about its borrowing rules, I used Rc and RefCell liberally in my code. This led to a significant amount of code bloat and forced me to use some constructs that I would not have had to use in a different language. For example, if I wanted to get the type of an Rc<RefCell<Space>> without having to borrow the Rc outright (and possibly face a borrow panic at runtime later), I had to wrap the borrow in its own block. See handle_land_space in game.rs for an example of this.

Testing
-------

Testing of the game was done exclusively by playing the game once it was at a playable stage. Testing of simple functions (such as rolling the dice) was done by calling the function directly from main and observing the results.

Benchmarks
----------

I don't have much to say here, other than that the game is (mostly) playable and recognizable to someone already familiar with the game. The game is not graphics-intensive, and it seems to run well on my laptop running Ubuntu 15.10. I tried testing it on Windows, but the graphics libraries fail to compile properly, so I wasn't able to play it there. I have not tried the game on Mac or any other OS.

Limitations
-----------

The user interface was probably the most limiting factor in designing the game. Unlike Swing or Qt, Rust's graphics libraries are not very well-developed and don't offer much in terms of a user interface, such as buttons or text fields. This forced me to be a little creative in handling user input (the user has to click in the game window to enter commands, which is also inconvenient).

As mentioned previously, the game is only confirmed to work on Linux.

Portmortem
----------

I feel that the actual design of the game structs (Board, Player, etc.) went well and allowed me to think about the overall flow of the game state more clearly. If I had to design a game of this scale again, I would probably not do it in Rust until the language becomes more mature and more support is available for graphics.

It goes without saying that Rust 's borrowing system can be very picky at times. Although the borrowing system in Rust has its merits in terms of runtime safety, I feel that it was a significant limitation to the progress of designing my game. Not only did it make my structs and code more complicated - it forced me to step back and think carefully about how to structure my code such that there was only one mutable borrow of an object at any given time. For a game such as Monopoly, in which players have to keep track of their properties and properties have to track the players landed on them, the borrowing system can make code very complicated and verbose.
