<div align="center">

# my-blaster-runs-hot
---
</div>

A twin stick shooter prototype inspired by the "My Blaster Runs Hot" mini-game from "Ratchet and Clank A Crack in Time". The goal is recreate all the functionality from the mini-game. The current color code is blue = player, red = enemy and green = civillian.

## Features 
---

### Animations

Using these assets from <https://secrethideout.itch.io/team-wars-platformer-battle> with slight modifications different animations will be run depending on the state of the entity. For example running, idle or death. Look at this animation sheet I use for the player. 

<div align="center">

![image](https://github.com/ssnover/my-blaster-runs-hot/blob/main/assets/darians-assets/TeamGunner/CHARACTER_SPRITES/Blue/Blue_Soldier_50.png)

</div>

It does not have uniform dimensions for every animation array, I wrote my own trait that when implemented describes the sprite sheet and animation arrays, to my animation plugin. Then that plugin will take the current state of the entity, with the sprite info and display the correct animation sequence.

### Collisions 

There is a nice physics engine that works well with Bevy, called Rapier that handles the definitions of the physics and has the ability to add custom groupings to determine what collisions the developer wants to track. You could see the player projectile go through the civilian as in the inspiration game ("My Blaster Runs Hot") the player could not kill the civilians. Also the enemy collisions with the player cause the player damage compared to the civilian collisions which despawn the civilian and add to the score.

### UI

There is start menu, main game loop, and end game screen. The entity number and type is determined by a simple text file that defines the rounds, then those entities are spawned and the current Round is displayed. A round is over when all entities are despawned. Then the next round starts. Meanwhile the player lives are displayed, after the player takes enough damage a life is lost. After all rounds are done or the player loses all lives then end game screen is displayed showing the score.

## Demo
---

Here are those features I just talked about in action

![Demo](https://github.com/ssnover/my-blaster-runs-hot/blob/main/demo/my-blaster-runs-hot.gif)

