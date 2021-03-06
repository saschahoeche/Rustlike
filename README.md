# Short Design Document

Project Name: Rustlike

### Short Description
A dungeon crawler with procedurally generated levels, monsters of increasing difficulty,
and turn-based movement.

### Story
The hero's hometown is suffering from a plague of monsters. Welling up from the deep,
they seem unstoppable. Legend tells of the *Amulet of Yala* - Yet Another Lost Amulet -
that can be used to stem the tide. After a long night at the tavern, the hero promises
to save the day - and sets forth into the dungeon.

### Basic Game Loops
- Enter dungeon level.
- Explore, revealing the map.
- Encounter enemies whom the player fights or flees from.
- Find power-ups and use them to strengthen the player.
- Locate the exit to the level - repeat

### Minimum Viable Product (MVP)
1. Create a basic dungeon map ✅
2. Place the player and let them walk around ✅
3. Spawn monsters, draw them, and let the player kill them by walking into them 
4. Add health and a combat system that uses it 
5. Add healing potions 
6. Display a "Game Over" screen when the player dies 
7. Add the *Amulet of Yala* to the level and let the player win by reaching it 

### Stretch Goals
1. Add Fields-of-View 
2. Add more interesting dungeon designs 
3. Add some dungeon themes 
4. Add multiple layers to the dungeon, with the Amulet in the last one 
5. Add varied weapons to the game 
6. Move to a data-driven design for spawning enemies 
7. Consider some visual effects to make combat more visceral 
8. Consider keeping score
9. 
### How To
Check the binaries folder for your operating system and start the binary to have a
look at the current development stage.