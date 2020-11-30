# Assyst - The dev-facilitating bot
Assyst's primary purpose is to provide ease-of-life features to developers.
It has a range of utilitarian commands, ranging from code execution to website screenshotting.

## Features
    - Powerful message edit and delete handling, so command messages can be edited and the command will be re-run.
    - Completely open-source. Anyone can contribute.
    - Simple to self-host, extend, and modify.
    - Powerful flag parsing and documentation of all flags.
    - Written in pure TypeScript, with as little dependency on libraries as possible.
    - Customisable prefix, with mention support.

[Full parser documentation](https://github.com/Jacherr/Assyst-TS/blob/master/PARSER_DOCS.md)

## Self-hosting
If you wish to self-host Assyst, you can follow these steps. Do keep in mind that this bot uses APIs for certain commands, such as fAPI for image manipulation and other various tasks like screenshotting web pages.

### Without docker
**1.) Clone this repository**
```sh
$ git clone https://github.com/AssystDev/Assyst
```

### With docker
**1.) Download Dockerfile, build image and create container**
```sh
# download latest dockerfile
$ wget https://raw.githubusercontent.com/Jacherr/Assyst-TS/master/Dockerfile
# build dockerfile and tag image `assyst`
$ sudo docker build . -t assyst
# create container
$ sudo docker run -i --name assyst -t assyst sh
# attach to container
$ sudo docker attach assyst
```
<hr/>
The following steps are the same for both dockerized assyst and without using docker.


**2.) Install all required dependencies**
```sh
# navigate to repository
$ cd Assyst-TS
# install all dependencies that are specified in package.json
$ npm i
```

**3.) Update configuration files**

When self-hosting a fork of Assyst, you must setup a config file, which includes access tokens for various APIs, database credentials and other settings. Fortunately, this repository provides a template which you can copy.
```sh
# create a copy of the template
$ cp privateConfig.example.json privateConfig.json
# open config file (you can obviously use any other editor)
$ vim privateConfig.json
```

**4.) Transpile source code into JS**
> Make sure you've installed `typescript` globally. Alternatively, you may transpile it by specifying a path to the typescript compiler binary, e.g. `./node_modules/typescript/bin/tsc`
```sh
$ tsc
```
Output can be found in the JS folder

**5.) Run the bot**
There are two ways to run the bot: using a shell script that starts the process in an infinite loop and watches the exit code (useful for the `stop` command and in case the bot exits due to an unhandled error), or by simply running the bot.
If you decide to go with the shell script, you may want to change the path of the source code
```sh
# Method 1: shell script
# make shell script executable
$ chmod +x run.sh
# run shell script
$ ./run.sh

# Method 2
node .
```
