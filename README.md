# [Merging for JoJoAPI](https://github.com/KojoBailey/japi-merging-rs)
This is a plugin for [JoJoAPI](https://jojomodding.miraheze.org/wiki/JoJoAPI) that allows for the **merging** of data in *[JoJo's Bizarre Adventure: All-Star Battle R](https://jojomodding.miraheze.org/wiki/All-Star_Battle_R)*. 

Or at least, it will once it's up and running!

This plugin was coded in Rust with the help of my [JoJoAPI Rust wrapper](https://github.com/KojoBailey/japi-wrapper-rs). Originally, this project was in [C++](https://github.com/KojoBailey/JAPI-XFBIN-Merger), but I've switched to Rust as it better suits my wants and needs.
<!--
## How do I merge?
Right now, you can only merge some nuccChunkBinary formats. These are:
- [messageInfo](https://jojomodding.miraheze.org/wiki/User:KojoBailey/NUCC_JSON#messageInfo)
- [PlayerColorParam](https://jojomodding.miraheze.org/wiki/User:KojoBailey/NUCC_JSON#PlayerColorParam)
- [SpeakingLineParam](https://jojomodding.miraheze.org/wiki/User:KojoBailey/NUCC_JSON#SpeakingLineParam)
- [MainModeParam](https://jojomodding.miraheze.org/wiki/User:KojoBailey/NUCC_JSON#MainModeParam)

Many more are to come, so sit tight!

Creating merge files is quite simple, as you just need to write the data you want to target in a JSON file. See the [Formats](#Formats) section for how to structure those correctly. You will also see a **Directory** under each section, indicating where those JSON files should go, like so:

![image](https://github.com/user-attachments/assets/00d952f8-2ade-483b-845d-b25019c071c8)

`_priority.json` simply controls which files get loaded in which order, and will update itself to add/remove new/old JSON files whenever you launch the game (and this activate the plugin). A **higher** priority number means that file takes **higher** precedence over others lower than it, since it will get loaded later and therefore overwrite the data that came before it.
-->
