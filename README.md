# Hello Godot

This is a demonstration project for using rust with godot. There is a simple hello_world, as well as an ecs file that contains a more elaborate example with bevy.

<br>

## Getting Started

You'll want to start by compiling the rust code found in the logic directory. Once complete, either copy the binary (.dll, .so, etc) to the godot directory, or create a logical link to it. From there, you should be able to open the project from the Godot engine. Make sure the *logic.gdnlib* resource references the rust binary for your environment and hit the play button (or F5).