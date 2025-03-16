# Language mappings for anticheat-test.com
### this is still a WIP so new strings will be added

# Contributing
Pull requests are welcome, please use the english.json file as a base and work off that

avoid using Google Translate and if you are unsure about translating something don't modify it

for texture ids I use https://minecraft-heads.com if you find a head matching the language you can also use that


I have added **two translation files** instead of one because **Minecraft versions 1.15.2 and below** have issues displaying Arabic text correctly.  

#### **Solution:**  
- For **Minecraft 1.15.2 and below**, the issue is fixed by **reversing Arabic characters**, and this translation is stored in `Arabic_1.8.8`.  
- For **Minecraft 1.16 and above**, Arabic text is displayed correctly, so the regular translation file `Arabic` is used.  

As a developer, you should **check the player's Minecraft version** and apply the correct translation:  
- If the version is **1.15.2 or lower**, use **`Arabic_1.8.8`**.  
- If the version is **1.16 or higher**, use **`Arabic`**.  
