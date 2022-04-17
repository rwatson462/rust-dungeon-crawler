# rust-dungeon-crawler
A Roguelike dungeon crawler game featuring procedurally generated levels and bad guys to kill

## Project name
_The Rusty Dungeon_

## Basic game loops
1. Enter dungeon level
2. Explore the entire level
3. Encounter enemies, kill them or run away
4. Find loot and get stronger
5. Find the key to the exit door
6. Exit the dungeon

## Initial release
The game is considered "ready for release" when these goals have been achieved:
 1. Create procedurally generated map
 2. Create the Player and enable movement
    * 4 axis of movement, no diagonal moves supported
    * Add collision with walls
 3. Allow exiting the level and starting new levels
 4. Add Monsters that roam randomly, killing the player if they touch
 5. Add combat:
    * Add Player/Monster health so player can take a few hits before death
    * Add Player fighting abilities so Monsters can be defeated
 6. Add Death screen and game restart mechanism
 7. Add Basic loot system, starting with Health potion
 8. Add the dungeon Key to the level, required to be picked up before exiting a level
 9. Add Treasure to loot system
10. Track player score based on:
    * Number of Monsters killed
    * Number of Dugeons cleared
    * Amount of Treasure picked up from loot crates

## Stretch goals
Once the game is complete, these goals may or may not be added later
 1. Restrict player vision to a specific radius, same with Monsters
 2. Add bigger dungeons every X levels
 3. Create different weapons for the player to use
    * Spear with some range
    * Hammer that does more damage but swings less often
    * Dagger that does less damage but swings more often
 4. Add powerups to the Loot crates
    * Extra damage for a period of time
    * Extra speed
    * Less damage received
 5. Add visual effects to make the game look better
 