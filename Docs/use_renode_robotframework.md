# Global documentation

This document is about the Robot Framework and Renode, their purpose and how they work.

## Renode

Renode is an open-source software development framework with commercial support from Antmicro that lets you develop, debug and test multi-node device systems reliably, scalably and effectively.
In this training we will use Renode to simulate the STM32 board.

### Install renode for Windows

- go to [https://github.com/renode/renode/releases](https://github.com/renode/renode/releases) and download the latest version of the .msi installer.

### Install renode for Linux

First of all go to [https://github.com/renode/renode/releases/tag/v1.13.3](https://github.com/renode/renode/releases) and download the latest version of the Linux portable file .tar.gz !

As of the time of writing this tutorial, the current version of renode is Renode 1.13.3.

#### Using the Linux portable release

After you download the file, unpack it using :

``` bash
mkdir renode_portable
tar xf  renode-*.linux-portable.tar.gz -C renode_portable --strip-components=1
```

#### Installing dependencies

##### Add the Mono repository to your system

Go to [https://www.mono-project.com/download/stable/#download-lin](https://www.mono-project.com/download/stable/#download-lin) and download it using the commands based on the linux distribution that you have (Ubuntu, Debian, Raspbian...)

##### Other dependencies (Ubuntu 20.04)

On Ubuntu 20.04, you can install the remaining dependencies with the following command:

``` bash
sudo apt-get install policykit-1 libgtk2.0-0 screen uml-utilities gtk-sharp2 libc6-dev gcc python3 python3-pip
```

If you are running a different distribution than Ubuntu, you will need to install an analogous list of packages using your package manager; note that the package names may differ slightly.

##### Additional prerequisites (for Robot framework testing)

To write and run test cases, Renode integrates with the Robot testing framework. This requires you to install Python 3 with pip (note that the relevant package may be called python-pip or python3-pip on Linux).

Once you have Python 3 and pip, install some additional modules :

``` bash
python3 -m pip install -r tests/requirements.txt
```

#### Running Renode

If you followed the instructions on installing from a package above, and if you built it from source, navigate to the relevant directory and use :

``` bash
./renode 
```

To use it from any location, enter the created directory and add it to the system path:

```bash
cd renode_portable
export PATH="`pwd`:$PATH"
```

### How to use Renode

Renode has a graphic interface with a Log window and a window for the Renode terminal.
Once Renode is open, you must create a machine with the command:

```sh
mach create "NameOfTheMachine"
```

After that you must load the board, you want to work with.
Renode possessed predefined platform that you can find here:
[Boards supported](https://renode.readthedocs.io/en/latest/introduction/supported-boards.html)

To load a predefined platform, you must use the command:

```sh
machine LoadPlatformDescription @path/to/my/board.repl
```

You can also create your own file .repl using their board and add peripherals.
We created a .repl file to simulate the STM32 board we need in this training, you can find it in the RenodeConfig folder.

After you have loaded your board, you must upload the software on it.
For that, type the following command:

```sh
sysbus LoadELF @path/to/my/project.elf
```

It supported .elf executable but also other format as .out.
Once you have done those steps, you can start your machine with the command:

```sh
start
```

Now your board is running your program and you can look at/trigger your peripherals by the sysbus.
For example to check the led's state:

```sh
sysbus.gpioPortA.LD2 State
```

Or to press the user button :

```sh
sysbus.gpioPortC.UserButton Press
```

More information here:
[Renode Documentation](https://renode.readthedocs.io/)
[Download Renode](https://github.com/renode/renode/releases/)

or you can open the renode_basic_tutorial.md which contains more general information about renode.

## Robot Framework

Robot Framework is a generic open-source automation framework.
It can be used for test automation and robotic process automation (RPA).
In this training we will use Robot Framework to test our different solution along the exercises.

To install Robot Framework, you will need python:

```sh
pip install robotframework
```

Robot Framework uses different types of files.
First the mains files are .robot files.
They are used to define the settings, the keywords, some functions, and the test cases.

Those .robot files can also use .resources files.
They are used to define the settings (you can use python libraries for example)
and the variables (dictionary, list, or scalable variable).

You can see some examples of tests using .robot and .resources files here :
[Robot Framework example](https://robotframework.org/?tab=0#getting-started)

The interesting thing with Robot Framework is that renode support those tests.
They can be run with the following command in your terminal:

```sh
renode-test path/myTests.robot
```

Once your tests are done, robot framework will create files to get more information about the results (.html, .txt ...) in your current directory.
You can find tests for the exercises in the folder RobotFrameworkTests.

More information here:
[Robot Framework Guides](https://docs.robotframework.org/docs)
[Robot Framework User Guide](https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html)
