# Global documentation

This document is about the Robot Framework and Renode, their purpose and how they work.

## Renode

Renode is an open source software development framework with commercial support from Antmicro that lets you develop, debug and test multi-node device systems reliably, scalably and effectively.
In this training we will use Renode to simulate the STM32 board.

To install renode :

For Windows:  (need admin rights)
- go to [https://github.com/renode/renode/releases](https://github.com/renode/renode/releases) and download the latest version of the .msi installer.

Renode has an graphic interface with a Log window and a window for the Renode terminal.
Once Renode is open, you have to create a machine with the command :

```sh
mach create "NameOfTheMachine"
```

After that you have to load the board you want to work with.
Renode possed predefined platform that you can find here :
[Boards supported](https://renode.readthedocs.io/en/latest/introduction/supported-boards.html)

To load a predefined platform you have to use the command :

```sh
machine LoadPlatformDescription @path/to/my/board.repl
```

You can also create your own file .repl using their board and add peripherals.
We created a .repl file to simulate the STM32 board we need in this training, you can find it in the RenodeConfig folder.

After you have load your board, you have to upload the software on it.
For that type the command :

```sh
sysbus LoadELF @path/to/my/project.elf
```

It supported .elf executable but also other format as .out.
Once you have done those steps, you can start youd machine with the command :

```sh
start
```

Now your board is running your programm and you can look at/trigger your peripherals by the sysbus.

More information here :
[Renode Documentation](https://renode.readthedocs.io/)
[Download Renode](https://github.com/renode/renode/releases/)

or you can open the renode_basic_tutorial.md wich contains more general informations about renode.

## Robot Framework

Robot Framework is a generic open source automation framework.
It can be used for test automation and robotic process automation (RPA).
In this training we will use Robot Framework to test our different solution along the exercises.

To install Robot Framework you will need python :

```sh
pip install robotframework
```

Robot Framework uses differents types of files.
First the mains files are .robot files.
They are used to define the settings, the keywords, some functions and the test cases.

Those .robot files can also use .ressources files.
They are used to define the settings (you can use python librairies for example)
and the variables (dictionnary, list or scalable variable).

You can see some examples of tests using .robot and .ressources files here :
[Robot Framework example](https://robotframework.org/?tab=0#getting-started)

The interesant thing with Robot Framework is that renode support those test.
They can be run with the following command in your terminal :

```sh
renode-test path/myTests.robot
```

Once your tests are done, robot framework will create files to get more informations about the results (.html, .txt ...) in your current directory.
You can find tests for the exercises in the folder RobotFrameworkTests.

More information here :
[Robot Framework Guides](https://docs.robotframework.org/docs)
[Robot Framework User Guide](https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html)
