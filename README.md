# Nevermore FMS :robot:
**This project is still a WIP, not everything listed here has been implemented yet. If you want to help feel free to contact me via [email](mailto:macdonnell.chase@gmail.com) or via Discord at `McMackety#0001`**

The Nevermore FMS is a fully-customizable and all-around agnostic FMS with the goals of being:
* **Extensible**: The Nevermore FMS embeds a full V8 Javascript engine within itself to run "Nevermore Workers". These workers have the ability to add React components to the UI, listen for events from the field, send messages to it's UI components, implement new hardware, and much more.
* **Safe**: The Nevermore FMS employs overrides for all critical components that can be invoked by referees and administrators at anytime, bypassing any code written inside workers. This provides a safety net that removes the mission critical aspects of designing a FRC game and can allow for custom game design to be far more accessible for FRC students.
* **Fully-Featured**: The Nevermore FMS is designed to be capable of handling all of the same parts as the Official FIRST FMS, while also making them more accessible to all students. This includes the networking components (VLAN and DHCP setup), hardware components (PLC's controlling lights and sensors), and UI components necessary for competition.

## Quick Disclaimer
**TLDR: Robots are dangerous, try not to kill nor injure anyone with this software, but if you do I take no responsibility :)**
I would assume that if you're reading this you have some familiarity with either FIRST and the FIRST Robotics Competition or general robotics. In both cases they teach the same thing: safety, safety, and more safety. Please, please, please be careful with this FMS, I can't tell you how many close call I've had from neglecting simple safety when interacting with robots being controlled by this FMS. All major releases of this FMS are both unit tested and manually tested, though misuse is still possible, please read the documentation (TODO) carefully to take proper precautions.

## Building
If you want to use this in production, make sure you only use released builds from [the releases page](https://github.com/Nevermore-FMS/nevermore-fms/releases). Though if you want to test or help develop, you can build it through the following steps.
1. First ensure you have [Rust](https://www.rust-lang.org/) fully installed for your system. Follow the instructions on the [install page](https://www.rust-lang.org/tools/install) for more info.
2. Run the following commands:
```bash
# Clone the repository.
git clone https://github.com/Nevermore-FMS/nevermore-fms.git

# Move into the downloaded directory.
cd nevermore-fms

# Build a release build
cargo build --release

# Run the build (Linux/OSX)
./target/release/nevermore-fms

# Run the build (Windows)
target/release/nevermore-fms.exe
```

## Developing Workers
Want to create your own Nevermore Workers? Well the good news is it's as simple as a single command to get started:
```bash
npx create-nevermore-app
```
Yep, that's right, with one command you can create a full worker project scaffold, including all the utilities you could want for easy development.

Once you have configured the project it can easily be run on a local development instance of Nevermore with:
```bash
# Ensure you have a running instance of the Nevermore FMS with developer options enabled.
npm run develop
```

Your worker should then be deployed to the local Nevermore instance and begin running immediately. More information on workers can be found in our documentation (TODO).

## Contributing
Want to help in developing the Nevermore FMS? Feel free to take on one of our outstanding issues, add your own features, or contact me for recommendations and help on things to work on. :smiley:

## Licensing
You most likely can already see that this project is under the GPLV3 license. Essentially, this means that you can modify this code in any way you want, though if you distribute the code you must provide the source code to them. For 99.99% of you this should literally not matter, you unknowingly interact with many libraries licensed under GPLV3 all the time. Though, if you have a questions feel free to contact me at [macdonnell.chase@gmail.com](mailto:macdonnell.chase@gmail.com), I'm not a lawyer but can give you alternative licenses if necessary and appropriate.