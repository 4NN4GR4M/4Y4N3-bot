# AYANE Bot

## Features Requred
User with the **"Admin"** role, when in the **#define-yourself** channel, type:
```
@AYANE generate-role-menu
- #Role1 :emoji1:
- #Role2 :emoji2:
- #Role3 :emoji3:
- #Role4 :emoji4:
```
Bot generates a menu message with the emoji + role.
MessageId is stored internally by the bot.
Whenever user reacts to that message with one of the emojis, if it's on the above message, they get assigned the corresponding role.