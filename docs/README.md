# About

The Nevermore FMS is a fully-customizable and all-around agnostic FMS with the goals of being:
* **Extensible**: The Nevermore FMS spawns a [dbus](https://www.freedesktop.org/wiki/Software/dbus/) IPC daemon that allow Nevermore Plugins to communicate with the FMS and each other. These plugins have the ability to add components to the UI, listen for events from the field, implement new hardware, and much more.
* **Safe**: The Nevermore FMS employs overrides for all critical components that can be invoked by referees and administrators at anytime, bypassing any code written inside plugins. This provides a safety net that removes the mission critical aspects of designing a FRC game and can allow for custom game design to be far more accessible for FRC students.
* **Fully-Featured**: The Nevermore FMS is designed to be capable of handling all of the same parts as the Official FIRST FMS (when combined with plugins), while also making them more accessible to all students. This includes the networking components (VLAN and DHCP setup), hardware components (PLC's controlling lights and sensors), and UI components necessary for competition.

## Quick Disclaimer
**TLDR: Robots are dangerous, try not to kill nor injure anyone with this software, but if you do we take no responsibility :)**

I would assume that if you're reading this you have some familiarity with either FIRST and the FIRST Robotics Competition or general robotics. In both cases they teach the same thing: safety, safety, and more safety. Please, please, please be careful with this FMS, I can't tell you how many close call I've had from neglecting simple safety when interacting with robots being controlled by this FMS. All major releases of this FMS are both unit tested and manually tested, though misuse is still possible, please read the documentation (TODO) carefully to take proper precautions.

## Contributing
Want to help in developing the Nevermore FMS? Feel free to take on one of our outstanding issues, add your own features, or contact us for recommendations and help on things to work on.

## Licensing
This project is under the GPLv3 license. Essentially, this means that you can modify this code in any way you want, though if you distribute the code you must provide the source code to them. For more information, go [here](https://choosealicense.com/licenses/gpl-3.0/)
